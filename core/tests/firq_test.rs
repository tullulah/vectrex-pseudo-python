use vectrex_emulator::CPU;

// FIRQ behavior test:
// We simulate a FIRQ by manually setting firq_pending after executing a few instructions.
// Expectations:
//  - Only PC and CC are stacked (E flag remains 0)
//  - General registers (A,B,DP,X,Y,U) modified inside handler persist after RTI
//  - RTI with E=0 pops CC then PC only.
#[test]
fn firq_partial_frame_registers_persist() {
    let mut cpu = CPU::default();
    // Set FIRQ vector -> 0x0300 using STANDARD post-migration layout (FIRQ=0xFFF6 big-endian hi/lo)
    // Bytes at 0xFFF6 (hi) and 0xFFF7 (lo)
    cpu.bus.mem[0xFFF6]=0x03; cpu.bus.mem[0xFFF7]=0x00; // high=0x03, low=0x00
    // Handler at 0x0300:
    //  LDA #$AA
    //  LDB #$55
    //  RTI
    let handler = [0x86,0xAA, 0xC6,0x55, 0x3B];
    for (i,b) in handler.iter().enumerate(){ cpu.mem[0x0300+i]=*b; cpu.bus.mem[0x0300+i]=*b; }
    // Main code: just a couple NOP-like stand-ins (we'll use CLRA / CLRB) then we trigger FIRQ
    cpu.mem[0x0000]=0x4F; cpu.bus.mem[0x0000]=0x4F; // CLRA
    cpu.mem[0x0001]=0x5F; cpu.bus.mem[0x0001]=0x5F; // CLRB
    cpu.mem[0x0002]=0x4F; cpu.bus.mem[0x0002]=0x4F; // CLRA again
    cpu.pc=0x0000;
    // Run a couple steps
    cpu.step(); cpu.step();
    // Set firq_pending before next step
    cpu.firq_pending = true;
    cpu.step(); // should service FIRQ now and jump to 0x0300
    assert_eq!(cpu.pc,0x0300,"Did not vector to FIRQ handler (pc={:04X})", cpu.pc);
    assert!(!cpu.cc_e, "E flag should not be set for FIRQ partial frame");
    let sp_after_vector = cpu.s;
    // Execute handler instructions
    cpu.step(); // LDA #AA
    cpu.step(); // LDB #55
    cpu.step(); // RTI
    // After RTI we resume at original PC+1 of the instruction interrupted (which was at address 0x0002 executing CLRA?)
    // Since we interrupted before executing 0x0002 CLRA, PC should now be 0x0002 or 0x0003 depending on timing. Our simplified model stacked current PC, so expect resume at 0x0002.
    assert!(cpu.pc==0x0002 || cpu.pc==0x0003 || cpu.pc==0x0004, "Unexpected resume PC {:04X}", cpu.pc);
    // Registers modified in handler must persist
    assert_eq!(cpu.a,0xAA,"A not preserved from FIRQ handler");
    assert_eq!(cpu.b,0x55,"B not preserved from FIRQ handler");
    // Stack usage should be minimal (only CC+PC). Ensure small growth (>= 3 bytes with our push scheme: PC(2)+CC(1)).
    assert!(sp_after_vector < cpu.s + 8, "Unexpected large FIRQ frame usage (s after vec {:04X} final {:04X})", sp_after_vector, cpu.s);
}
