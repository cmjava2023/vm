use std::rc::Rc;

use crate::class::ClassInstance;

pub enum OpCode {}

pub enum Update {
    None,
}

pub struct Frame {
    pub local_variables: LocalVariables,
    pub operand_stack: FrameStack,
}

#[derive(Clone)]
pub enum VariableValue {
    // Primitve Types
    //   Integral Types
    Byte(i8),
    Short(i16),
    Int(i32),
    LongFirst(u32),
    LongSecond(u32),
    Char(u16),
    //    Floating-Point Types
    Float(f32),
    DoubleFirst(u32),
    DoubleSecond(u32),
    //    Other
    Boolean(u8),
    /// used for in-method jumps,
    /// therefor an offset works.
    /// Encodes an absolute position.
    ReturnAddress(usize),
    Invalid,
    // Reference Types
    // TODO different reference types (array, interface)
    Reference(Option<Rc<ClassInstance>>),
}

pub enum StackValue {
    // Primitve Types
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
    /// used for in-method jumps,
    /// therefor an offset works.
    /// Encodes an absolute position.
    ReturnAddress(usize),
    // Reference Types
    // TODO different reference types (array, interface)
    Reference(Option<Rc<ClassInstance>>),
}

pub struct LocalVariables {
    local_variables: Vec<VariableValue>,
}

pub struct FrameStack {
    values: Vec<StackValue>,
}

pub enum VariableValueOrValue {
    // Primitve Types
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
    /// used for in-method jumps,
    /// therefor an offset works.
    /// Encodes an absolute position.
    ReturnAddress(usize),
    Invalid,
    // Reference Types
    // TODO different reference types (array, interface)
    Reference(Option<Rc<ClassInstance>>),
}

impl LocalVariables {
    pub fn new(length: usize) -> LocalVariables {
        LocalVariables {
            local_variables: vec![VariableValue::Invalid; length],
        }
    }

    pub fn set(&mut self, index: usize, value: VariableValueOrValue) {
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

impl FrameStack {
    pub fn new(max_depth: usize) -> FrameStack {
        FrameStack {
            values: Vec::with_capacity(max_depth),
        }
    }

    pub fn depth(&self) -> usize {
        self.values
            .iter()
            .map(|v| {
                if matches!(v, StackValue::Long(_))
                    || matches!(v, StackValue::Double(_))
                {
                    2
                } else {
                    1
                }
            })
            .sum()
    }

    pub fn push(&mut self, value: StackValue) -> Result<(), StackValue> {
        if self.depth() >= self.values.capacity() {
            return Err(value);
        }

        self.values.push(value);
        Ok(())
    }

    pub fn pop(&mut self) -> Option<StackValue> {
        self.values.pop()
    }
}
