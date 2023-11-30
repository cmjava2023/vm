pub mod access_flags;
pub mod builtin_classes;
pub mod bytecode_classes;

use core::fmt;
use std::{any::Any, rc::Rc};

use crate::executor::{frame_stack::StackValue, Frame, OpCode, RuntimeError};

#[derive(Debug, Clone, PartialEq)]
pub enum ArgumentKind {
    Simple(SimpleArgumentKind),
    Array {
        dimensions: usize,
        kind: SimpleArgumentKind,
    },
}
#[derive(Debug, Clone, PartialEq)]
pub enum SimpleArgumentKind {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Class(String),
    Short,
    Boolean,
}

#[derive(Debug, Clone)]
pub struct Method {
    pub code: MethodCode,
    pub name: String,
    pub parameters: Vec<ArgumentKind>,
    pub return_type: Option<ArgumentKind>,
    pub is_static: bool,
    // TODO flags
    // TODO attributes
}

#[derive(Debug, Clone)]
pub enum MethodCode {
    Bytecode(Code),
    Rust(for<'a> fn(&'a mut Frame) -> RustMethodReturn),
}

pub enum RustMethodReturn {
    Void,
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
            .find(|element| element.name == method_name)
            .cloned()
    }

    pub fn get_static_field(&self, field_name: &str) -> Option<Rc<Field>> {
        self.static_fields()
            .iter()
            .find(|element| element.name == field_name)
            .cloned()
    }
}

impl std::fmt::Debug for dyn Class {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Class '{}/{}'", self.package(), self.name())
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
    // UTF-16 encoded Unicode Code point in the Basic Multilingual Plane
    Char(u16),
    //    Floating-Point Types
    Float(f32),
    Double(f64),
    //    Other
    /// Encodes false as 0, true as 1.
    ///
    /// This is according to [the Java VM Spec](
    /// https://docs.oracle.com/javase/specs/jvms/se8/html/
    /// jvms-2.html#jvms-2.3.4
    /// )
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

impl fmt::Debug for dyn ClassInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "instance of Class '{}'", self.class().name())
    }
}

impl TryFrom<StackValue> for Rc<dyn ClassInstance> {
    type Error = RuntimeError;

    fn try_from(value: StackValue) -> Result<Self, Self::Error> {
        match value.try_into()? {
            Some(r) => Ok(r),
            _ => Err(RuntimeError::NullPointer),
        }
    }
}

impl TryFrom<StackValue> for Option<Rc<dyn ClassInstance>> {
    type Error = RuntimeError;

    fn try_from(value: StackValue) -> Result<Self, Self::Error> {
        match value {
            StackValue::Reference(r) => Ok(r),
            _ => Err(RuntimeError::InvalidType {
                expected: "reference",
                actual: value.type_name(),
            }),
        }
    }
}
