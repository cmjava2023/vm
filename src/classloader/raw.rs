use enumflags2::BitFlags;

use crate::{
    class::access_flags::{ClassAccessFlag, FieldAccessFlag, MethodAccessFlag},
    classloader::CpInfo,
};

#[derive(Debug)]
pub struct RawAttributeInfo {
    pub(super) attribute_name_index: u16,
    pub(super) info: Vec<u8>,
}

#[derive(Debug)]
pub struct RawMethodInfo {
    pub(super) access_flags: BitFlags<MethodAccessFlag>,
    pub(super) name_index: u16,
    pub(super) descriptor_index: u16,
    pub(super) attributes: Vec<RawAttributeInfo>,
}

#[derive(Debug)]
pub struct RawFieldInfo {
    pub(super) access_flags: BitFlags<FieldAccessFlag>,
    pub(super) name_index: u16,
    pub(super) descriptor_index: u16,
    pub(super) attributes: Vec<RawAttributeInfo>,
}

#[derive(Debug)]
pub struct RawClassFile {
    #[allow(dead_code)] // verification is not a priority right now
    pub(super) minor_version: u16,
    #[allow(dead_code)] // verification is not a priority right now
    pub(super) major_version: u16,
    pub(super) constant_pool: Vec<CpInfo>,
    pub(super) access_flags: BitFlags<ClassAccessFlag>,
    pub(super) this_class: u16,
    pub(super) super_class: u16,
    pub(super) interfaces: Vec<u16>,
    pub(super) fields: Vec<RawFieldInfo>,
    pub(super) methods: Vec<RawMethodInfo>,
    pub(super) attributes: Vec<RawAttributeInfo>,
}

impl RawClassFile {
    pub fn get_java_cp_entry(&self, reference: usize) -> Option<&CpInfo> {
        if reference == 0 {
            panic!("Java CP Entries are 1 indexed");
        }
        self.constant_pool.get(reference - 1)
    }
}
