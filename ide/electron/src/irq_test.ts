// Simple IRQ test: set IRQ vector, enable Timer1 interrupt, let it underflow, ensure PC jumps and I flag set.
import { Cpu6809 } from './emu6809';

function irqTest(){
  const cpu = new Cpu6809();
  cpu.setVectorMode('via');
  // Install a fake IRQ vector at 0xFFF8/0xFFF9 pointing to 0x9000
  cpu.mem[0xFFF8] = 0x90; cpu.mem[0xFFF9] = 0x00;
  // Initialize PC somewhere safe
  cpu.pc = 0x8000;
  // Enable Timer1 interrupt (bit6) in IER
  // Write set bits (bit7=1) mask 0x40
  (cpu as any).viaWriteEffect(0x0E, 0xC0);
  // Program timer latch small
  (cpu as any).viaWriteEffect(0x06, 0x05); // latch low
  (cpu as any).viaWriteEffect(0x07, 0x00); // latch high
  (cpu as any).viaWriteEffect(0x04, 0x05); // counter low
  (cpu as any).viaWriteEffect(0x05, 0x00); // counter high
  // Execute until IRQ should fire
  for (let i=0;i<50;i++) cpu.step();
  const jumped = cpu.pc === 0x9000;
  const iSet = cpu.cc_i === true;
  console.log('IRQ vector taken:', jumped, 'PC=', cpu.pc.toString(16), 'I flag:', iSet);
}

irqTest();
