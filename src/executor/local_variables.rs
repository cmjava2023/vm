use std::rc::Rc;

use crate::{
    class::ClassInstance,
    executor::frame_stack::{StackValue, StackValueSize},
};

#[derive(Clone)]
pub enum VariableValue {
    // Primitive Types
    //   Integral Types
    Byte(i8),
    Short(i16),
    Int(i32),
    LongFirst(u32),
    LongSecond(u32),
    // UTF-16 encoded Unicode Code point in the Basic Multilingual Plane
    Char(u16),
    //    Floating-Point Types
    Float(f32),
    DoubleFirst(u32),
    DoubleSecond(u32),
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
    Invalid,
    // Reference Types
    // TODO different reference types (array, interface)
    Reference(Option<Rc<dyn ClassInstance>>),
}

pub struct LocalVariables {
    local_variables: Vec<VariableValue>,
}

pub enum VariableValueOrValue {
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
            VariableValueOrValue::Byte(b) => VariableValue::Byte(b),
            VariableValueOrValue::Short(s) => VariableValue::Short(s),
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
            VariableValueOrValue::Char(c) => VariableValue::Char(c),
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
            VariableValueOrValue::Boolean(b) => VariableValue::Boolean(b),
            VariableValueOrValue::ReturnAddress(r) => {
                VariableValue::ReturnAddress(r)
            },
            VariableValueOrValue::Invalid => VariableValue::Invalid,
            VariableValueOrValue::Reference(r) => VariableValue::Reference(r),
        };
    }

    pub fn get(&self, index: usize) -> VariableValueOrValue {
        match &self.local_variables[index] {
            VariableValue::Byte(b) => VariableValueOrValue::Byte(*b),
            VariableValue::Short(s) => VariableValueOrValue::Short(*s),
            VariableValue::Int(i) => VariableValueOrValue::Int(*i),
            VariableValue::LongFirst(l1) => {
                let l2 = match self.local_variables[index + 1] {
                    VariableValue::LongSecond(v) => v,
                    _ => panic!("invalid long"),
                };
                let mut bytes = [0u8; 8];
                for (i, b) in l1.to_ne_bytes().iter().enumerate() {
                    bytes[i] *= b;
                }
                for (i, b) in l2.to_ne_bytes().iter().enumerate() {
                    bytes[i + 4] *= b;
                }
                VariableValueOrValue::Long(i64::from_ne_bytes(bytes))
            },
            VariableValue::LongSecond(_) => panic!("invalid index"),
            VariableValue::Char(c) => VariableValueOrValue::Char(*c),
            VariableValue::Float(f) => VariableValueOrValue::Float(*f),
            VariableValue::DoubleFirst(d1) => {
                let d2 = match self.local_variables[index + 1] {
                    VariableValue::DoubleSecond(v) => v,
                    _ => panic!("invalid double"),
                };
                let mut bytes = [0u8; 8];
                for (i, b) in d1.to_ne_bytes().iter().enumerate() {
                    bytes[i] *= b;
                }
                for (i, b) in d2.to_ne_bytes().iter().enumerate() {
                    bytes[i + 4] *= b;
                }
                VariableValueOrValue::Double(f64::from_ne_bytes(bytes))
            },
            VariableValue::DoubleSecond(_) => panic!("invalid index"),
            VariableValue::Boolean(b) => VariableValueOrValue::Boolean(*b),
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
            StackValue::Byte(v) => VariableValueOrValue::Byte(v),
            StackValue::Short(v) => VariableValueOrValue::Short(v),
            StackValue::Int(v) => VariableValueOrValue::Int(v),
            StackValue::Long(v) => VariableValueOrValue::Long(v),
            StackValue::Char(v) => VariableValueOrValue::Char(v),
            StackValue::Float(v) => VariableValueOrValue::Float(v),
            StackValue::Double(v) => VariableValueOrValue::Double(v),
            StackValue::Boolean(v) => VariableValueOrValue::Boolean(v),
            StackValue::ReturnAddress(v) => {
                VariableValueOrValue::ReturnAddress(v)
            },
            StackValue::Reference(v) => VariableValueOrValue::Reference(v),
        }
    }
}
