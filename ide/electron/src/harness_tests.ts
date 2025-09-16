// Headless harness-driven tests for VIA & IRQ behavior.
// Each test invokes the already-built headless runtime (compiled form of headless.ts) indirectly by importing CPU directly
// only for assertions that aren't yet exposed via a Jest/Mocha framework. This keeps dependencies minimal.
// We simulate what a more formal test runner would do: run scenarios and emit a JSON summary + exit code 0/1.

import { Cpu6809 } from './emu6809';
import { testWaiExitViaIrq } from './harness_wai_test';
import { testShiftRegister, testPb7Toggle } from './harness_shift_pb7_tests';

interface TestResult { name:string; passed:boolean; detail?:any; }

function testTimer1Underflow(): TestResult {
  const cpu = new Cpu6809();
  cpu.setVectorMode('via');
  // Enable T1 interrupt (bit6) IER write: bit7=1 set bit6
  (cpu as any).viaWriteEffect(0x0E, 0xC0);
  // Program timer to small count (e.g., 12 cycles). Write low then high triggers load.
  (cpu as any).viaWriteEffect(0x04, 12 & 0xFF); // T1C-L
  (cpu as any).viaWriteEffect(0x05, 0x00);      // T1C-H (loads & clears IFR6)
  // Run enough cycles to force underflow (each step ~2 cycles average using rough table)
  for (let i=0;i<40;i++) cpu.step();
  const ifr = (cpu as any).viaRead(0x0D);
  const passed = (ifr & 0x40)!==0;
  return { name:'timer1_underflow_ifr6', passed, detail:{ ifr:ifr.toString(16) } };
}

function testTimer1IrqVector(): TestResult {
  const cpu = new Cpu6809();
  cpu.setVectorMode('via');
  // Install IRQ vector to 0x9000
  cpu.mem[0xFFF8]=0x90; cpu.mem[0xFFF9]=0x00;
  cpu.pc = 0x8000;
  (cpu as any).viaWriteEffect(0x0E, 0xC0); // enable T1 interrupt
  (cpu as any).viaWriteEffect(0x04, 8);    // low
  (cpu as any).viaWriteEffect(0x05, 0x00); // high -> load
  for (let i=0;i<80;i++) {
    cpu.step();
    if (cpu.pc >= 0x9000 && cpu.pc < 0x9100 && cpu.cc_i) break; // entered vector page
  }
  const inVectorPage = (cpu.pc & 0xFF00) === 0x9000;
  const passed = inVectorPage && cpu.cc_i === true;
  return { name:'timer1_irq_vector_taken', passed, detail:{ pc:cpu.pc.toString(16), inVectorPage, cc_i:cpu.cc_i } };
}

// Placeholder for future BIOS WAIT_RECAL authentic test (requires BIOS image present in runtime load path)
// Authentic WAIT_RECAL test: load BIOS via headless CPU global, run until a WAI is encountered and exited by Timer1 IRQ.
function testAuthenticWaitRecal(): TestResult {
  const cpu = new Cpu6809();
  const anyCpu:any = cpu as any;
  const biosAbs = 'C:/Users/DanielFerrerGuerrero/source/repos/pseudo-python/core/src/bios/vectrex.bin'.replace(/\\/g,'/');
  try {
    if (anyCpu.loadBios){
      const fs = require('fs');
      if (!fs.existsSync(biosAbs)){
        return { name:'wait_recal_auth', passed:false, detail:{ error:'bios_not_found', path:biosAbs } };
      }
      const buf = new Uint8Array(fs.readFileSync(biosAbs));
      anyCpu.loadBios(buf);
      anyCpu.setVectorMode('via');
      anyCpu.disableSyntheticIrq = true; anyCpu.devTimerAssist=false; anyCpu.autoStartUser=false;
      anyCpu.noInterceptWaitRecal = true; // force authentic WAIT_RECAL path instead of interception
      // Ensure IRQs permitted for timer (IER bit6)
      anyCpu.viaWriteEffect(0x0E, 0xC0);
      let sawWai=false, sawIrq=false, waiCycles=0, irqCycles=0, waiBeforeIrq=true;
      const deadline = 800000; // larger budget for authentic BIOS loop
      for (let i=0;i<deadline && !(sawWai && sawIrq); i++){
        cpu.step();
        if (!sawWai){
          if (anyCpu.waiWaiting) { sawWai=true; waiCycles=cpu.cycles; }
          else if (anyCpu.tailInstr && anyCpu.tailInstr.some((t:any)=>t.op===0x3E)) { sawWai=true; waiCycles=cpu.cycles; }
          else if (anyCpu.criticalEvents && anyCpu.criticalEvents.some((e:any)=>e.type==='wait_recal_enter')) { sawWai=true; waiCycles=cpu.cycles; }
        }
        if (!sawIrq && anyCpu.criticalEvents && anyCpu.criticalEvents.some((e:any)=>e.type==='irq_enter')) { sawIrq=true; irqCycles=cpu.cycles; }
      }
      if (sawWai && sawIrq) waiBeforeIrq = waiCycles <= irqCycles;
      const passed = sawWai && sawIrq && waiBeforeIrq;
      return { name:'wait_recal_auth', passed, detail:{ sawWai, sawIrq, waiCycles, irqCycles, waiBeforeIrq, totalCycles:cpu.cycles, path:biosAbs } };
    }
  } catch (e:any){
    return { name:'wait_recal_auth', passed:false, detail:{ error:e?.message||String(e), path:biosAbs } };
  }
  return { name:'wait_recal_auth', passed:false, detail:{ error:'loadBios_not_exposed', path:biosAbs } };
}

function runAll(){
  const results: TestResult[] = [];
  results.push(testTimer1Underflow());
  results.push(testTimer1IrqVector());
  results.push(testAuthenticWaitRecal());
  results.push(testWaiExitViaIrq());
  results.push(testShiftRegister());
  results.push(testPb7Toggle());
  const passedCount = results.filter(r=>r.passed).length;
  const summary = { total: results.length, passed: passedCount, failed: results.length - passedCount, results };
  console.log(JSON.stringify({ event:'harness_summary', summary }, null, 2));
  if (summary.failed > 0){
    console.error('Some harness tests failed');
    process.exit(1);
  }
}

runAll();
