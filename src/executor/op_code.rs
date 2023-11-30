use std::{ops::Neg, rc::Rc};

use crate::{
    class::{Class, ClassInstance, Field, Method},
    classloader::cp_decoder::RuntimeCPEntry,
    executor::{
        frame_stack::StackValue, local_variables::VariableValueOrValue, Frame,
        Update,
    },
};

#[derive(Clone, Debug)]
pub enum Ldc {
    Int(i32),
    Float(f32),
    String(Rc<dyn ClassInstance>),
    Class(Rc<dyn Class>),
    Method(Rc<Method>),
}

#[derive(Clone, Debug)]
pub enum OpCode {
    GetStatic(Rc<Field>),
    Ldc(Ldc),
    Return,
    InvokeVirtual(Rc<Method>),
    InvokeSpecial(RuntimeCPEntry), // Placeholder, to enable bytecode parsing
    /// Load reference from `index` in local variable array to stack.
    Aload(usize),
    /// Value to push onto stack
    Bipush(i32),
    I2b,
    I2c,
    I2d,
    I2f,
    I2l,
    I2s,
    Iadd,
    Iand,
    /// Value to push onto stack
    Iconst(i32),
    Idiv,
    Iinc {
        index: usize,
        constant: i32,
    },
    /// Iload_ are converted,
    /// index into local variables
    Iload(usize),
    Fload(usize),
    Dload(usize),
    Lload(usize),
    Imul,
    Ineg,
    Ior,
    Irem,
    Ishl,
    Ishr,
    /// Istore_ are converted,
    /// index into local variables
    Istore(usize),
    Isub,
    Iushr,
    Ixor,
    /// Lstore_ are converted,
    /// index into local variables
    Lstore(usize),
    /// Fstore_ are converted,
    /// index into local variables
    Fstore(usize),
    /// Dstore_ are converted,
    /// index into local variables
    Dstore(usize),
}

