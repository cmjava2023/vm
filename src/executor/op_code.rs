use std::rc::Rc;

use crate::{
    class::{Class, ClassInstance, Field, Method},
    classloader::cp_decoder::RuntimeCPEntry,
    executor::{frame_stack::StackValue, Frame, Update},
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
    Bipush(u8),
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
            _ => todo!("Missing OpCode implementation for: {:?}", self),
        }
    }
}
