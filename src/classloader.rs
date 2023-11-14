#![allow(dead_code)]
pub mod attribute_parser;
pub mod class_creator;
pub mod cp_decoder;
pub mod file_parser;
pub mod opcode_parser;

use std::usize;

use enumflags2::{bitflags, BitFlags};

#[bitflags]
#[derive(Clone, Copy, Debug)]
#[repr(u16)]
pub enum ClassAccessFlag {
    Public = 0x0001,
    Final = 0x0010,
    Super = 0x0020,
    Interface = 0x0200,
    Abstract = 0x0400,
    Synthetic = 0x1000,
    Annotation = 0x2000,
    Enum = 0x4000,
}

#[bitflags]
#[derive(Clone, Copy, Debug)]
#[repr(u16)]
pub enum FieldAccessFlag {
    Public = 0x0001,
    Private = 0x002,
    Protected = 0x004,
    Static = 0x008,
    Final = 0x0010,
    Volatile = 0x0040,
    Transient = 0x0080,
    Synthetic = 0x1000,
    Enum = 0x4000,
}

#[bitflags]
#[derive(Clone, Copy, Debug)]
#[repr(u16)]
pub enum MethodAccessFlag {
    Public = 0x0001,
    Private = 0x002,
    Protected = 0x004,
    Static = 0x008,
    Final = 0x0010,
    Synchronized = 0x0020,
    Bridge = 0x0040,
    Vargs = 0x0080,
    Native = 0x0100,
    Abstract = 0x0400,
    Strict = 0x0800,
    Synthetic = 0x1000,
}

#[derive(Debug, Copy, Clone)]
pub enum ReferenceKind {
    GetField = 1,
    GetStatic = 2,
    PutField = 3,
    PutStatic = 4,
    InvokeVirtual = 5,
    InvokeStatic = 6,
    InvokeSpecial = 7,
    NewInvokeSpecial = 8,
    InvokeInterface = 9,
}

impl TryFrom<u8> for ReferenceKind {
    type Error = &'static str;

    fn try_from(kind: u8) -> Result<Self, Self::Error> {
        match kind {
            1 => Ok(ReferenceKind::GetField),
            2 => Ok(ReferenceKind::GetStatic),
            3 => Ok(ReferenceKind::PutField),
            4 => Ok(ReferenceKind::PutStatic),
            5 => Ok(ReferenceKind::InvokeVirtual),
            6 => Ok(ReferenceKind::InvokeStatic),
            7 => Ok(ReferenceKind::InvokeSpecial),
            8 => Ok(ReferenceKind::NewInvokeSpecial),
            9 => Ok(ReferenceKind::InvokeInterface),
            _ => Err("ReferenceKind must be between 1-9 (inclusive)!"),
        }
    }
}

#[derive(Debug)]
pub enum CpInfo {
    ClassInfo {
        name_index: u16,
    },
    FieldRefInfo {
        class_index: u16,
        name_and_type_index: u16,
    },
    MethodRefInfo {
        class_index: u16,
        name_and_type_index: u16,
    },
    InterfaceMethodRefInfo {
        class_index: u16,
        name_and_type_index: u16,
    },
    StringInfo {
        string_index: u16,
    },
    IntegerInfo(i32),
    FloatInfo(f32),
    LongInfo(i64),
    DoubleInfo(f64),
    NameAndTypeInfo {
        name_index: u16,
        descriptor_index: u16,
    },
    UTF8INFO(String),
    MethodHandleInfo {
        reference_kind: ReferenceKind,
        reference_index: u16,
    },
    MethodTypeInfo {
        descriptor_index: u16,
    },
    InvokeDynamicInfo {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
}

impl CpInfo {
    pub fn as_class_info(&self) -> Option<u16> {
        if let CpInfo::ClassInfo { name_index } = *self {
            Some(name_index)
        } else {
            None
        }
    }

    pub fn as_field_ref_info(&self) -> Option<(u16, u16)> {
        if let CpInfo::FieldRefInfo {
            class_index,
            name_and_type_index,
        } = *self
        {
            Some((class_index, name_and_type_index))
        } else {
            None
        }
    }

    pub fn as_method_ref_info(&self) -> Option<(u16, u16)> {
        if let CpInfo::MethodRefInfo {
            class_index,
            name_and_type_index,
        } = *self
        {
            Some((class_index, name_and_type_index))
        } else {
            None
        }
    }

    pub fn as_interface_ref_info(&self) -> Option<(u16, u16)> {
        if let CpInfo::InterfaceMethodRefInfo {
            class_index,
            name_and_type_index,
        } = *self
        {
            Some((class_index, name_and_type_index))
        } else {
            None
        }
    }

