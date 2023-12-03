pub mod frame_stack;
pub mod local_variables;
pub mod op_code;
pub mod program_counter;

use std::rc::Rc;

use thiserror::Error;

pub use crate::executor::op_code::OpCode;
use crate::{
    class::{
        ArgumentKind, Code, Method, MethodCode, RustMethodReturn,
        SimpleArgumentKind,
    },
    executor::{
        frame_stack::FrameStack,
        local_variables::{LocalVariables, VariableValueOrValue},
        program_counter::ProgramCounter,
    },
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
    #[error(
        "array index out of bounds: \
the len is {length} but the index is {index}"
    )]
    ArrayIndexOutOfBounds { length: usize, index: usize },
    #[error("Unexpected type '{actual}' (expected '{expected}')")]
    InvalidType {
        expected: &'static str,
        actual: &'static str,
    },
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
