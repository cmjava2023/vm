pub mod attribute_parser;
pub mod class_creator;
pub mod constant_pool;
pub mod cp_decoder;
pub mod file_parser;
pub mod opcode_parser;
pub mod raw;

use std::{path::Path, rc::Rc, usize};

use enumflags2::BitFlags;

use self::{
    attribute_parser::parse_attributes, class_creator::create_bytecode_class,
    file_parser::parse,
};
use crate::{
    class::{
        access_flags::{ClassAccessFlag, FieldAccessFlag, MethodAccessFlag},
        Class,
    },
    classloader::constant_pool::CpInfo,
    heap::Heap,
};

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

#[allow(dead_code)] // implementing Exceptions is later feature
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
    #[allow(dead_code)] // implementing Exceptions is later feature
    exception_table: Vec<ExceptionTable>,
    #[allow(dead_code)] // optional features that may get implemented later
    attributes: Vec<AttributeInfo>,
}

#[derive(Debug)]
pub enum AttributeInfo {
    Code(CodeAttribute),
    SourceFile,
    LineNumberTable,
    LocalVariableTable,
    Exceptions,
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

#[allow(dead_code)] // handling field type/access flags is an optional feature
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
    #[allow(dead_code)] // handling access flags is an optional feature
    access_flags: BitFlags<ClassAccessFlag>,
    this_class: u16,
    #[allow(dead_code)] // inheritance is a later feature
    super_class: u16,
    #[allow(dead_code)] // inheritance is a later feature
    interfaces: Vec<u16>,
    #[allow(dead_code)] // object creation is handled later
    fields: Vec<FieldInfo>,
    methods: Vec<MethodInfo>,
    #[allow(dead_code)] // ?, not relevant for current feature set
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

pub fn load_class<P: AsRef<Path>>(
    path_to_file: P,
    heap: &mut Heap,
) -> Rc<dyn Class> {
    let raw_class = parse(path_to_file).unwrap();
    let class = parse_attributes(raw_class);
    let bytecode_class: Rc<dyn Class> =
        Rc::new(create_bytecode_class(&class, heap));
    if bytecode_class.package() == "" {
        heap.add_class(
            bytecode_class.name().to_string(),
            bytecode_class.clone(),
        );
    } else {
        heap.add_class(
            format!("{}/{}", bytecode_class.package(), bytecode_class.name()),
            bytecode_class.clone(),
        );
    }
    bytecode_class
}
