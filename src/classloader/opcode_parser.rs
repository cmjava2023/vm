use nom::{
    number::complete::{be_u16, be_u8},
    IResult,
};

use super::cp_decoder::RuntimeCPEntry;
use crate::{
    classloader::{
        cp_decoder::{remove_cp_offset},
        ClassFile,
    },
    executor::OpCode,
};

fn parse_getstatic<'a>(
    current_content: &'a [u8],
    _class_file: &ClassFile,
    runtime_cp: &[RuntimeCPEntry],
) -> IResult<&'a [u8], OpCode> {
    let (current_content, cp_ref) = be_u16(current_content)?;
    let cp_entry = &runtime_cp[remove_cp_offset(cp_ref as usize)];
    Ok((current_content, OpCode::GetStatic(cp_entry.clone())))
}

fn parse_ldc<'a>(
    current_content: &'a [u8],
    _class_file: &ClassFile,
    runtime_cp: &[RuntimeCPEntry],
) -> IResult<&'a [u8], OpCode> {
    let (current_content, cp_ref) = be_u8(current_content)?;
    let cp_entry = &runtime_cp[remove_cp_offset(cp_ref as usize)];
    Ok((current_content, OpCode::Ldc(cp_entry.clone())))
}

fn parse_invokevirtual<'a>(
    current_content: &'a [u8],
    _class_file: &ClassFile,
    runtime_cp: &[RuntimeCPEntry],
) -> IResult<&'a [u8], OpCode> {
    let (current_content, cp_ref) = be_u16(current_content)?;
    let cp_entry = &runtime_cp[remove_cp_offset(cp_ref as usize)];
    Ok((current_content, OpCode::Invokevirtual(cp_entry.clone())))
}

fn parse_invokespecial<'a>(
    current_content: &'a [u8],
    _class_file: &ClassFile,
    runtime_cp: &[RuntimeCPEntry],
) -> IResult<&'a [u8], OpCode> {
    let (current_content, cp_ref) = be_u16(current_content)?;
    let cp_entry = &runtime_cp[remove_cp_offset(cp_ref as usize)];
    Ok((current_content, OpCode::Invokespecial(cp_entry.clone())))
}

pub fn parse_opcodes<'a>(
    code: &'a Vec<u8>,
    class_file: &ClassFile,
    runtime_cp: &[RuntimeCPEntry],
) -> IResult<&'a [u8], Vec<OpCode>> {
    let mut current_content = code.as_slice();
    let mut opcodes: Vec<OpCode> = Vec::new();
    while !current_content.is_empty() {
        let opcode;
        (current_content, opcode) = be_u8(current_content)?;
        match opcode {
            18 => {
                let (new_content, opcode) =
                    parse_ldc(current_content, class_file, runtime_cp)?;
                opcodes.push(opcode);
                current_content = new_content;
            },
            42 => {
                opcodes.push(OpCode::Aload0());
            },
            177 => {
                opcodes.push(OpCode::Return());
            },
            178 => {
                let (new_content, opcode) =
                    parse_getstatic(current_content, class_file, runtime_cp)?;
                opcodes.push(opcode);
                current_content = new_content;
            },
            182 => {
                let (new_content, opcode) = parse_invokevirtual(
                    current_content,
                    class_file,
                    runtime_cp,
                )?;
                opcodes.push(opcode);
                current_content = new_content;
            },
            183 => {
                let (new_content, opcode) = parse_invokespecial(
                    current_content,
                    class_file,
                    runtime_cp,
                )?;
                opcodes.push(opcode);
                current_content = new_content;
            },
            _ => {
                panic!("Unsupported Opcode {}", opcode)
            },
        }
    }

    Ok((current_content, opcodes))
}
