use libxdsim::component::Type;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Bit(pub bool);

impl Type for Bit {
    fn serialize(&self) -> Box<[u8]> {
        Box::new([if self.0 { 1 } else { 0 }])
    }
}

pub const fn id() -> (&'static str, u16, u16) {
    ("dummy-bit", 0, 1)
}

pub fn deserialize(bytes: Box<[u8]>) -> Bit {
    assert_eq!(bytes.len(), 1);
    Bit(bytes[0] != 0)
}
