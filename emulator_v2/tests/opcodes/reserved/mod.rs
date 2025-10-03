// Reserved opcodes tests
// Estos opcodes NO están definidos en la especificación MC6809
// Tests verifican que el emulador correctamente hace panic al encontrarlos

pub mod test_reserved_0x01;
pub mod test_reserved_0x02;
pub mod test_reserved_0x05;
pub mod test_reserved_0x0B;
pub mod test_reserved_0x14;
pub mod test_reserved_0x15;
pub mod test_reserved_0x18;
pub mod test_reserved_0x1B;
