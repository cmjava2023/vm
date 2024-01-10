pub mod frame_stack;
pub mod local_variables;
pub mod op_code;
pub mod program_counter;

use std::rc::Rc;

use thiserror::Error;

use self::{frame_stack::StackValue, op_code::OffsetDirection};
pub use crate::executor::op_code::OpCode;
use crate::{
    class::{
        ArgumentKind, Class, ClassInstance, Code, Method, MethodCode,
        RustMethodReturn, SimpleArgumentKind,
    },
    executor::{
        frame_stack::FrameStack,
        local_variables::{LocalVariables, VariableValueOrValue},
        program_counter::ProgramCounter,
    },
    heap::Heap,
};

pub struct ExecutorFrame {
    frame: Frame,
    pc: ProgramCounter,
    class: Rc<dyn Class>,
}

pub fn run(code: &Code, heap: &mut Heap, initial_class: Rc<dyn Class>) {
    let mut frame_stack: Vec<ExecutorFrame> = Vec::new();
    let mut current_frame: Frame = Frame {
        local_variables: LocalVariables::new(code.local_variable_count),
        operand_stack: FrameStack::new(code.stack_depth),
    };
    let mut current_pc: ProgramCounter =
        ProgramCounter::new(code.byte_code.clone());
    let mut current_class = initial_class;

    'executor_loop: loop {
        match current_pc.current().0.execute(
            &mut current_frame,
            heap,
            &current_class,
        ) {
            Update::None => current_pc.next(1).unwrap(),
            Update::MethodCall {
                method,
                is_static,
                defining_class,
            } => match &method.code {
                MethodCode::Bytecode(c) => {
                    let mut new_frame = Frame {
                        local_variables: LocalVariables::new(
                            c.local_variable_count,
                        ),
                        operand_stack: FrameStack::new(c.stack_depth),
                    };
                    let pc = ProgramCounter::new(c.byte_code.clone());

                    assert_eq!(
                        is_static, method.is_static,
                        "method metadata and InvokeXXX agree"
                    );

                    prepare_parameters(
                        &mut current_frame,
                        &mut new_frame,
                        method.parameters.len(),
                        method.is_static,
                    );

                    frame_stack.push(ExecutorFrame {
                        frame: current_frame,
                        pc: current_pc,
                        class: current_class,
                    });
                    current_frame = new_frame;
                    current_pc = pc;
                    current_class = defining_class;
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
                        RustMethodReturn::Value(value) => current_frame
                            .operand_stack
                            .push(value.into())
                            .unwrap(),
                    }

                    current_pc.next(1).unwrap();
                },
            },
            Update::Return(value) => {
                (current_frame, current_pc) = match frame_stack.pop() {
                    None => break,
                    Some(frame) => (frame.frame, frame.pc),
                };
                match value {
                    ReturnValue::Int(i) => current_frame
                        .operand_stack
                        .push(StackValue::Int(i))
                        .unwrap(),
                    ReturnValue::Long(l) => current_frame
                        .operand_stack
                        .push(StackValue::Long(l))
                        .unwrap(),
                    ReturnValue::Float(f) => current_frame
                        .operand_stack
                        .push(StackValue::Float(f))
                        .unwrap(),
                    ReturnValue::Double(d) => current_frame
                        .operand_stack
                        .push(StackValue::Double(d))
                        .unwrap(),
                    ReturnValue::Reference(a) => current_frame
                        .operand_stack
                        .push(StackValue::Reference(a))
                        .unwrap(),
                    ReturnValue::Void => (),
                }
                current_pc.next(1).unwrap();
            },
            Update::GoTo(offset, direction) => match direction {
                OffsetDirection::Forward => current_pc.next(offset).unwrap(),
                OffsetDirection::Backward => {
                    current_pc.previous(offset).unwrap()
                },
            },
            Update::Exception(e) => {
                // allow the while-loop to handle the current method
                // without special casing the first iteration
                frame_stack.push(ExecutorFrame {
                    frame: current_frame,
                    pc: current_pc,
                    class: current_class,
                });
                // search the call stack for an exception handler
                // matching the current exception
                while let Some(ExecutorFrame { frame, pc, class }) =
                    frame_stack.pop()
                {
                    current_frame = frame;
                    current_pc = pc;
                    current_class = class;
                    // check all exception handler of the current method
                    // expectation: the order is 'correct', i.e.
                    // the first matching handler is the one that's supposed
                    // to handle the current exception
                    // (i.e. this code does NOT search the most specific
                    // matching handler)
                    for exception in code.exception_table.iter() {
                        // is the exception handler active in the region
                        // that is currently executed?
                        if exception.active.contains(&current_pc.current().1) {
                            // does the exception handler handle the
                            // class of the thrown exception?
                            // TODO: inheritance
                            // (i.e. is e.class().class_identifier()
                            //  could also be a subclass of identifer)
                            let catch_type_match = match &exception.catch_type {
                                // exception handler handles all exceptions
                                None => true,
                                Some(identifier) => {
                                    e.class().class_identifier() == identifier
                                },
                            };
                            if catch_type_match {
                                current_frame.operand_stack.clear();
                                current_frame
                                    .operand_stack
                                    .push(StackValue::Reference(Some(
                                        e.clone(),
                                    )))
                                    .unwrap();
                                current_pc
                                    .set(exception.handler_position)
                                    .unwrap();
                                continue 'executor_loop;
                            }
                        }
                    }
                }
                // no handler has been found: terminate
                panic!("Uncaught exception: {:?}", e);
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
    Return(ReturnValue),
    GoTo(usize, OffsetDirection),
    MethodCall {
        method: Rc<Method>,
        is_static: bool,
        defining_class: Rc<dyn Class>,
    },
    Exception(Rc<dyn ClassInstance>),
}

pub enum ReturnValue {
    // Primitive Types
    //   Integral Types
    Int(i32),
    Long(i64),
    //    Floating-Point Types
    Float(f32),
    Double(f64),
    //    Other
    Void,
    // Reference Types
    // TODO different reference types (array, interface)
    Reference(Option<Rc<dyn ClassInstance>>),
}

#[derive(Debug)]
pub struct Frame {
    pub local_variables: LocalVariables,
    pub operand_stack: FrameStack,
}
