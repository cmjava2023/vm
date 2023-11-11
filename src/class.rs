pub mod builtin_classes;
pub mod bytecode_classes;

use core::fmt;
use std::rc::Rc;

use crate::executor::{Frame, OpCode, StackValue, Update};

pub enum Method {
    Bytecode(BytecodeMethod),
    // TODO pass execution frame (i.e. stack and local variables)
    // TODO return value?
    Rust(for<'a> fn(&'a Frame) -> Update),
}

pub trait Class {
    fn methods(&self) -> &[Method];
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

pub struct BytecodeClass {
    methods: Vec<Method>,
    static_fields: Vec<Rc<Field>>,
    instance_fields: Vec<String>,
    // TODO flags
    package: String,
    name: String,
    super_class: Option<Rc<dyn Class>>,
    // TODO how are interfaces represented?
    interfaces: Vec<Rc<dyn std::any::Any>>,
    // TODO attributes
}

pub struct Field {
    pub name: String,
    // TODO flags
    // TODO attributes
    // TODO data type
    pub value: FieldValue,
}

#[derive(Clone)]
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
    Reference(Option<Rc<ClassInstance>>),
}

impl Into<StackValue> for FieldValue {
    fn into(self) -> StackValue {
        match self {
            FieldValue::Byte(v) => StackValue::Byte(v),
            FieldValue::Short(v) => StackValue::Short(v),
            FieldValue::Int(v) => StackValue::Int(v),
            FieldValue::Long(v) => StackValue::Long(v),
            FieldValue::Char(v) => StackValue::Char(v),
            FieldValue::Float(v) => StackValue::Float(v),
            FieldValue::Double(v) => StackValue::Double(v),
            FieldValue::Boolean(v) => StackValue::Boolean(v),
            FieldValue::Reference(v) => StackValue::Reference(v),
        }
    }
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

pub struct BytecodeMethod {
    pub name: String,
    // TODO parameter
    // TODO return type
    // TODO flags
    // TODO attributes
    pub code: Code,
}

pub struct Code {
    pub stack_depth: u32,
    pub local_variable_count: u32,
    // TODO exceptions
    // TODO attributes
    pub byte_code: Vec<OpCode>,
}

pub struct ClassInstance {
    pub class: Rc<dyn Class>,
    pub instance_fields: Vec<Field>,
}

impl fmt::Debug for ClassInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "instance of Class '{}'", self.class.name())
    }
}
