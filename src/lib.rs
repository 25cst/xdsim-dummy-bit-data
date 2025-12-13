use libxdsim::v0::component::Type;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Bit(pub bool);

impl Type for Bit {
    fn serialize(&self) -> Box<[u8]> {
        Box::new([if self.0 { 1 } else { 0 }])
    }

    fn deserialize(bytes: Box<[u8]>) -> Self where Self: Sized {
        assert_eq!(bytes.len(), 1);
        Bit(bytes[0] != 0)
    }
}

#[unsafe(no_mangle)]
pub fn deserialize_boxed(bytes: Box<[u8]>) -> Box<dyn Type> {
    Box::new(Bit::deserialize(bytes))
}
