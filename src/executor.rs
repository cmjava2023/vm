use std::rc::Rc;

use thiserror::Error;

use crate::{
    class::{
        ArgumentKind, Class, ClassInstance, Code, Field, FieldValue, Method,
        MethodCode, RustMethodReturn, SimpleArgumentKind,
    },
    classloader::cp_decoder::RuntimeCPEntry,
};

pub struct ExecutorFrame {
    frame: Frame,
    pc: ProgramCounter,
}

pub fn run(code: &Code) {
    let mut frame_stack: Vec<ExecutorFrame> = Vec::new();
    let mut current_frame: Frame = Frame {
        local_variables: LocalVariables::new(code.local_variable_count),
        operand_stack: FrameStack::new(code.stack_depth),
    };
    let mut current_pc: ProgramCounter =
        ProgramCounter::new(code.byte_code.clone());

    loop {
        match current_pc.current().0.execute(&mut current_frame) {
            Update::None => current_pc.next(1).unwrap(),
            Update::MethodCall(method) => match &method.code {
                MethodCode::Bytecode(c) => {
                    let mut new_frame = Frame {
                        local_variables: LocalVariables::new(
                            c.local_variable_count,
                        ),
                        operand_stack: FrameStack::new(c.stack_depth),
                    };
                    let pc = ProgramCounter::new(c.byte_code.clone());

                    prepare_parameters(
                        &mut current_frame,
                        &mut new_frame,
                        method.parameters.len(),
                        method.is_static,
                    );

                    frame_stack.push(ExecutorFrame {
                        frame: current_frame,
                        pc: current_pc,
                    });
                    current_frame = new_frame;
                    current_pc = pc;
                },
                MethodCode::Rust(code) => {
                    // Calculate number of local variable slots needed
                    // to pass the parameters to `method`,
                    // since for builtin-methods,
                    // there's no java compiler which determines ahead of time
                    // the amount of local variable slots needed
                    // to execute the method.
                    // Note that double/long values always occupy
                    // two slots of local variables.
                    let local_variable_count: usize = method
                        .parameters
                        .iter()
                        .map(|p| {
                            if p == &ArgumentKind::Simple(
                                SimpleArgumentKind::Long,
                            ) || p
                                == &ArgumentKind::Simple(
                                    SimpleArgumentKind::Double,
                                )
                            {
                                2
                            } else {
                                1
                            }
                        })
                        .sum();
                    let mut new_frame = Frame {
                        local_variables: LocalVariables::new(
                            // Non-Static methods receive "this"
                            // implicitly as additional parameter
                            (if method.is_static { 0 } else { 1 })
                                + local_variable_count,
                        ),
                        operand_stack: FrameStack::new(0),
                    };

                    prepare_parameters(
                        &mut current_frame,
                        &mut new_frame,
                        method.parameters.len(),
                        method.is_static,
                    );

                    match code(&mut new_frame) {
                        RustMethodReturn::Void => (),
                    }

                    current_pc.next(1).unwrap();
                },
            },
            Update::Return => {
                (current_frame, current_pc) = match frame_stack.pop() {
                    None => break,
                    Some(frame) => (frame.frame, frame.pc),
                };
                current_pc.next(1).unwrap();
            },
        }
    }
}

fn prepare_parameters(
    current_frame: &mut Frame,
    new_frame: &mut Frame,
    parameter_count: usize,
    is_static: bool,
) {
    // Non-Static methods receive "this"
    // implicitly as additional parameter
    let real_parameter_count =
        (if is_static { 0 } else { 1 }) + parameter_count;

    let mut parameters: Vec<VariableValueOrValue> = Vec::new();
    for _ in 0..real_parameter_count {
        parameters.insert(0, current_frame.operand_stack.pop().unwrap().into());
    }
    let mut variable_index = 0;
    for param in parameters.into_iter() {
        let size = param.size() as usize;
        new_frame.local_variables.set(variable_index, param);
        // long/double values occupy two slots
        // (the one passed to `set()` and the next one).
        // Account for this when calculating which index to use next:
        variable_index += size;
    }
}

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("NullPointer Exception")]
    NullPointer,
    #[error("Unexpected type '{actual}' (expected '{expected}')")]
    InvalidType {
        expected: &'static str,
        actual: &'static str,
    },
}

