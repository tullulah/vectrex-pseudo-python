code = r"""#include <iostream>
#include <fstream>
#include <vector>
#include <string>
#include "emulator/Emulator.h"
#include "nlohmann/json.hpp"

using json = nlohmann::json;

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
    if (argc != 3) {
        std::cerr << "Usage: " << argv[0] << " <test.bin> <cycles>\n";
        return 1;
    }
    Emulator emulator;
    const char* biosPath = "C:/Users/DanielFerrerGuerrero/source/repos/pseudo-python/ide/frontend/dist/bios.bin";
    emulator.Init(biosPath);  // Init carga la BIOS automáticamente
    std::ifstream file(argv[1], std::ios::binary);
    if (!file) {
        std::cerr << "Error opening test file\n";
        return 1;
    }
    std::vector<uint8_t> testCode((std::istreambuf_iterator<char>(file)), std::istreambuf_iterator<char>());
    file.close();
    const uint16_t RAM_START = 0xC800;
    for (size_t i = 0; i < testCode.size(); ++i) {
        emulator.GetMemoryBus().Write(RAM_START + static_cast<uint16_t>(i), testCode[i]);
    }
    auto& regs = const_cast<CpuRegisters&>(emulator.GetCpu().Registers());
    regs.PC = RAM_START;
    uint64_t totalCycles = 0;
    int cyclesToRun = std::atoi(argv[2]);
    while (totalCycles < static_cast<uint64_t>(cyclesToRun)) {
        auto cycles = emulator.GetCpu().ExecuteInstruction(false, false);
        totalCycles += cycles;
    }
    json output = {
        {"cycles", totalCycles},
        {"cpu", serializeCpu(emulator)},
        {"via", serializeVia(emulator)}
    };
    std::cout << output.dump(2) << std::endl;
    return 0;
}
"""

with open('main.cpp', 'w') as f:
    f.write(code)
print("✅ main.cpp creado")
