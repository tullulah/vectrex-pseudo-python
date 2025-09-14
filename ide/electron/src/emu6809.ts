// Minimal 6809 emulator (subset) translated from Rust CPU version for prototype.
// Supports limited opcodes used by generated minimal binaries and BIOS interception.

export interface VectorSegment { x1:number; y1:number; x2:number; y2:number; intensity:number; }

export class Cpu6809 {
  a=0; b=0; dp=0xD0; x=0; u=0; pc=0; cc_z=false; cc_n=false; cc_c=false;
  mem = new Uint8Array(65536);
  callStack: number[] = [];
  biosPresent = false;
  lastIntensity = 0x5F;
  frameSegments: VectorSegment[] = [];
  frameReady = false;
  trace = false;

  loadBin(bytes: Uint8Array, base=0) { for (let i=0;i<bytes.length;i++){ const addr=base+i; if (addr<65536) this.mem[addr]=bytes[i]; } }
  loadBios(bytes: Uint8Array) { if (bytes.length===8192){ this.loadBin(bytes,0xF000); this.biosPresent=true; } }

  private setD(v:number){ this.a=(v>>8)&0xFF; this.b=v&0xFF; }
  private d(){ return (this.a<<8)|this.b; }
  private nz8(v:number){ this.cc_z=(v&0xFF)===0; this.cc_n=(v&0x80)!==0; }
  private nz16(v:number){ this.cc_z=(v&0xFFFF)===0; this.cc_n=(v&0x8000)!==0; }

  private interceptBios(addr:number){
    switch(addr){
      case 0xF192: // WAIT_RECAL
        this.dp=0xD0; // Frame boundary
        this.frameReady = true;
        break;
      case 0xF2A5: // INTENSITY_5F
        this.lastIntensity = 0x5F; break;
      case 0xF2AB: // INTENSITY_A
        this.lastIntensity = this.a; break;
      case 0xF3DD: // DRAW_VL
        this.decodeVectorList(); break;
      // RESET0REF / MOVETO etc. ignored for now
      default: break;
    }
  }

  private decodeVectorList(){
    // Assume U points to list: sequence of (y,x) signed bytes; bit7 set in y = end (clear bit then use); segments connect consecutive points starting at first.
    let ptr = this.u & 0xFFFF; if (ptr>=0x10000) return;
    const coords: Array<[number,number]> = [];
    for (let safety=0; safety<1024; safety++){
      const yRaw = this.mem[ptr];
      const end = (yRaw & 0x80)!==0;
      const y = ((yRaw & 0x7F) << 25) >> 25; // sign extend 7 bits -> 32
      const xRaw = this.mem[(ptr+1)&0xFFFF];
      const x = (xRaw<<24)>>24; // sign extend 8 bits
      coords.push([x,y]);
      ptr = (ptr+2)&0xFFFF;
      if (end) break;
    }
    if (coords.length>1){
      for (let i=1;i<coords.length;i++){
        const [x1,y1]=coords[i-1]; const [x2,y2]=coords[i];
        this.frameSegments.push({x1,y1,x2,y2,intensity:this.lastIntensity});
      }
    }
  }

  step(): boolean {
    const op = this.mem[this.pc];
    const pc0=this.pc;
    this.pc = (this.pc + 1) & 0xFFFF;
    switch(op){
      case 0xCC: { // LDD imm
        const hi=this.mem[this.pc]; const lo=this.mem[(this.pc+1)&0xFFFF]; this.pc=(this.pc+2)&0xFFFF; this.setD((hi<<8)|lo); this.nz16(this.d()); break; }
      case 0x86: { const v=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; this.a=v; this.nz8(this.a); break; }
      case 0xC6: { const v=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; this.b=v; this.nz8(this.b); break; }
      case 0x8E: { const hi=this.mem[this.pc]; const lo=this.mem[(this.pc+1)&0xFFFF]; this.pc=(this.pc+2)&0xFFFF; this.x=((hi<<8)|lo)&0xFFFF; break; }
      case 0xCE: { const hi=this.mem[this.pc]; const lo=this.mem[(this.pc+1)&0xFFFF]; this.pc=(this.pc+2)&0xFFFF; this.u=((hi<<8)|lo)&0xFFFF; break; }
      case 0xBD: { const hi=this.mem[this.pc]; const lo=this.mem[(this.pc+1)&0xFFFF]; this.pc=(this.pc+2)&0xFFFF; const addr=((hi<<8)|lo)&0xFFFF; if (addr>=0xF000){ this.interceptBios(addr); } else { this.callStack.push(this.pc); this.pc=addr; } break; }
      case 0x39: { // RTS
        this.pc = this.callStack.pop() ?? this.pc; break; }
      case 0x4F: { this.a=0; this.nz8(this.a); break; }
      case 0x5F: { this.b=0; this.nz8(this.b); break; }
      case 0x20: { const off=(this.mem[this.pc]<<24)>>24; this.pc=(this.pc+1)&0xFFFF; this.pc=(this.pc + off) & 0xFFFF; break; }
      case 0x27: { const off=(this.mem[this.pc]<<24)>>24; this.pc=(this.pc+1)&0xFFFF; if (this.cc_z) this.pc=(this.pc+off)&0xFFFF; break; }
      case 0x26: { const off=(this.mem[this.pc]<<24)>>24; this.pc=(this.pc+1)&0xFFFF; if (!this.cc_z) this.pc=(this.pc+off)&0xFFFF; break; }
      default:
        // Stop on unimplemented opcode to avoid infinite loops.
        if (this.trace) console.warn('UNIMPL', op.toString(16), 'at', pc0.toString(16));
        return false;
    }
    return true;
  }

  runUntilFrame(maxSteps=200000){
    this.frameReady=false;
    for (let i=0;i<maxSteps;i++){
      if (!this.step()) break;
      if (this.frameReady) break;
    }
    const segs = this.frameSegments.slice();
    this.frameSegments.length = 0;
    return { frameReady:this.frameReady, segments: segs };
  }
}

export const globalCpu = new Cpu6809();
