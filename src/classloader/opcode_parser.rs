use ::std::rc::Rc;
use nom::{
    number::complete::{be_i8, be_u16, be_u8},
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
            2..=8 => opcodes.push(OpCode::Iconst(-1 + (i32::from(opcode) - 2))),
            16 => {
                let (new_content, byte_value) = be_u8(current_content)?;
                current_content = new_content;
                opcodes.push(OpCode::Bipush(byte_value));
            },
            18 => {
                let (new_content, opcode) =
                    parse_ldc(current_content, class_file, runtime_cp, heap)?;
                opcodes.push(opcode);
                current_content = new_content;
            },
            21 => {
                let (new_content, index) = be_u8(current_content)?;
                current_content = new_content;
                opcodes.push(OpCode::Iload(index.into()));
            },
            26..=29 => opcodes.push(OpCode::Iload((opcode - 26).into())),
            41 => {
                let (new_content, index) = be_u8(current_content)?;
                current_content = new_content;
                opcodes.push(OpCode::Aload(index.into()));
            },
            42..=45 => {
                opcodes.push(OpCode::Aload((opcode - 42).into()));
            },
            54 => {
                let (new_content, index) = be_u8(current_content)?;
                current_content = new_content;
                opcodes.push(OpCode::Istore(index.into()));
            },
            55 => {
                let (new_content, index) = be_u8(current_content)?;
                current_content = new_content;
                opcodes.push(OpCode::Lstore(index.into()));
            },
            56 => {
                let (new_content, index) = be_u8(current_content)?;
                current_content = new_content;
                opcodes.push(OpCode::Fstore(index.into()));
            },
            57 => {
                let (new_content, index) = be_u8(current_content)?;
                current_content = new_content;
                opcodes.push(OpCode::Dstore(index.into()));
            },
            59..=62 => opcodes.push(OpCode::Istore((opcode - 59).into())),
            63..=66 => opcodes.push(OpCode::Lstore((opcode - 63).into())),
            67..=70 => opcodes.push(OpCode::Istore((opcode - 67).into())),
            71..=74 => opcodes.push(OpCode::Dstore((opcode - 71).into())),
            96 => {
                opcodes.push(OpCode::Iadd);
            },
            100 => {
                opcodes.push(OpCode::Isub);
            },
            104 => {
                opcodes.push(OpCode::Imul);
            },
            108 => {
                opcodes.push(OpCode::Idiv);
            },
            112 => {
                opcodes.push(OpCode::Irem);
            },
            116 => {
                opcodes.push(OpCode::Ineg);
            },
            120 => {
                opcodes.push(OpCode::Ishl);
            },
            122 => {
                opcodes.push(OpCode::Ishr);
            },
            124 => {
                opcodes.push(OpCode::Iushr);
            },
            126 => {
                opcodes.push(OpCode::Iand);
            },
            128 => {
                opcodes.push(OpCode::Ior);
            },
            130 => {
                opcodes.push(OpCode::Ixor);
            },
            132 => {
                let (new_content, index) = be_u8(current_content)?;
                let (new_content, constant) = be_i8(new_content)?;
                current_content = new_content;
                opcodes.push(OpCode::Iinc {
                    index: index.into(),
                    constant: constant.into(),
                });
            },
            133 => {
                opcodes.push(OpCode::I2l);
            },
            134 => {
                opcodes.push(OpCode::I2f);
            },
            135 => {
                opcodes.push(OpCode::I2d);
            },
            145 => {
                opcodes.push(OpCode::I2b);
            },
            146 => {
                opcodes.push(OpCode::I2c);
            },
            147 => {
                opcodes.push(OpCode::I2s);
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
