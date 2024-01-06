use std::rc::Rc;

use crate::{
    class::{ClassInstance, FieldValue},
    executor::{local_variables::VariableValueOrValue, RuntimeError},
};

#[derive(Debug, Clone)]
pub enum StackValue {
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
    /// used for in-method jumps,
    /// therefor an offset works.
    /// Encodes an absolute position.
    ReturnAddress(usize),
    // Reference Types
    // TODO different reference types (array, interface)
    Reference(Option<Rc<dyn ClassInstance>>),
}

#[repr(usize)]
pub enum StackValueSize {
    One = 1,
    Two = 2,
}

impl StackValue {
    pub fn size(&self) -> StackValueSize {
        if matches!(self, StackValue::Long(_))
            || matches!(self, StackValue::Double(_))
        {
            StackValueSize::Two
        } else {
            StackValueSize::One
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            StackValue::Byte(_) => "byte",
            StackValue::Short(_) => "short",
            StackValue::Int(_) => "int",
            StackValue::Long(_) => "long",
            StackValue::Char(_) => "char",
            StackValue::Float(_) => "float",
            StackValue::Double(_) => "double",
            StackValue::Boolean(_) => "boolean",
            StackValue::ReturnAddress(_) => "return_address",
            StackValue::Reference(_) => "reference",
        }
    }

    pub fn as_computation_int(&self) -> Result<i32, RuntimeError> {
        // int computional types according to
        // https://docs.oracle.com/javase/specs/jvms/se8/html
        // /jvms-2.html#jvms-2.11.1-320
        Ok(match *self {
            StackValue::Boolean(b) => b.into(),
            StackValue::Byte(b) => b.into(),
            StackValue::Char(c) => c.into(),
            StackValue::Short(s) => s.into(),
            StackValue::Int(i) => i,
            _ => Err(RuntimeError::InvalidType {
                expected: "int (computational type",
                actual: self.type_name(),
            })?,
        })
    }
}

#[derive(Debug)]
pub struct FrameStack {
    values: Vec<StackValue>,
}

impl FrameStack {
    pub fn new(max_depth: usize) -> FrameStack {
        FrameStack {
            values: Vec::with_capacity(max_depth),
        }
    }

    pub fn depth(&self) -> usize {
        self.values.iter().map(|v| v.size() as usize).sum()
    }

    pub fn push(&mut self, value: StackValue) -> Result<(), StackValue> {
        if self.depth() + (value.size() as usize) > self.values.capacity() {
            return Err(value);
        }

        self.values.push(value);
        Ok(())
    }

    pub fn pop(&mut self) -> Option<StackValue> {
        self.values.pop()
    }
}

impl From<FieldValue> for StackValue {
    fn from(value: FieldValue) -> Self {
        match value {
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

impl From<VariableValueOrValue> for StackValue {
    fn from(value: VariableValueOrValue) -> Self {
        match value {
            VariableValueOrValue::Byte(b) => StackValue::Byte(b),
            VariableValueOrValue::Short(s) => StackValue::Short(s),
            VariableValueOrValue::Int(i) => StackValue::Int(i),
            VariableValueOrValue::Long(l) => StackValue::Long(l),
            VariableValueOrValue::Char(c) => StackValue::Char(c),
            VariableValueOrValue::Float(f) => StackValue::Float(f),
            VariableValueOrValue::Double(d) => StackValue::Double(d),
            VariableValueOrValue::Boolean(b) => StackValue::Boolean(b),
            VariableValueOrValue::Reference(r) => StackValue::Reference(r),
            VariableValueOrValue::ReturnAddress(a) => {
                StackValue::ReturnAddress(a)
            },
            VariableValueOrValue::Invalid => {
                panic!("cannot convert invalid local variable value")
            },
        }
    }
}