impl OpCode {
    pub fn execute(&self, frame: &mut Frame) -> Update {
        match self {
            Self::Ldc(Ldc::Int(i)) => {
                frame.operand_stack.push(StackValue::Int(*i)).unwrap();
                Update::None
            },
            Self::Ldc(Ldc::Float(f)) => {
                frame.operand_stack.push(StackValue::Float(*f)).unwrap();
                Update::None
            },
            Self::Ldc(Ldc::String(s)) => {
                frame
                    .operand_stack
                    .push(StackValue::Reference(Some(s.clone())))
                    .unwrap();
                Update::None
            },
            Self::InvokeVirtual(method) => Update::MethodCall(method.clone()),
            Self::GetStatic(field) => {
                frame
                    .operand_stack
                    .push(field.value.clone().into())
                    .unwrap();
                Update::None
            },
            Self::Return => Update::Return,
            Self::Aload(index) => {
                frame
                    .operand_stack
                    .push(frame.local_variables.get(*index).into())
                    .unwrap();
                Update::None
            },
            Self::Bipush(v) => {
                frame.operand_stack.push(StackValue::Int(*v)).unwrap();
                Update::None
            },
            // note: split this into multiple cases,
            // in case the types are supposed to be verified
            Self::Dstore(index)
            | Self::Fstore(index)
            | Self::Istore(index)
            | Self::Lstore(index) => {
                frame
                    .local_variables
                    .set(*index, frame.operand_stack.pop().unwrap().into());
                Update::None
            },
            // note: split this into multiple cases,
            // in case the types are supposed to be verifiedcccccclh
            Self::Dload(index)
            | Self::Fload(index)
            | Self::Iload(index)
            | Self::Lload(index) => {
                frame
                    .operand_stack
                    .push(frame.local_variables.get(*index).into())
                    .unwrap();
                Update::None
            },
            Self::I2b => {
                let int = frame.operand_stack.pop().expect("stack has value");
                let int = match int {
                    StackValue::Int(i) => i,
                    _ => panic!("expect int stack value, got '{:?}'", int),
                };
                let byte: i8 = int as i8;
                frame.operand_stack.push(StackValue::Byte(byte)).unwrap();
                Update::None
            },
            Self::I2c => {
                let int = frame.operand_stack.pop().expect("stack has value");
                let int = match int {
                    StackValue::Int(i) => i,
                    _ => panic!("expect int stack value, got '{:?}'", int),
                };
                let c: u16 = int as u16;
                frame.operand_stack.push(StackValue::Char(c)).unwrap();
                Update::None
            },
            Self::I2d => {
                let int = frame.operand_stack.pop().expect("stack has value");
                let int = match int {
                    StackValue::Int(i) => i,
                    _ => panic!("expect int stack value, got '{:?}'", int),
                };
                let double: f64 = int.into();
                frame
                    .operand_stack
                    .push(StackValue::Double(double))
                    .unwrap();
                Update::None
            },
            Self::I2f => {
                let int = frame.operand_stack.pop().expect("stack has value");
                let int = match int {
                    StackValue::Int(i) => i,
                    _ => panic!("expect int stack value, got '{:?}'", int),
                };
                let float: f32 = int as f32;
                frame.operand_stack.push(StackValue::Float(float)).unwrap();
                Update::None
            },
            Self::I2l => {
                let int = frame.operand_stack.pop().expect("stack has value");
                let int = match int {
                    StackValue::Int(i) => i,
                    _ => panic!("expect int stack value, got '{:?}'", int),
                };
                let long: i64 = int.into();
                frame.operand_stack.push(StackValue::Long(long)).unwrap();
                Update::None
            },
            Self::I2s => {
                let int = frame.operand_stack.pop().expect("stack has value");
                let int = match int {
                    StackValue::Int(i) => i,
                    _ => panic!("expect int stack value, got '{:?}'", int),
                };
                let short: i16 = int as i16;
                frame.operand_stack.push(StackValue::Short(short)).unwrap();
                Update::None
            },
            Self::Iadd => {
                let op2 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();
                let op1 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();

                let result: i32 = op1.wrapping_add(op2);

                // result is always int,
                // for byte/short/etc it will be explicitly casted
                // by the following bytecode op
                // (generated by compiler)
                frame.operand_stack.push(StackValue::Int(result)).unwrap();

                Update::None
            },
            Self::Iand => {
                let op2 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();
                let op1 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();

                let result: i32 = op1 & op2;

                // result is always int,
                // for byte/short/etc it will be explicitly casted
                // by the following bytecode op
                // (generated by compiler)
                frame.operand_stack.push(StackValue::Int(result)).unwrap();

                Update::None
            },
            Self::Iconst(val) => {
                frame.operand_stack.push(StackValue::Int(*val)).unwrap();
                Update::None
            },
            Self::Idiv => {
                let op2 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();
                let op1 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();

                // TODO(FW): this panics for division by 0
                let result: i32 = op1.wrapping_div(op2);

                // result is always int,
                // for byte/short/etc it will be explicitly casted
                // by the following bytecode op
                // (generated by compiler)
                frame.operand_stack.push(StackValue::Int(result)).unwrap();

                Update::None
            },
            Self::Iinc { index, constant } => {
                let op1 = frame.local_variables.get(*index);
                let op1 = match op1 {
                    VariableValueOrValue::Int(i) => i,
                    _ => panic!("expect int at {} got '{:?}'", index, op1),
                };

                let result: i32 = op1 + constant;

                frame
                    .local_variables
                    .set(*index, VariableValueOrValue::Int(result));

                Update::None
            },
            Self::Imul => {
                let op2 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();
                let op1 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();

                let result: i32 = op1.wrapping_mul(op2);

                frame.operand_stack.push(StackValue::Int(result)).unwrap();

                Update::None
            },
            Self::Ineg => {
                let op1 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();

                let result: i32 = op1.neg();

                frame.operand_stack.push(StackValue::Int(result)).unwrap();

                Update::None
            },
            Self::Ior => {
                let op2 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();
                let op1 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();

                let result: i32 = op1 | op2;

                frame.operand_stack.push(StackValue::Int(result)).unwrap();

                Update::None
            },
            Self::Irem => {
                let op2 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();
                let op1 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();

                let result: i32 = op1.wrapping_rem(op2);

                frame.operand_stack.push(StackValue::Int(result)).unwrap();

                Update::None
            },
            Self::Ishl => {
                let op2 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();
                let op1 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();

                let result: i32 = op1 << (op2 & 0x1f);

                frame.operand_stack.push(StackValue::Int(result)).unwrap();

                Update::None
            },
            Self::Ishr => {
                let op2 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();
                let op1 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();

                let result: i32 = op1 >> (op2 & 0x1f);

                frame.operand_stack.push(StackValue::Int(result)).unwrap();

                Update::None
            },
            Self::Iushr => {
                let op2 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();
                let op1 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();

                // perform shift with zero extension
                // by casting to an unsigned value before the shift
                let result: i32 = ((op1 as u32) >> (op2 & 0x1f)) as i32;

                frame.operand_stack.push(StackValue::Int(result)).unwrap();

                Update::None
            },
            Self::Isub => {
                let op2 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();
                let op1 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();

                let result: i32 = op1.wrapping_sub(op2);

                frame.operand_stack.push(StackValue::Int(result)).unwrap();

                Update::None
            },
            Self::Ixor => {
                let op2 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();
                let op1 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();

                let result: i32 = op1 ^ op2;

                frame.operand_stack.push(StackValue::Int(result)).unwrap();

                Update::None
            },
            _ => todo!("Missing OpCode implementation for: {:?}", self),
        }
    }
}