#[derive(Clone, Debug)]
pub enum OpCode {
    GetStatic(Rc<Field>),
    Ldc(Ldc),
    Return,
    InvokeVirtual(Rc<Method>),
    InvokeSpecial(RuntimeCPEntry), // Placeholder, to enable bytecode parsing
    Aload0,
    // value to push onto stack
    Bipush(u8),
    I2b,
    I2c,
    I2d,
    I2f,
    I2l,
    I2s,
    Iadd,
    Iand,
    // value to push onto stack
    Iconst(i32),
    Idiv,
    Iinc,
    // Iload_ are converted,
    // index into local variables
    Iload(usize),
    Imul,
    Ineg,
    Ior,
    Irem,
    Ishl,
    Ishr,
    // Istore_ are converted,
    // index into local variables
    Istore(usize),
    Isub,
    Iushr,
    Ixor,
    // Lstore_ are converted,
    // index into local variables
    Lstore(usize),
    // Fstore_ are converted,
    // index into local variables
    Fstore(usize),
    // Dstore_ are converted,
    // index into local variables
    Dstore(usize),
}

#[derive(Clone, Debug)]
pub enum Ldc {
    Int(i32),
    Float(f32),
    String(Rc<dyn ClassInstance>),
    Class(Rc<dyn Class>),
    Method(Rc<Method>),
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
            _ => todo!(),
        }
    }
}

pub enum Update {
    None,
    Return,
    MethodCall(Rc<Method>),
}

pub struct Frame {
    pub local_variables: LocalVariables,
    pub operand_stack: FrameStack,
}

#[derive(Clone)]
pub enum VariableValue {
    // Primitive Types
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
    Reference(Option<Rc<dyn ClassInstance>>),
}

#[derive(Debug)]
pub enum StackValue {
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

pub struct LocalVariables {
    local_variables: Vec<VariableValue>,
}

pub struct FrameStack {
    values: Vec<StackValue>,
}

pub struct ProgramCounter {
    current_op_code: usize,
    current_op_codes: Vec<OpCode>,
}

pub enum VariableValueOrValue {
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

#[derive(Error, Debug)]
pub enum ProgramCounterError {
    #[error("Index {requested_pos} out of {actual_len}")]
    OutOfBoundsError {
        actual_len: usize,
        requested_pos: usize,
    },
}

impl ProgramCounter {
    pub fn new(op_codes: Vec<OpCode>) -> ProgramCounter {
        assert!(
            !op_codes.is_empty(),
            "op_codes has to contain at least one op code"
        );
        ProgramCounter {
            current_op_code: 0,
            current_op_codes: op_codes,
        }
    }

    /// relative to current
    pub fn next(&mut self, offset: usize) -> Result<(), ProgramCounterError> {
        if self.current_op_codes.len() <= self.current_op_code + offset {
            return Err(ProgramCounterError::OutOfBoundsError {
                actual_len: self.current_op_codes.len(),
                requested_pos: self.current_op_code + offset,
            });
        }
        self.current_op_code += offset;
        Ok(())
    }

    // todo: fn previous() might be needed as offset as usize cannot be negative

    /// absolute
    pub fn set(&mut self, position: usize) -> Result<(), ProgramCounterError> {
        if self.current_op_codes.len() <= position {
            return Err(ProgramCounterError::OutOfBoundsError {
                actual_len: self.current_op_codes.len(),
                requested_pos: position,
            });
        }
        self.current_op_code = position;
        Ok(())
    }

    pub fn current(&self) -> (&OpCode, usize) {
        (
            (self
                .current_op_codes
                .get(self.current_op_code)
                .expect("current_op_code is never out of bounds")),
            self.current_op_code,
        )
    }
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
