pub mod file_parser;

use enumflags2::{bitflags, BitFlags };


#[bitflags]
#[derive(Clone, Copy)]
#[repr(u16)]
enum ClassAccessFlag{
    Public=0x0001,
    Final=0x0010,
    Super=0x0020,
    Interface=0x0200,
    Abstract=0x0400,
    Synthetic=0x1000,
    Annotation=0x2000,
    Enum=0x4000,
}

#[bitflags]
#[derive(Clone, Copy)]
#[repr(u16)]
enum FieldAccessFlag{
    Public=0x0001,
    Private=0x002,
    Protected=0x004,
    Static=0x008,
    Final=0x0010,
    Volatile=0x0040,
    Transient=0x0080,
    Synthetic=0x1000,
    Enum=0x4000,
}

#[bitflags]
#[derive(Clone, Copy)]
#[repr(u16)]
enum MethodAccessFlag{
    Public=0x0001,
    Private=0x002,
    Protected=0x004,
    Static=0x008,
    Final=0x0010,
    Synchronized=0x0020,
    Bridge=0x0040,
    Vargs=0x0080,
    Native=0x0100,
    Abstract=0x0400,
    Strict=0x0800,
    Synthetic=0x1000,
}

enum ReferenceKind{
    GetField=1,
    GetStatic=2,
    PutField=3,
    PutStatic=4,
    InvokeVirtual=5,
    InvokeStatic=6,
    InvokeSpecial=7,
    NewInvokeSpecial=8,
    InvokeInterface=9,
}

impl TryFrom<u8> for ReferenceKind {
    type Error = &'static str;

    fn try_from(kind: u8) -> Result<Self, Self::Error> {
        match kind {
            1 =>Ok(ReferenceKind::GetField), 
            2 =>Ok(ReferenceKind::GetStatic),
            3 =>Ok(ReferenceKind::PutField),
            4 =>Ok(ReferenceKind::PutStatic),
            5 =>Ok(ReferenceKind::InvokeVirtual),
            6 =>Ok(ReferenceKind::InvokeStatic),
            7 =>Ok(ReferenceKind::InvokeSpecial),
            8 =>Ok(ReferenceKind::NewInvokeSpecial),
            9 =>Ok(ReferenceKind::InvokeInterface),
            _ => Err("ReferenceKind must be between 1-9 (inclusive)!")
        }
    }
}

enum  CpInfo{
    ClassInfo{name_index: u16},
    FieldRefInfo{class_index: u16, name_and_type_index: u16},
    MethodRefInfo{class_index: u16, name_and_type_index: u16},
    InterfaceMethodRefInfo{class_index: u16, name_and_type_index: u16},
    StringInfo{string_index:u16},
    IntegerInfo(i32),
    FloatInfo(f32),
    LongInfo(i64),
    DoubleInfo(f64),
    NameAndTypeInfo{namae_index: u16, descriptor_index: u16},
    UTF8INFO(String),
    MethodHandleInfo{reference_kind: ReferenceKind, reference_index: u16},
    MethodTypeInfo{descriptor_index: u16},
    InvokeDynamicInfo{bootstrap_method_attr_index: u16, name_and_type_index: u16},

}

struct AttributeInfo{
    attribute_name_index: u16,
    info: Vec<u8>,
}

struct MethodInfo{
    access_flags: BitFlags::<MethodAccessFlag>,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<AttributeInfo>,
}

struct FieldInfo{
    access_flags: BitFlags::<FieldAccessFlag>,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<AttributeInfo>,

}

struct ClassFile {
    minor_version: u16,
    major_version: u16,
    cp_info :Vec<CpInfo>,
    access_flags: BitFlags::<ClassAccessFlag>,
    this_class: u16,
    super_class: u16,
    interfaces_count: u16,
    interfaces: Vec<u16>,
    field_info: Vec<FieldInfo>,
    method_info: Vec<MethodInfo>,
    attributes_count: u16,
    attributes: Vec<AttributeInfo>
}