#include "emulator/Cpu.h"
#include "emulator/MemoryBus.h"
#include "emulator/BiosRom.h"
#include "emulator/Ram.h"
#include "emulator/UnmappedMemoryDevice.h"
#include "emulator/Via.h"
#include "engine/EngineUtil.h"
#include "engine/Paths.h"
#include <vector>
#include <string>
#include <cstdio>
#include <optional>
#include "core/ErrorHandler.h"

// Simple standalone tool: run CPU and collect BIOS call stack (JSR/BSR ... RTS) frames.
// Build target will be added via CMake.

struct Frame { uint16_t target; uint16_t return_addr; };

static bool IsBiosAddr(uint16_t addr) { return addr >= 0xF000; }

// Detect JSR opcodes (prefixes for extended forms) and BSR.
static bool IsCallOpcode(uint8_t op) {
    switch(op) {
        case 0x8D: // BSR
        case 0x9D: // JSR direct
        case 0xAD: // JSR indexed
        case 0xBD: // JSR extended
            return true;
        default: return false;
    }
}

int main(int argc, char** argv) {
    bool rootOk = EngineUtil::FindAndSetRootPath(fs::path(fs::absolute(argv[0])));
    std::string biosPath;
    if (argc > 1) {
        biosPath = argv[1];
        if(!rootOk) {
            // We rely on explicit path; warn but continue.
            std::fprintf(stderr, "[warn] Root path auto-detect failed; using explicit BIOS: %s\n", biosPath.c_str());
        }
    } else {
        if(!rootOk) {
            std::fprintf(stderr, "[error] Could not locate project root (System.bin not found upward) and no BIOS path provided.\n");
            std::fprintf(stderr, "        Usage: %s <path-to-bios.bin>\n", argv[0]);
            return 1;
        }
        biosPath = Paths::biosRomFile.string();
    }

    MemoryBus bus;

    BiosRom bios; bios.Init(bus); if(!bios.LoadBiosRom(biosPath.c_str())) { std::fprintf(stderr, "Failed to load BIOS %s\n", biosPath.c_str()); return 2; }
    Ram ram; ram.Init(bus);
    Via via; via.Init(bus); via.Reset();
    UnmappedMemoryDevice unmapped; unmapped.Init(bus);

    Cpu cpu; cpu.Init(bus); cpu.Reset();

    std::vector<Frame> stack;
    stack.reserve(32);

    bool hadAny = false;
    size_t instrCount = 0;
    const size_t instrLimit = 5000; // reduced for debug
    const size_t traceFirst = 64;   // print first N instructions
    const size_t progressEvery = 1000;

    std::printf("[trace] Reset PC=$%04X (BIOS file: %s)\n", cpu.Registers().PC, biosPath.c_str());

    // We'll fetch opcodes by peeking bus before execution. Simpler: inside loop read bus.ReadRaw(PC)

    // Suppress undefined write spam / failures while tracing
    ErrorHandler::SetPolicy(ErrorHandler::Policy::LogOnce);

    while(instrCount < instrLimit) {
        uint16_t pc = cpu.Registers().PC;
        uint8_t opcode = bus.ReadRaw(pc);
        if(instrCount < traceFirst) {
            std::printf("[i=%04zu] PC=$%04X OPC=%02X S=$%04X A=%02X B=%02X DP=%02X\n", instrCount, pc, opcode, cpu.Registers().S, cpu.Registers().A, cpu.Registers().B, cpu.Registers().DP);
        } else if(instrCount > 0 && (instrCount % progressEvery)==0) {
            std::printf("[progress] i=%zu PC=$%04X stackDepth=%zu\n", instrCount, pc, stack.size());
        }
        bool wasIndexedJsr = (opcode == 0xAD);
    uint16_t preS = cpu.Registers().S; // hardware stack pointer before exec
        uint16_t prePC = pc;

        if(IsCallOpcode(opcode) && opcode != 0xAD) {
            uint16_t returnAddr = 0; uint16_t target = 0; bool push = false;
            switch(opcode) {
                case 0x8D: { // BSR rel8 signed
                    int8_t off = (int8_t)bus.ReadRaw(pc+1);
                    returnAddr = pc + 2;
                    target = static_cast<uint16_t>(returnAddr + off);
                    push = true; break; }
                case 0x9D: { // JSR direct
                    uint8_t zp = bus.ReadRaw(pc+1);
                    returnAddr = pc + 2;
                    uint8_t dp = cpu.Registers().DP;
                    target = (uint16_t(dp) << 8) | zp; push = true; break; }
                case 0xBD: { // extended absolute 16
                    uint8_t hi = bus.ReadRaw(pc+1); uint8_t lo = bus.ReadRaw(pc+2);
                    returnAddr = pc + 3; target = (uint16_t(hi) << 8) | lo; push = true; break; }
            }
            if(push && IsBiosAddr(target)) {
                stack.push_back(Frame{target, returnAddr});
                hadAny = true;
            }
        }

    // Execute instruction
    cpu.ExecuteInstruction(false,false);
        ++instrCount;

        if(wasIndexedJsr) {
            // Detect if a JSR indexed actually happened by seeing S decreased by 2 (pushed return address)
            if(cpu.Registers().S == static_cast<uint16_t>(preS - 2)) {
                // The return address was the PC after opcode + postbyte (2 bytes consumed minimum)
                // We can't easily know the exact length of complex indexed pre-increment etc; but opcode 0xAD consumes 2 bytes before branching.
                // Return address is prePC + 2.
                uint16_t returnAddr = static_cast<uint16_t>(prePC + 2);
                uint16_t target = cpu.Registers().PC; // After JSR PC == subroutine entry
                if(IsBiosAddr(target)) {
                    stack.push_back(Frame{target, returnAddr});
                    hadAny = true;
                }
            }
        }

        // Handle RTS (0x39)
        if(opcode == 0x39) {
            if(!stack.empty()) {
                if(cpu.Registers().PC == stack.back().return_addr) {
                    stack.pop_back();
                }
            }
            if(hadAny && stack.empty()) break;
        }
    }

    // Output
    std::printf("BIOS call stack frames captured (%zu total frames still on stack) after %zu instructions:\n", stack.size(), instrCount);
    for(size_t i=0;i<stack.size();++i) {
        std::printf("[%02zu] target=$%04X return=$%04X\n", i, stack[i].target, stack[i].return_addr);
    }

    if(stack.empty()) {
        std::puts("(stack empty — all BIOS calls returned)");
    }

    return 0;
}