    pub fn as_string_info(&self) -> Option<u16> {
        if let CpInfo::StringInfo { string_index } = *self {
            Some(string_index)
        } else {
            None
        }
    }

    pub fn as_integer_info(&self) -> Option<i32> {
        if let CpInfo::IntegerInfo(value) = *self {
            Some(value)
        } else {
            None
        }
    }

    pub fn as_float_info(&self) -> Option<f32> {
        if let CpInfo::FloatInfo(value) = *self {
            Some(value)
        } else {
            None
        }
    }

    pub fn as_long_info(&self) -> Option<i64> {
        if let CpInfo::LongInfo(value) = *self {
            Some(value)
        } else {
            None
        }
    }

    pub fn as_double_info(&self) -> Option<f64> {
        if let CpInfo::DoubleInfo(value) = *self {
            Some(value)
        } else {
            None
        }
    }

    pub fn as_name_and_type_info(&self) -> Option<(u16, u16)> {
        if let CpInfo::NameAndTypeInfo {
            name_index,
            descriptor_index,
        } = *self
        {
            Some((name_index, descriptor_index))
        } else {
            None
        }
    }

    pub fn as_utf8_info(&self) -> Option<&str> {
        if let CpInfo::UTF8INFO(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn as_mehod_handle_info(&self) -> Option<(ReferenceKind, u16)> {
        if let CpInfo::MethodHandleInfo {
            reference_kind,
            reference_index,
        } = *self
        {
            Some((reference_kind, reference_index))
        } else {
            None
        }
    }

    pub fn as_method_type_info(&self) -> Option<u16> {
        if let CpInfo::MethodTypeInfo { descriptor_index } = *self {
            Some(descriptor_index)
        } else {
            None
        }
    }

    pub fn as_invoke_dynamic_info(&self) -> Option<(u16, u16)> {
        if let CpInfo::InvokeDynamicInfo {
            bootstrap_method_attr_index,
            name_and_type_index,
        } = *self
        {
            Some((bootstrap_method_attr_index, name_and_type_index))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct RawAttributeInfo {
    attribute_name_index: u16,
    info: Vec<u8>,
}

#[derive(Debug)]
pub struct RawMethodInfo {
    access_flags: BitFlags<MethodAccessFlag>,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<RawAttributeInfo>,
}

#[derive(Debug)]
pub struct RawFieldInfo {
    access_flags: BitFlags<FieldAccessFlag>,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<RawAttributeInfo>,
}

#[derive(Debug)]
pub struct RawClassFile {
    minor_version: u16,
    major_version: u16,
    constant_pool: Vec<CpInfo>,
    access_flags: BitFlags<ClassAccessFlag>,
    this_class: u16,
    super_class: u16,
    interfaces: Vec<u16>,
    fields: Vec<RawFieldInfo>,
    methods: Vec<RawMethodInfo>,
    attributes: Vec<RawAttributeInfo>,
}

impl RawClassFile {
    pub fn get_java_cp_entry(&self, reference: usize) -> Option<&CpInfo> {
        if reference == 0 {
            panic!("Java CP Entries are 1 indexed");
        }
        self.constant_pool.get(reference - 1)
    }
}

#[derive(Debug)]
struct ExceptionTable {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

#[derive(Debug)]
pub struct CodeAttribute {
    max_stack: u16,
    max_locals: u16,
    code: Vec<u8>,
    exception_table: Vec<ExceptionTable>,
    attributes: Vec<AttributeInfo>,
}

#[derive(Debug)]
pub enum AttributeInfo {
    Code(CodeAttribute),
    SourceFile,
    LineNumberTable,
    LocalVariableTable,
}

impl AttributeInfo {
    pub fn as_code_attribute(&self) -> Option<&CodeAttribute> {
        if let AttributeInfo::Code(code_attribute) = self {
            Some(code_attribute)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct MethodInfo {
    access_flags: BitFlags<MethodAccessFlag>,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<AttributeInfo>,
}

#[derive(Debug)]
pub struct FieldInfo {
    access_flags: BitFlags<FieldAccessFlag>,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<AttributeInfo>,
}

#[derive(Debug)]
pub struct ClassFile {
    constant_pool: Vec<CpInfo>,
    access_flags: BitFlags<ClassAccessFlag>,
    this_class: u16,
    super_class: u16,
    interfaces: Vec<u16>,
    fields: Vec<FieldInfo>,
    methods: Vec<MethodInfo>,
    attributes: Vec<AttributeInfo>,
}

impl ClassFile {
    pub fn get_java_cp_entry(&self, reference: usize) -> Option<&CpInfo> {
        if reference == 0 {
            panic!("Java CP Entries are 1 indexed");
        }
        self.constant_pool.get(reference - 1)
    }
}
