use std::rc::Rc;

use crate::{
    class::ClassInstance,
    executor::{
        frame_stack::{StackValue, StackValueSize},
        RuntimeError,
    },
};

#[derive(Debug, Clone)]
pub enum VariableValue {
    // Primitive Types
    //   Integral Types
    /// covers int, byte, short, char, boolean
    Int(i32),
    LongFirst(u32),
    LongSecond(u32),
    //    Floating-Point Types
    Float(f32),
    DoubleFirst(u32),
    DoubleSecond(u32),
    //    Other
    /// used for in-method jumps,
    /// therefor an offset works.
    /// Encodes an absolute position.
    ReturnAddress(usize),
    Invalid,
    // Reference Types
    // TODO different reference types (array, interface)
    Reference(Option<Rc<dyn ClassInstance>>),
}

#[derive(Debug)]
pub struct LocalVariables {
    local_variables: Vec<VariableValue>,
}

#[derive(Debug)]
pub enum VariableValueOrValue {
    // Primitive Types
    //   Integral Types
    /// Covers int, short, byte, char, boolean
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
    Invalid,
    // Reference Types
    // TODO different reference types (array, interface)
    Reference(Option<Rc<dyn ClassInstance>>),
}

impl VariableValueOrValue {
    /// Return how many slots this value occupies.
    ///
    /// This is needed, since long/doubles are treated specially
    /// by a Java VM.
    pub fn size(&self) -> StackValueSize {
        if matches!(self, VariableValueOrValue::Long(_))
            || matches!(self, VariableValueOrValue::Double(_))
        {
            StackValueSize::Two
        } else {
            StackValueSize::One
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            VariableValueOrValue::Int(_) => "int",
            VariableValueOrValue::Long(_) => "long",
            VariableValueOrValue::Float(_) => "float",
            VariableValueOrValue::Double(_) => "double",
            VariableValueOrValue::ReturnAddress(_) => "return_address",
            VariableValueOrValue::Reference(_) => "reference",
            VariableValueOrValue::Invalid => "invalid!",
        }
    }
}

impl LocalVariables {
    pub fn new(length: usize) -> LocalVariables {
        LocalVariables {
            local_variables: vec![VariableValue::Invalid; length],
        }
    }

    pub fn set(&mut self, index: usize, value: VariableValueOrValue) {
        // invalid the previous slot when overwriting the second
        // part of a long/double, to make sure it cannot be interpreted as such
        if index > 0
            && (matches!(
                self.local_variables[index],
                VariableValue::LongSecond(_)
            ) || matches!(
                self.local_variables[index],
                VariableValue::DoubleSecond(_)
            ))
        {
            self.local_variables[index - 1] = VariableValue::Invalid;
        }

        self.local_variables[index] = match value {
            VariableValueOrValue::Int(i) => VariableValue::Int(i),
            VariableValueOrValue::Long(l) => {
                let bytes = l.to_ne_bytes();
                let l1bytes: &[u8; 4] = bytes[0..4].try_into().unwrap();
                let l2bytes: &[u8; 4] = bytes[4..8].try_into().unwrap();
                let l1 = u32::from_ne_bytes(*l1bytes);
                let l2 = u32::from_ne_bytes(*l2bytes);
                self.local_variables[index + 1] = VariableValue::LongSecond(l2);
                VariableValue::LongFirst(l1)
            },
            VariableValueOrValue::Float(f) => VariableValue::Float(f),
            VariableValueOrValue::Double(d) => {
                let bytes = d.to_ne_bytes();
                let l1bytes: &[u8; 4] = bytes[0..4].try_into().unwrap();
                let l2bytes: &[u8; 4] = bytes[4..8].try_into().unwrap();
                let l1 = u32::from_ne_bytes(*l1bytes);
                let l2 = u32::from_ne_bytes(*l2bytes);
                self.local_variables[index + 1] =
                    VariableValue::DoubleSecond(l2);
                VariableValue::DoubleFirst(l1)
            },
            VariableValueOrValue::ReturnAddress(r) => {
                VariableValue::ReturnAddress(r)
            },
            VariableValueOrValue::Invalid => VariableValue::Invalid,
            VariableValueOrValue::Reference(r) => VariableValue::Reference(r),
        };
    }

