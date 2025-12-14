use libxdsim::{
    defs::PackageDefinition,
    package_ident_v0,
    v0::{ComponentType, PackageDefinitionV0, component::Data},
};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Bit(pub bool);

impl Data for Bit {
    fn serialize(&self) -> Box<[u8]> {
        Box::new([if self.0 { 1 } else { 0 }])
    }

    fn deserialize(bytes: Box<[u8]>) -> Self
    where
        Self: Sized,
    {
        assert_eq!(bytes.len(), 1);
        Bit(bytes[0] != 0)
    }
}

#[cfg_attr(feature = "export-abi", unsafe(no_mangle))]
pub fn deserialize_boxed(bytes: Box<[u8]>) -> Box<dyn Data> {
    Box::new(Bit::deserialize(bytes))
}

#[cfg_attr(feature = "export-abi", unsafe(no_mangle))]
pub fn package_definition() -> Box<dyn PackageDefinition> {
    Box::new(PackageDefinitionV0 {
        ident: package_ident_v0!("dummy-bit"),
        component_type: ComponentType::Data,

        authors: &["siriusmart"],
        description: "An example bit data type.",
        homepage: "https://github.com/25cst/xdsim-dummy-bit-data",
    })
}
