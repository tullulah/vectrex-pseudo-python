// Tests for shift register completion (IFR bit4) and PB7 toggle (ACR bit7)
import { Cpu6809 } from './emu6809';

interface TestResult { name:string; passed:boolean; detail?:any; }

export function testShiftRegister(): TestResult {
	const cpu = new Cpu6809();
	cpu.setVectorMode('via');
	// ACR internal shift mode bits2-4 to 0b100
	(cpu as any).viaWriteEffect(0x0B, 0x10);
	// Enable SR interrupt (IFR4) in IER: bit7=1 + bit4 mask => 0x90
	(cpu as any).viaWriteEffect(0x0E, 0x90);
	// Load SR with pattern 0xA5 which will be shifted out
	(cpu as any).viaWriteEffect(0x0A, 0xA5);
	for (let i=0;i<100;i++) cpu.step();
	const ifr = (cpu as any).viaRead(0x0D);
	const srVal = (cpu as any).viaRead(0x0A);
	const passed = (ifr & 0x10)!==0;
	return { name:'shift_register_complete_ifr4', passed, detail:{ ifr:ifr.toString(16), sr:srVal.toString(16) } };
}

export function testPb7Toggle(): TestResult {
	const cpu = new Cpu6809();
	cpu.setVectorMode('via');
	// Enable Timer1 interrupt to allow underflow detection but not required for PB7 toggle itself; PB7 toggle uses ACR bit7
	(cpu as any).viaWriteEffect(0x0E, 0xC0); // enable T1
	(cpu as any).viaWriteEffect(0x04, 10); // T1 low
	(cpu as any).viaWriteEffect(0x05, 0x00); // T1 high
	(cpu as any).viaWriteEffect(0x0B, 0xC0); // ACR bit7+bit6
	let toggles=0; let last: number | null = null;
	for (let i=0;i<500;i++){
		cpu.step();
		const orb = (cpu as any).viaRead(0x00);
		const pb7 = (orb & 0x80)?1:0;
		if (last===null) last=pb7; else if (pb7!==last){ toggles++; last=pb7; }
	}
	const passed = toggles>=1;
	return { name:'pb7_toggle_on_t1_underflow', passed, detail:{ toggles } };
}

if (require.main === module){
	const results=[testShiftRegister(), testPb7Toggle()];
	console.log(JSON.stringify({ event:'shift_pb7_tests', results }, null, 2));
	const allPass = results.every(r=>r.passed);
	if (!allPass) process.exit(1);
}