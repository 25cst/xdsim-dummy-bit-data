#[repr(C)]
#[derive(Clone, Copy)]
pub struct Bit(pub bool);

impl Bit {
    pub const fn id() -> &'static str {
        "dummies-0.1::bit"
    }

    pub fn serialize(&self) -> Vec<u8> {
        vec![if self.0 { 1 } else { 0 }]
    }

    pub fn deserialize(bytes: Vec<u8>) -> Self {
        assert_eq!(bytes.len(), 1);
        Self(bytes[0] != 0)
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Byte(pub u8);

impl Byte {
    pub const fn id() -> &'static str {
        "dummies-0.1::byte"
    }

    pub fn serialize(&self) -> Vec<u8> {
        vec![self.0]
    }

    pub fn deserialize(bytes: Vec<u8>) -> Self {
        assert_eq!(bytes.len(), 1);
        Self(bytes[0])
    }
}
