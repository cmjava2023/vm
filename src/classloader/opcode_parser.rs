use ::std::rc::Rc;
use nom::{
    number::complete::{be_u16, be_u8},
    IResult,
};

use crate::{
    classloader::{
        cp_decoder::{remove_cp_offset, RuntimeCPEntry},
        ClassFile,
    },
    executor::{op_code::Ldc, OpCode},
    heap::Heap,
};

fn parse_getstatic<'a>(
    current_content: &'a [u8],
    _class_file: &ClassFile,
    runtime_cp: &[RuntimeCPEntry],
    heap: &mut Heap,
) -> IResult<&'a [u8], OpCode> {
    let (current_content, cp_ref) = be_u16(current_content)?;
    let cp_entry = &runtime_cp[remove_cp_offset(cp_ref as usize)];
    let (name, class_name, _) = cp_entry
        .as_field_ref()
        .unwrap_or_else(|| panic!("CPentry {:?} is FieldRefInfo", cp_entry));
    let class = heap
        .find_class(class_name)
        .unwrap_or_else(|| panic!("Class with name  {} exists", class_name));
    let field = class.get_static_field(name).unwrap_or_else(|| {
        panic!("Class with name{} has method {}", class_name, name)
    });
    Ok((current_content, OpCode::GetStatic(field)))
}

fn parse_ldc<'a>(
    current_content: &'a [u8],
    _class_file: &ClassFile,
    runtime_cp: &[RuntimeCPEntry],
    heap: &mut Heap,
) -> IResult<&'a [u8], OpCode> {
    let (current_content, cp_ref) = be_u8(current_content)?;
    let cp_entry = &runtime_cp[remove_cp_offset(cp_ref as usize)];

    match cp_entry {
        RuntimeCPEntry::StringInfo(value) => Ok((
            current_content,
            OpCode::Ldc(Ldc::String(Rc::new(heap.new_string(value.clone())))),
        )),
        RuntimeCPEntry::IntegerInfo(value) => {
            Ok((current_content, OpCode::Ldc(Ldc::Int(*value))))
        },
        RuntimeCPEntry::FloatInfo(value) => {
            Ok((current_content, OpCode::Ldc(Ldc::Float(*value))))
        },
        _ => panic!("{:?} Unsupported Type for Ldc ", cp_entry),
    }
}

fn parse_invokevirtual<'a>(
    current_content: &'a [u8],
    _class_file: &ClassFile,
    runtime_cp: &[RuntimeCPEntry],
    heap: &mut Heap,
) -> IResult<&'a [u8], OpCode> {
    let (current_content, cp_ref) = be_u16(current_content)?;
    let cp_entry = &runtime_cp[remove_cp_offset(cp_ref as usize)];
    let (class_name, name, _) = cp_entry
        .as_method_ref()
        .unwrap_or_else(|| panic!("CPEntry {:?} is MethodRefInfo", cp_entry));
    let class = heap
        .find_class(class_name)
        .unwrap_or_else(|| panic!("Class with name  {} exists", class_name));
    let method = class.get_method(name).unwrap_or_else(|| {
        panic!("Class with name{} has method {}", class_name, name)
    });

    Ok((current_content, OpCode::InvokeVirtual(method)))
}

fn parse_invokespecial<'a>(
    current_content: &'a [u8],
    _class_file: &ClassFile,
    runtime_cp: &[RuntimeCPEntry],
) -> IResult<&'a [u8], OpCode> {
    let (current_content, cp_ref) = be_u16(current_content)?;
    let cp_entry = &runtime_cp[remove_cp_offset(cp_ref as usize)];
    Ok((current_content, OpCode::InvokeSpecial(cp_entry.clone())))
}

pub fn parse_opcodes<'a>(
    code: &'a Vec<u8>,
    class_file: &ClassFile,
    runtime_cp: &[RuntimeCPEntry],
    heap: &mut Heap,
) -> IResult<&'a [u8], Vec<OpCode>> {
    let mut current_content = code.as_slice();
    let mut opcodes: Vec<OpCode> = Vec::new();
    while !current_content.is_empty() {
        let opcode;
        (current_content, opcode) = be_u8(current_content)?;
        match opcode {
            18 => {
                let (new_content, opcode) =
                    parse_ldc(current_content, class_file, runtime_cp, heap)?;
                opcodes.push(opcode);
                current_content = new_content;
            },
            42 => {
                opcodes.push(OpCode::Aload0);
            },
            177 => {
                opcodes.push(OpCode::Return);
            },
            178 => {
                let (new_content, opcode) = parse_getstatic(
                    current_content,
                    class_file,
                    runtime_cp,
                    heap,
                )?;
                opcodes.push(opcode);
                current_content = new_content;
            },
            182 => {
                let (new_content, opcode) = parse_invokevirtual(
                    current_content,
                    class_file,
                    runtime_cp,
                    heap,
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
