// Harness test: ensure WAI is exited by Timer1 IRQ and PC jumps to vector.
import { Cpu6809 } from './emu6809';

export interface WaiTestResult { name:string; passed:boolean; detail:any; }

export function testWaiExitViaIrq(): WaiTestResult {
  const cpu = new Cpu6809();
  cpu.setVectorMode('via');
  // IRQ vector -> 0x8800
  cpu.mem[0xFFF8]=0x88; cpu.mem[0xFFF9]=0x00;
  // Program a tiny Timer1 interval
  (cpu as any).viaWriteEffect(0x0E, 0xC0); // enable T1
  (cpu as any).viaWriteEffect(0x04, 20);   // low
  (cpu as any).viaWriteEffect(0x05, 0x00); // high triggers load
  // Place WAI opcode 0x3E at 0x8000 and execute
  cpu.mem[0x8000] = 0x3E; cpu.pc = 0x8000;
  cpu.step(); // execute WAI
  const enteredWai = (cpu as any).waiWaiting === true;
  for (let i=0;i<500 && (cpu as any).waiWaiting; i++) cpu.step();
  const exitedWai = !(cpu as any).waiWaiting;
  const inVectorPage = (cpu.pc & 0xFF00) === 0x8800;
  const passed = enteredWai && exitedWai && inVectorPage && cpu.cc_i;
  return { name:'wai_exit_via_timer1_irq', passed, detail:{ enteredWai, exitedWai, pc:cpu.pc.toString(16), cc_i:cpu.cc_i, ifr:(cpu as any).viaRead(0x0D).toString(16), inVectorPage } };
}
