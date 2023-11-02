pub mod file_parser;

struct CpInfo{
    tag: u8,
    info: Vec<u8>
}

struct AttributeInfo{
    attribute_name_index: u16,
    attribute_length: u32,
    info: Vec<u8>
}

struct ClassFile {
    minor_version: u16,
    major_version: u16,
    constant_pool_count: u16,
    //cp_info        constant_pool[constant_pool_count-1];
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interfaces_count: u16,
    interfaces: Vec<u16>,
    fields_count: u16,
    //field_info     fields[fields_count];
    methods_count: u16,
    //method_info    methods[methods_count];
    attributes_count: u16,
    attributes: Vec<AttributeInfo>
}