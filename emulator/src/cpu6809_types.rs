#[derive(Clone,Copy,Debug,Default)]
pub struct LoopSample {
    pub pc: u16,
    pub a: u8, pub b: u8, pub x: u16, pub y: u16, pub u: u16, pub s: u16, pub dp: u8,
    pub via_ifr: u8, pub via_ier: u8, pub via_acr: u8, pub via_pcr: u8,
    pub cycles: u64,
}

#[derive(Clone,Debug)]
pub struct TraceEntry {
    pub pc:u16, pub opcode:u8, pub sub:u8, pub a:u8, pub b:u8, pub x:u16, pub y:u16, pub u:u16, pub s:u16, pub dp:u8,
    pub op_str: Option<String>, pub loop_count:u32, pub flags:u8, pub cycles:u32, pub illegal: bool, pub call_depth: u16
}

#[derive(Clone,Copy,Default,Debug)]
pub struct InputState { pub x:i16, pub y:i16, pub buttons:u8 }

// Shadow call stack instrumentation
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum ShadowKind { JSR, BSR, LBSR, IRQ, FIRQ, NMI, SWI, SWI2, SWI3, PshsPc, PshuPc }
impl Default for ShadowKind { fn default()->Self { ShadowKind::JSR } }

#[derive(Clone,Copy,Debug,Default)]
pub struct ShadowFrame { pub ret: u16, pub sp_at_push: u16, pub kind: ShadowKind }
