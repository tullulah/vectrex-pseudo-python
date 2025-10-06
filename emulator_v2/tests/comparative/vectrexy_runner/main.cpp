#include <iostream>
#include <fstream>
#include <iomanip>
#include <vector>
#include <string>
#ifdef _WIN32
#include <windows.h>  // Para __try/__except y EXCEPTION_EXECUTE_HANDLER
#endif
#include "disable_asserts.h"  // DEBE estar ANTES de headers de Vectrexy
#include "emulator/Emulator.h"
#include "emulator/EngineTypes.h"  // Para Input, RenderContext, AudioContext
#include "core/ErrorHandler.h"
#include "nlohmann/json.hpp"

using json = nlohmann::json;

// Test direct Via object access (separada para usar __try/__except sin conflicto con try/catch de C++)
bool testViaDirectAccess(Emulator& emulator) {
    __try {
        auto& via = emulator.GetVia();
        std::cerr << "[DEBUG] Got Via reference successfully\n";
        
        std::cerr << "[DEBUG] Testing Via::IrqEnabled()...\n";
        bool irq = via.IrqEnabled();
        std::cerr << "[DEBUG] Via::IrqEnabled() = " << irq << " SUCCESS\n";
        
        std::cerr << "[DEBUG] Testing Via::FirqEnabled()...\n";
        bool firq = via.FirqEnabled();
        std::cerr << "[DEBUG] Via::FirqEnabled() = " << firq << " SUCCESS\n";
        
        return true;
    } __except(EXCEPTION_EXECUTE_HANDLER) {
        std::cerr << "[ERROR] SEH Exception accessing Via object!\n";
        return false;
    }
}

// C++ Original: Vectrexy no tiene este problema - puede leer VIA sin crashes
// Intentamos aislar qué registro específico causa problemas
int safeReadViaRegister(Emulator& emulator, uint16_t address, const char* regName) {
    __try {
        auto& bus = emulator.GetMemoryBus();
        uint8_t value = bus.Read(address);
        std::cerr << "[DEBUG] " << regName << " (0x" << std::hex << address << std::dec 
                  << ") = 0x" << std::hex << static_cast<int>(value) << std::dec << " SUCCESS\n";
        return value;
    } __except(EXCEPTION_EXECUTE_HANDLER) {
        std::cerr << "[ERROR] SEH Exception reading " << regName << " at 0x" 
                  << std::hex << address << std::dec << "!\n";
        return -1;
    }
}

json serializeCpu(Emulator& emulator) {
    const auto& regs = emulator.GetCpu().Registers();
    return {
        {"pc", regs.PC},
        {"a", regs.A},
        {"b", regs.B},
        {"x", regs.X},
        {"y", regs.Y},
        {"u", regs.U},
        {"s", regs.S},
        {"dp", regs.DP},
        {"cc", {
            {"c", static_cast<bool>(regs.CC.Carry)},
            {"v", static_cast<bool>(regs.CC.Overflow)},
            {"z", static_cast<bool>(regs.CC.Zero)},
            {"n", static_cast<bool>(regs.CC.Negative)},
            {"i", static_cast<bool>(regs.CC.InterruptMask)},
            {"h", static_cast<bool>(regs.CC.HalfCarry)},
            {"f", static_cast<bool>(regs.CC.FastInterruptMask)},
            {"e", static_cast<bool>(regs.CC.Entire)}
        }}
    };
}

json serializeVia(Emulator& emulator) {
    auto& bus = emulator.GetMemoryBus();
    uint8_t ifr = bus.Read(0xD00D);
    uint8_t ier = bus.Read(0xD00E);
    uint8_t t1_low = bus.Read(0xD004);
    uint8_t t1_high = bus.Read(0xD005);
    uint8_t t2_low = bus.Read(0xD008);
    uint16_t timer1 = t1_low | (static_cast<uint16_t>(t1_high) << 8);
    uint16_t timer2 = t2_low;
    return {
        {"ifr", ifr},
        {"ier", ier},
        {"timer1_counter", timer1},
        {"timer2_counter", timer2}
    };
}

