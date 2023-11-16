pub mod builtin_classes;
pub mod bytecode_classes;

use core::fmt;
use std::{any::Any, rc::Rc};

use crate::executor::{Frame, OpCode, Update};

#[derive(Debug, Clone)]
pub struct Method {
    pub code: MethodCode,
    pub name: String,
    // TODO parameter
    // TODO return type
    // TODO flags
    // TODO attributes
}

#[derive(Debug, Clone)]
pub enum MethodCode {
    Bytecode(Code),
    // TODO pass execution frame (i.e. stack and local variables)
    // TODO return value?
    Rust(for<'a> fn(&'a mut Frame) -> Update),
}

pub trait Class {
    fn methods(&self) -> &[Rc<Method>];
    fn static_fields(&self) -> &[Rc<Field>];
    fn instance_fields(&self) -> &[String];
    // TODO flags
    fn package(&self) -> &str;
    fn name(&self) -> &str;
    fn super_class(&self) -> Option<Rc<dyn Class>>;
    // TODO how are interfaces represented?
    fn interfaces(&self) -> &[Rc<dyn std::any::Any>];
    // TODO attributes
}

impl dyn Class {
    pub fn get_method(&self, method_name: &str) -> Option<Rc<Method>> {
        self.methods()
            .iter()
            .find(|element| element.name == method_name).cloned()
    }

    pub fn get_static_field(&self, field_name: &str) -> Option<Rc<Field>> {
        self.static_fields()
            .iter()
            .find(|element| element.name == field_name).cloned()
    }
}

impl std::fmt::Debug for dyn Class {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Class")
    }
}

#[derive(Debug)]
pub struct BytecodeClass {
    pub methods: Vec<Rc<Method>>,
    pub static_fields: Vec<Rc<Field>>,
    pub instance_fields: Vec<String>,
    // TODO flags
    pub package: String,
    pub name: String,
    pub super_class: Option<Rc<dyn Class>>,
    // TODO how are interfaces represented?
    pub interfaces: Vec<Rc<dyn std::any::Any>>,
    // TODO attributes
}

#[derive(Debug)]
pub struct Field {
    pub name: String,
    // TODO flags
    // TODO attributes
    // TODO data type
    pub value: FieldValue,
}

#[derive(Debug, Clone)]
pub enum FieldValue {
    // Primitive Types
    //   Integral Types
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Char(u16),
    //    Floating-Point Types
    Float(f32),
    Double(f64),
    //    Other
    Boolean(u8),
    // Reference Types
    // TODO different reference types (array, interface)
    Reference(Option<Rc<dyn ClassInstance>>),
}

impl FieldValue {
    pub fn byte() -> Self {
        Self::Byte(0)
    }

    pub fn short() -> Self {
        Self::Short(0)
    }

    pub fn int() -> Self {
        Self::Int(0)
    }

    pub fn long() -> Self {
        Self::Long(0)
    }

    pub fn char() -> Self {
        Self::Char(0)
    }

    pub fn float() -> Self {
        Self::Float(0.0)
    }

    pub fn double() -> Self {
        Self::Double(0.0)
    }

    pub fn boolean() -> Self {
        Self::Boolean(0)
    }

    pub fn reference() -> Self {
        Self::Reference(None)
    }
}

#[derive(Debug, Clone)]
pub struct Code {
    pub stack_depth: usize,
    pub local_variable_count: usize,
    // TODO exceptions
    // TODO attributes
    pub byte_code: Vec<OpCode>,
}

pub trait ClassInstance {
    fn as_any(&self) -> &dyn Any;
    fn class(&self) -> Rc<dyn Class>;
    fn instance_fields(&self) -> &[Rc<Field>];
}

pub struct BytecodeClassInstance {
    pub class: Rc<dyn Class>,
    pub instance_fields: Vec<Rc<Field>>,
}

impl fmt::Debug for dyn ClassInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "instance of Class '{}'", self.class().name())
    }
}

impl ClassInstance for BytecodeClassInstance {
    fn class(&self) -> Rc<dyn Class> {
        self.class.clone()
    }

    fn instance_fields(&self) -> &[Rc<Field>] {
        self.instance_fields.as_slice()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
