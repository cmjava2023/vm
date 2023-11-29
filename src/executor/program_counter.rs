use thiserror::Error;

use crate::executor::OpCode;

pub struct ProgramCounter {
    current_op_code: usize,
    current_op_codes: Vec<OpCode>,
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