int main(int argc, char* argv[]) {
    try {
        if (argc < 3) {
            std::cerr << "Usage: " << argv[0] << " <test.bin> <cycles> [--bios-only] [--trace]\n";
            std::cerr << "Example: " << argv[0] << " test.bin 1000\n";
            std::cerr << "         " << argv[0] << " test.bin 2500000 --bios-only  (execute BIOS from reset vector)\n";
            std::cerr << "         " << argv[0] << " test.bin 1000 --trace  (log every instruction)\n";
            return 1;
        }
        
        // Check for flags
        bool biosOnlyMode = false;
        bool traceMode = false;
        
        for (int i = 3; i < argc; ++i) {
            std::string arg(argv[i]);
            if (arg == "--bios-only") biosOnlyMode = true;
            if (arg == "--trace") traceMode = true;
        }
        
        std::cerr << "[DEBUG] Initializing emulator...\n";
        ErrorHandler::SetPolicy(ErrorHandler::Policy::Ignore);
        Emulator emulator;
        const char* biosPath = "C:/Users/DanielFerrerGuerrero/source/repos/pseudo-python/ide/frontend/dist/bios.bin";
        emulator.Init(biosPath);
        emulator.Reset();  // Reset after init to clear any state
        std::cerr << "[DEBUG] Emulator initialized and reset\n";
        
        // Verify PC after Reset() - should be 0xF000 from reset vector
        std::cerr << "[DEBUG] PC after Reset(): 0x" << std::hex 
                  << emulator.GetCpu().Registers().PC << std::dec << "\n";
        
        // CRITICAL: Crear contextos para Via::SetSyncContext
        // Esto es necesario para que Via::Sync() no crashee al desreferenciar punteros nulos
        std::cerr << "[DEBUG] Setting up Via SyncContext...\n";
        Input input;  // Default input state (no buttons pressed)
        RenderContext renderContext;  // Empty render context
        constexpr float CPU_FREQ = 1500000.0f;  // 1.5 MHz
        constexpr float AUDIO_SAMPLE_RATE = 44100.0f;
        AudioContext audioContext(CPU_FREQ / AUDIO_SAMPLE_RATE);
        
        emulator.GetVia().SetSyncContext(input, renderContext, audioContext);
        std::cerr << "[DEBUG] Via SyncContext configured\n";
        
        if (biosOnlyMode) {
            // BIOS-only mode: Do NOT load test.bin, do NOT override PC
            // PC is already set to 0xF000 by Reset() (BIOS reset vector)
            std::cerr << "[DEBUG] BIOS-only mode: Executing from RESET vector (PC=0xF000)\n";
        } else {
            // Normal mode: Load test code to RAM and set PC to 0xC800
            std::cerr << "[DEBUG] Loading test file: " << argv[1] << "\n";
            std::ifstream file(argv[1], std::ios::binary);
            if (!file) {
                std::cerr << "Error opening test file\n";
                return 1;
            }
            std::vector<uint8_t> testCode((std::istreambuf_iterator<char>(file)), std::istreambuf_iterator<char>());
            file.close();
            std::cerr << "[DEBUG] Loaded " << testCode.size() << " bytes\n";
            
            const uint16_t RAM_START = 0xC800;
            for (size_t i = 0; i < testCode.size(); ++i) {
                emulator.GetMemoryBus().Write(RAM_START + static_cast<uint16_t>(i), testCode[i]);
            }
            std::cerr << "[DEBUG] Test code written to RAM at 0x" << std::hex << RAM_START << std::dec << "\n";
            
            auto& regs = const_cast<CpuRegisters&>(emulator.GetCpu().Registers());
            regs.PC = RAM_START;
            std::cerr << "[DEBUG] PC set to 0x" << std::hex << RAM_START << std::dec << "\n";
        }
        
        // CRITICAL: Intentar leer VIA ANTES de ejecutar instrucciones
        std::cerr << "[DEBUG] Attempting to read VIA state BEFORE execution...\n";
        try {
            auto& bus = emulator.GetMemoryBus();
            uint8_t ifr_before = bus.Read(0xD00D);
            uint8_t ier_before = bus.Read(0xD00E);
            std::cerr << "[DEBUG] VIA state BEFORE execution: IFR=0x" << std::hex << static_cast<int>(ifr_before) 
                      << " IER=0x" << static_cast<int>(ier_before) << std::dec << "\n";
        } catch (...) {
            std::cerr << "[ERROR] Cannot read VIA even before execution!\n";
        }
        
        uint64_t totalCycles = 0;
        int cyclesToRun = std::atoi(argv[2]);
        std::cerr << "[DEBUG] Executing " << cyclesToRun << " cycles";
        if (traceMode) std::cerr << " (TRACE MODE - logging every instruction)";
        std::cerr << "...\n";
        
        int instructions = 0;
        auto& bus = emulator.GetMemoryBus();
        
        while (totalCycles < static_cast<uint64_t>(cyclesToRun)) {
            instructions++;
            
            // Read PC and opcode BEFORE execution
            uint16_t pc = emulator.GetCpu().Registers().PC;
            uint8_t opcode = bus.Read(pc);
            
            if (traceMode) {
                std::cerr << "[TRACE] Instr #" << instructions << ": PC=0x" 
                          << std::hex << pc << " Opcode=0x" 
                          << std::setw(2) << std::setfill('0') << static_cast<int>(opcode) 
                          << std::dec << "\n";
            } else if (instructions <= 10) {
                std::cerr << "[DEBUG] About to execute instruction " << instructions 
                          << " at PC=0x" << std::hex << pc << std::dec << "\n";
            }
            
            try {
                auto cycles = emulator.GetCpu().ExecuteInstruction(false, false);
                
                if (cycles == 0) {
                    std::cerr << "[ERROR] ExecuteInstruction returned 0 cycles!\n";
                    break;
                }
                totalCycles += cycles;
                
                // Log registros DESPUÉS de ejecutar si estamos en zona crítica
                if (traceMode && pc >= 0xF340 && pc <= 0xF350) {
                    auto& regs = emulator.GetCpu().Registers();
                    std::cerr << "  → A=" << std::hex << std::setw(2) << std::setfill('0') << static_cast<int>(regs.A)
                              << " B=" << std::setw(2) << std::setfill('0') << static_cast<int>(regs.B)
                              << " X=" << std::setw(4) << std::setfill('0') << regs.X
                              << " Y=" << std::setw(4) << std::setfill('0') << regs.Y
                              << " DP=" << std::setw(2) << std::setfill('0') << static_cast<int>(regs.DP)
                              << " CC=N" << static_cast<int>(regs.CC.Negative)
                              << "Z" << static_cast<int>(regs.CC.Zero)
                              << "V" << static_cast<int>(regs.CC.Overflow)
                              << "C" << static_cast<int>(regs.CC.Carry)
                              << std::dec << "\n";
                }
                
                if (!traceMode && instructions <= 10) {
                    std::cerr << "[DEBUG] Completed instr " << instructions << ": " << cycles << " cycles (total: " << totalCycles << ")\n";
                }
            } catch (const std::exception& e) {
                std::cerr << "[EXCEPTION in instruction " << instructions << "] " << e.what() << "\n";
                if (traceMode) {
                    std::cerr << "[TRACE] ERROR at instr #" << instructions << ": PC=0x" 
                              << std::hex << pc << " Opcode=0x" << static_cast<int>(opcode) 
                              << std::dec << "\n";
                }
                break;
            }
        }
        std::cerr << "[DEBUG] Executed " << totalCycles << " total cycles in " << instructions << " instructions\n";
        
        std::cerr << "[DEBUG] Serializing state...\n";
        json output = {
            {"cycles", totalCycles},
            {"cpu", serializeCpu(emulator)}
        };
        
        // C++ Original: Vectrexy lee VIA sin problemas
        // Rust también debe poder leer VIA post-ejecución
        // Intentamos leer cada registro VIA por separado para aislar el problema
        std::cerr << "[DEBUG] Attempting VIA serialization...\n";
        
        // CRITICAL: Primero intentar acceso directo a Via object
        std::cerr << "[DEBUG] Testing direct Via object access...\n";
        bool viaAccessOk = testViaDirectAccess(emulator);
        std::cerr << "[DEBUG] Direct Via access " << (viaAccessOk ? "SUCCEEDED" : "FAILED") << "\n";
        
        int ifr = safeReadViaRegister(emulator, 0xD00D, "IFR");
        int ier = safeReadViaRegister(emulator, 0xD00E, "IER");
        int t1_low = safeReadViaRegister(emulator, 0xD004, "Timer1_Low");
        int t1_high = safeReadViaRegister(emulator, 0xD005, "Timer1_High");
        int t2_low = safeReadViaRegister(emulator, 0xD008, "Timer2_Low");
        
        uint16_t timer1 = (t1_low >= 0 && t1_high >= 0) 
            ? (t1_low | (static_cast<uint16_t>(t1_high) << 8)) 
            : 0;
        uint16_t timer2 = (t2_low >= 0) ? t2_low : 0;
        
        output["via"] = {
            {"ifr", (ifr >= 0) ? ifr : 0},
            {"ier", (ier >= 0) ? ier : 0},
            {"timer1_counter", timer1},
            {"timer2_counter", timer2},
            {"port_a", safeReadViaRegister(emulator, 0xD001, "Port_A")},
            {"port_b", safeReadViaRegister(emulator, 0xD000, "Port_B")},
            {"shift_register", safeReadViaRegister(emulator, 0xD00A, "Shift_Register")}
        };
        
        // Intentar leer información adicional (sin crashear)
        output["vectors"] = {
            {"count", 0},  // No podemos acceder a RenderContext desde aquí
            {"lines", json::array()}
        };
        
        output["audio_samples"] = 0;  // No podemos acceder a AudioContext desde aquí
        
        std::cerr << "[DEBUG] Outputting JSON...\n";
        std::cout << output.dump(2) << std::endl;
        std::cerr << "[DEBUG] Done!\n";
        return 0;
    } catch (const std::exception& e) {
        std::cerr << "EXCEPTION: " << e.what() << "\n";
        return 1;
    } catch (...) {
        std::cerr << "UNKNOWN EXCEPTION\n";
        return 1;
    }
}
