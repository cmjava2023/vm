use std::rc::Rc;

use crate::{
    class::{ClassInstance, FieldValue},
    executor::{local_variables::VariableValueOrValue, RuntimeError},
};

#[derive(Debug, Clone)]
pub enum StackValue {
    // Primitive Types
    //   Integral Types
    /// Covers int, byte, short, bool, char
    Int(i32),
    Long(i64),
    //    Floating-Point Types
    Float(f32),
    Double(f64),
    //    Other
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
            StackValue::Int(_) => "int",
            StackValue::Long(_) => "long",
            StackValue::Float(_) => "float",
            StackValue::Double(_) => "double",
            StackValue::ReturnAddress(_) => "return_address",
            StackValue::Reference(_) => "reference",
        }
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

    pub fn clear(&mut self) {
        self.values.clear()
    }
}

impl From<FieldValue> for StackValue {
    fn from(value: FieldValue) -> Self {
        match value {
            FieldValue::Byte(v) => StackValue::Int(v.into()),
            FieldValue::Short(v) => StackValue::Int(v.into()),
            FieldValue::Int(v) => StackValue::Int(v),
            FieldValue::Long(v) => StackValue::Long(v),
            FieldValue::Char(v) => StackValue::Int(v.into()),
            FieldValue::Float(v) => StackValue::Float(v),
            FieldValue::Double(v) => StackValue::Double(v),
            FieldValue::Boolean(v) => StackValue::Int(v.into()),
            FieldValue::Reference(v) => StackValue::Reference(v),
        }
    }
}

impl From<VariableValueOrValue> for StackValue {
    fn from(value: VariableValueOrValue) -> Self {
        match value {
            VariableValueOrValue::Int(i) => StackValue::Int(i),
            VariableValueOrValue::Long(l) => StackValue::Long(l),
            VariableValueOrValue::Float(f) => StackValue::Float(f),
            VariableValueOrValue::Double(d) => StackValue::Double(d),
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

impl TryFrom<StackValue> for i32 {
    type Error = RuntimeError;

    fn try_from(value: StackValue) -> Result<Self, Self::Error> {
        match value {
            StackValue::Int(i) => Ok(i),
            _ => Err(RuntimeError::InvalidType {
                expected: "int",
                actual: value.type_name(),
            }),
        }
    }
}

impl TryFrom<StackValue> for i64 {
    type Error = RuntimeError;

    fn try_from(value: StackValue) -> Result<Self, Self::Error> {
        match value {
            StackValue::Long(l) => Ok(l),
            _ => Err(RuntimeError::InvalidType {
                expected: "long",
                actual: value.type_name(),
            }),
        }
    }
}

impl TryFrom<StackValue> for f32 {
    type Error = RuntimeError;

    fn try_from(value: StackValue) -> Result<Self, Self::Error> {
        match value {
            StackValue::Float(f) => Ok(f),
            _ => Err(RuntimeError::InvalidType {
                expected: "float",
                actual: value.type_name(),
            }),
        }
    }
}

impl TryFrom<StackValue> for f64 {
    type Error = RuntimeError;

    fn try_from(value: StackValue) -> Result<Self, Self::Error> {
        match value {
            StackValue::Double(d) => Ok(d),
            _ => Err(RuntimeError::InvalidType {
                expected: "double",
                actual: value.type_name(),
            }),
        }
    }
}
