// Minimal ad-hoc test for Timer1 underflow -> IFR bit6 set.
import { Cpu6809 } from './emu6809';

function runTimerTest(){
  const cpu = new Cpu6809();
  cpu.setVectorMode('via');
  // Enable T1 interrupt (bit6) in IER: write 0x80|0x40 (set bit6)
  // IER at 0xD00E
  cpu['viaWriteEffect']?.(0x0E, 0xC0); // if private; accessing for dev test
  // Load latch & counter with small value (e.g., 10 cycles)
  // Using helper methods would be cleaner, but we directly poke provisional layout low/high reversed comment considered.
  cpu['viaWriteEffect']?.(0x04, 10 & 0xFF); // low
  cpu['viaWriteEffect']?.(0x05, 0x00); // high
  cpu['viaWriteEffect']?.(0x06, 10 & 0xFF); // latch low
  cpu['viaWriteEffect']?.(0x07, 0x00); // latch high
  // Execute enough instructions to exceed 10 cycles (each step ~2 cyc in rough model)
  for (let i=0;i<20;i++){ cpu.step(); }
  const ifr = cpu['viaRead']?.(0x0D) ?? 0;
  const passed = (ifr & 0x40)!==0;
  console.log('Timer1 underflow IFR bit6 set:', passed, 'IFR=', ifr.toString(16));
}

runTimerTest();