    pub fn get(&self, index: usize) -> VariableValueOrValue {
        match &self.local_variables[index] {
            VariableValue::Int(i) => VariableValueOrValue::Int(*i),
            VariableValue::LongFirst(l1) => {
                let l2 = match self.local_variables[index + 1] {
                    VariableValue::LongSecond(v) => v,
                    _ => panic!("invalid long"),
                };
                let mut bytes = [0u8; 8];
                for (i, b) in l1.to_ne_bytes().iter().enumerate() {
                    bytes[i] = *b;
                }
                for (i, b) in l2.to_ne_bytes().iter().enumerate() {
                    bytes[i + 4] = *b;
                }
                VariableValueOrValue::Long(i64::from_ne_bytes(bytes))
            },
            VariableValue::LongSecond(_) => panic!("invalid index"),
            VariableValue::Float(f) => VariableValueOrValue::Float(*f),
            VariableValue::DoubleFirst(d1) => {
                let d2 = match self.local_variables[index + 1] {
                    VariableValue::DoubleSecond(v) => v,
                    _ => panic!("invalid double"),
                };
                let mut bytes = [0u8; 8];
                for (i, b) in d1.to_ne_bytes().iter().enumerate() {
                    bytes[i] = *b;
                }
                for (i, b) in d2.to_ne_bytes().iter().enumerate() {
                    bytes[i + 4] = *b;
                }
                VariableValueOrValue::Double(f64::from_ne_bytes(bytes))
            },
            VariableValue::DoubleSecond(_) => panic!("invalid index"),
            VariableValue::ReturnAddress(r) => {
                VariableValueOrValue::ReturnAddress(*r)
            },
            VariableValue::Invalid => VariableValueOrValue::Invalid,
            VariableValue::Reference(r) => {
                VariableValueOrValue::Reference(r.clone())
            },
        }
    }
}

impl From<StackValue> for VariableValueOrValue {
    fn from(value: StackValue) -> Self {
        match value {
            StackValue::Int(v) => VariableValueOrValue::Int(v),
            StackValue::Long(v) => VariableValueOrValue::Long(v),
            StackValue::Float(v) => VariableValueOrValue::Float(v),
            StackValue::Double(v) => VariableValueOrValue::Double(v),
            StackValue::ReturnAddress(v) => {
                VariableValueOrValue::ReturnAddress(v)
            },
            StackValue::Reference(v) => VariableValueOrValue::Reference(v),
        }
    }
}

impl TryFrom<VariableValueOrValue> for i32 {
    type Error = RuntimeError;

    fn try_from(value: VariableValueOrValue) -> Result<Self, Self::Error> {
        match value {
            VariableValueOrValue::Int(i) => Ok(i),
            _ => Err(RuntimeError::InvalidType {
                expected: "int",
                actual: value.type_name(),
            }),
        }
    }
}

impl TryFrom<VariableValueOrValue> for i64 {
    type Error = RuntimeError;

    fn try_from(value: VariableValueOrValue) -> Result<Self, Self::Error> {
        match value {
            VariableValueOrValue::Long(l) => Ok(l),
            _ => Err(RuntimeError::InvalidType {
                expected: "long",
                actual: value.type_name(),
            }),
        }
    }
}

impl TryFrom<VariableValueOrValue> for f32 {
    type Error = RuntimeError;

    fn try_from(value: VariableValueOrValue) -> Result<Self, Self::Error> {
        match value {
            VariableValueOrValue::Float(f) => Ok(f),
            _ => Err(RuntimeError::InvalidType {
                expected: "float",
                actual: value.type_name(),
            }),
        }
    }
}

impl TryFrom<VariableValueOrValue> for f64 {
    type Error = RuntimeError;

    fn try_from(value: VariableValueOrValue) -> Result<Self, Self::Error> {
        match value {
            VariableValueOrValue::Double(d) => Ok(d),
            _ => Err(RuntimeError::InvalidType {
                expected: "double",
                actual: value.type_name(),
            }),
        }
    }
}
