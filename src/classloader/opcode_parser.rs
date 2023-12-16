use ::std::rc::Rc;
use nom::{
    number::complete::{be_i16, be_i32, be_i8, be_u16, be_u8},
    IResult,
};

use crate::{
    class::{Field, Method},
    classloader::{
        class_creator::signature_parser::parse_method_arguments,
        cp_decoder::{remove_cp_offset, RuntimeCPEntry},
        ClassFile,
    },
    executor::{
        op_code::{
            ArrayReferenceKinds, ArrayType, Dup, FloatCmp, Ldc, OffsetDirection,
        },
        OpCode,
    },
    heap::Heap,
};

fn parse_wide(current_content: &[u8]) -> IResult<&[u8], OpCode> {
    let (mut current_content, opcode) = be_u8(current_content)?;
    match opcode {
        21 => {
            let (new_content, index) = be_u16(current_content)?;
            current_content = new_content;
            Ok((current_content, OpCode::Iload(index.into())))
        },
        22 => {
            let (new_content, index) = be_u16(current_content)?;
            current_content = new_content;
            Ok((current_content, OpCode::Lload(index.into())))
        },
        23 => {
            let (new_content, index) = be_u16(current_content)?;
            current_content = new_content;
            Ok((current_content, OpCode::Fload(index.into())))
        },
        24 => {
            let (new_content, index) = be_u16(current_content)?;
            current_content = new_content;
            Ok((current_content, OpCode::Dload(index.into())))
        },
        25 => {
            let (new_content, index) = be_u16(current_content)?;
            current_content = new_content;
            Ok((current_content, OpCode::Aload(index.into())))
        },
        54 => {
            let (new_content, index) = be_u16(current_content)?;
            current_content = new_content;
            Ok((current_content, OpCode::Istore(index.into())))
        },
        55 => {
            let (new_content, index) = be_u16(current_content)?;
            current_content = new_content;
            Ok((current_content, OpCode::Lstore(index.into())))
        },
        56 => {
            let (new_content, index) = be_u16(current_content)?;
            current_content = new_content;
            Ok((current_content, OpCode::Fstore(index.into())))
        },
        57 => {
            let (new_content, index) = be_u16(current_content)?;
            current_content = new_content;
            Ok((current_content, OpCode::Dstore(index.into())))
        },
        58 => {
            let (new_content, index) = be_u16(current_content)?;
            current_content = new_content;
            Ok((current_content, OpCode::Astore(index.into())))
        },
        132 => {
            let (new_content, index) = be_u16(current_content)?;
            let (new_content, constant) = be_i16(new_content)?;
            current_content = new_content;
            Ok((
                current_content,
                OpCode::Iinc {
                    index: index.into(),
                    constant: constant.into(),
                },
            ))
        },
        169 => {
            let (new_content, index) = be_u16(current_content)?;
            current_content = new_content;
            Ok((current_content, OpCode::Ret(index.into())))
        },
        _ => panic!(" OpCode {} is not supported with the wide Opcode", opcode),
    }
}

fn parse_static_field<'a>(
    current_content: &'a [u8],
    _class_file: &ClassFile,
    runtime_cp: &[RuntimeCPEntry],
    heap: &mut Heap,
) -> IResult<&'a [u8], Rc<Field>> {
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
    Ok((current_content, field))
}

fn parse_ldc<'a>(
    current_content: &'a [u8],
    _class_file: &ClassFile,
    runtime_cp: &[RuntimeCPEntry],
    heap: &mut Heap,
    wide: bool,
) -> IResult<&'a [u8], OpCode> {
    let (current_content, cp_ref) = if wide {
        be_u16(current_content)?
    } else {
        let (new_content, new_ref) = be_u8(current_content)?;
        (new_content, new_ref.into())
    };

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
        RuntimeCPEntry::LongInfo(value) => {
            Ok((current_content, OpCode::Ldc(Ldc::Long(*value))))
        },
        RuntimeCPEntry::DoubleInfo(value) => {
            Ok((current_content, OpCode::Ldc(Ldc::Double(*value))))
        },
        _ => panic!("{:?} Unsupported Type for Ldc ", cp_entry),
    }
}

fn parse_cp_method_ref<'a>(
    current_content: &'a [u8],
    _class_file: &ClassFile,
    runtime_cp: &[RuntimeCPEntry],
    heap: &mut Heap,
) -> IResult<&'a [u8], Rc<Method>> {
    let (current_content, cp_ref) = be_u16(current_content)?;
    let cp_entry = &runtime_cp[remove_cp_offset(cp_ref as usize)];
    let (class_name, name, descriptor) = cp_entry
        .as_method_ref()
        .unwrap_or_else(|| panic!("CPEntry {:?} is MethodRefInfo", cp_entry));
    let class = heap
        .find_class(class_name)
        .unwrap_or_else(|| panic!("Class with name  {} exists", class_name));
    let descriptor = parse_method_arguments(descriptor);
    let method = class.get_method(name, descriptor).unwrap_or_else(|| {
        panic!("Class with name{} has method {}", class_name, name)
    });

    Ok((current_content, method))
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

fn signed_offset_to_usize_and_direction(
    offset: i32,
) -> (usize, OffsetDirection) {
    if offset > 0 {
        (offset.try_into().unwrap(), OffsetDirection::Forward)
    } else {
        (offset.abs().try_into().unwrap(), OffsetDirection::Backward)
    }
}

fn byte_offset_to_opcode_offset(
    byte_offset: &usize,
    direction: &OffsetDirection,
    index: usize,
    opcode_sizes: &[u8],
) -> usize {
    let mut remaining_byte_offset = *byte_offset;
    let mut current_index = index;
    if let OffsetDirection::Forward = direction {
        while remaining_byte_offset > 0 {
            remaining_byte_offset -= usize::from(opcode_sizes[current_index]);
            current_index += 1;
        }
        current_index - index
    } else {
        while remaining_byte_offset > 0 {
            current_index -= 1;
            remaining_byte_offset -= usize::from(opcode_sizes[current_index]);
        }
        index - current_index
    }
}

fn parse_branch_offsets(opcodes: &mut [OpCode], opcode_sizes: Vec<u8>) {
    for (i, opcode) in opcodes.iter_mut().enumerate() {
        match opcode {
            OpCode::IfEq(byte_offset, direction) => {
                *opcode = OpCode::IfEq(
                    byte_offset_to_opcode_offset(
                        byte_offset,
                        direction,
                        i,
                        &opcode_sizes,
                    ),
                    *direction,
                )
            },
            OpCode::IfNe(byte_offset, direction) => {
                *opcode = OpCode::IfNe(
                    byte_offset_to_opcode_offset(
                        byte_offset,
                        direction,
                        i,
                        &opcode_sizes,
                    ),
                    *direction,
                )
            },
            OpCode::IfLt(byte_offset, direction) => {
                *opcode = OpCode::IfLt(
                    byte_offset_to_opcode_offset(
                        byte_offset,
                        direction,
                        i,
                        &opcode_sizes,
                    ),
                    *direction,
                )
            },
            OpCode::IfGe(byte_offset, direction) => {
                *opcode = OpCode::IfGe(
                    byte_offset_to_opcode_offset(
                        byte_offset,
                        direction,
                        i,
                        &opcode_sizes,
                    ),
                    *direction,
                )
            },
            OpCode::IfGt(byte_offset, direction) => {
                *opcode = OpCode::IfGt(
                    byte_offset_to_opcode_offset(
                        byte_offset,
                        direction,
                        i,
                        &opcode_sizes,
                    ),
                    *direction,
                )
            },
            OpCode::IfLe(byte_offset, direction) => {
                *opcode = OpCode::IfLe(
                    byte_offset_to_opcode_offset(
                        byte_offset,
                        direction,
                        i,
                        &opcode_sizes,
                    ),
                    *direction,
                )
            },
            OpCode::IficmpEq(byte_offset, direction) => {
                *opcode = OpCode::IficmpEq(
                    byte_offset_to_opcode_offset(
                        byte_offset,
                        direction,
                        i,
                        &opcode_sizes,
                    ),
                    *direction,
                )
            },
            OpCode::IficmpNe(byte_offset, direction) => {
                *opcode = OpCode::IficmpNe(
                    byte_offset_to_opcode_offset(
                        byte_offset,
                        direction,
                        i,
                        &opcode_sizes,
                    ),
                    *direction,
                )
            },
            OpCode::IficmpLt(byte_offset, direction) => {
                *opcode = OpCode::IficmpLt(
                    byte_offset_to_opcode_offset(
                        byte_offset,
                        direction,
                        i,
                        &opcode_sizes,
                    ),
                    *direction,
                )
            },
            OpCode::IficmpGe(byte_offset, direction) => {
                *opcode = OpCode::IficmpGe(
                    byte_offset_to_opcode_offset(
                        byte_offset,
                        direction,
                        i,
                        &opcode_sizes,
                    ),
                    *direction,
                )
            },
            OpCode::IficmpGt(byte_offset, direction) => {
                *opcode = OpCode::IficmpGt(
                    byte_offset_to_opcode_offset(
                        byte_offset,
                        direction,
                        i,
                        &opcode_sizes,
                    ),
                    *direction,
                )
            },
            OpCode::IficmpLe(byte_offset, direction) => {
                *opcode = OpCode::IficmpLe(
                    byte_offset_to_opcode_offset(
                        byte_offset,
                        direction,
                        i,
                        &opcode_sizes,
                    ),
                    *direction,
                )
            },
            OpCode::IfacmpEq(byte_offset, direction) => {
                *opcode = OpCode::IfacmpEq(
                    byte_offset_to_opcode_offset(
                        byte_offset,
                        direction,
                        i,
                        &opcode_sizes,
                    ),
                    *direction,
                )
            },
            OpCode::IfacmpNe(byte_offset, direction) => {
                *opcode = OpCode::IfacmpNe(
                    byte_offset_to_opcode_offset(
                        byte_offset,
                        direction,
                        i,
                        &opcode_sizes,
                    ),
                    *direction,
                )
            },
            OpCode::Goto(byte_offset, direction) => {
                *opcode = OpCode::Goto(
                    byte_offset_to_opcode_offset(
                        byte_offset,
                        direction,
                        i,
                        &opcode_sizes,
                    ),
                    *direction,
                )
            },
            OpCode::Jsr(byte_offset, direction) => {
                *opcode = OpCode::Jsr(
                    byte_offset_to_opcode_offset(
                        byte_offset,
                        direction,
                        i,
                        &opcode_sizes,
                    ),
                    *direction,
                )
            },
            OpCode::IfNull(byte_offset, direction) => {
                *opcode = OpCode::IfNull(
                    byte_offset_to_opcode_offset(
                        byte_offset,
                        direction,
                        i,
                        &opcode_sizes,
                    ),
                    *direction,
                )
            },
            OpCode::IfNonNull(byte_offset, direction) => {
                *opcode = OpCode::IfNonNull(
                    byte_offset_to_opcode_offset(
                        byte_offset,
                        direction,
                        i,
                        &opcode_sizes,
                    ),
                    *direction,
                )
            },
            _ => {},
        }
    }
}

pub fn parse_opcodes<'a>(
    code: &'a Vec<u8>,
    class_file: &ClassFile,
    runtime_cp: &[RuntimeCPEntry],
    heap: &mut Heap,
) -> IResult<&'a [u8], Vec<OpCode>> {
    let mut current_content = code.as_slice();
    let mut opcodes: Vec<OpCode> = Vec::new();
    let mut opcode_sizes: Vec<u8> = Vec::new();
    while !current_content.is_empty() {
        let opcode;
        (current_content, opcode) = be_u8(current_content)?;
        match opcode {
            0 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Nop);
            },
            1 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::AconstNull);
            },
            2..=8 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Iconst(-1 + (i32::from(opcode) - 2)));
            },
            9 | 10 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Lconst(i64::from(opcode) - 9));
            },
            11..=13 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Fconst(f32::from(opcode) - 11_f32));
            },
            14 | 15 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Dconst(f64::from(opcode) - 14_f64));
            },
            16 => {
                opcode_sizes.push(2);
                let (new_content, byte_value) = be_i8(current_content)?;
                current_content = new_content;
                opcodes.push(OpCode::Bipush(byte_value.into()));
            },
            17 => {
                opcode_sizes.push(3);
                let (new_content, byte_value) = be_i16(current_content)?;
                current_content = new_content;
                opcodes.push(OpCode::Sipush(byte_value.into()));
            },
            18 => {
                opcode_sizes.push(2);
                let (new_content, opcode) = parse_ldc(
                    current_content,
                    class_file,
                    runtime_cp,
                    heap,
                    false,
                )?;
                opcodes.push(opcode);
                current_content = new_content;
            },
            // because long and double are not treated specially in cp,
            // we can parse ldc_w and ldc2_w the same way
            19 | 20 => {
                opcode_sizes.push(3);
                let (new_content, opcode) = parse_ldc(
                    current_content,
                    class_file,
                    runtime_cp,
                    heap,
                    true,
                )?;
                opcodes.push(opcode);
                current_content = new_content;
            },
            21 => {
                opcode_sizes.push(2);
                let (new_content, index) = be_u8(current_content)?;
                current_content = new_content;
                opcodes.push(OpCode::Iload(index.into()));
            },
            22 => {
                opcode_sizes.push(2);
                let (new_content, index) = be_u8(current_content)?;
                current_content = new_content;
                opcodes.push(OpCode::Lload(index.into()));
            },
            23 => {
                opcode_sizes.push(2);
                let (new_content, index) = be_u8(current_content)?;
                current_content = new_content;
                opcodes.push(OpCode::Fload(index.into()));
            },
            24 => {
                opcode_sizes.push(2);
                let (new_content, index) = be_u8(current_content)?;
                current_content = new_content;
                opcodes.push(OpCode::Dload(index.into()));
            },
            25 => {
                opcode_sizes.push(2);
                let (new_content, index) = be_u8(current_content)?;
                current_content = new_content;
                opcodes.push(OpCode::Aload(index.into()));
            },
            26..=29 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Iload((opcode - 26).into()));
            },
            30..=33 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Lload((opcode - 30).into()));
            },
            34..=37 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Fload((opcode - 34).into()));
            },
            38..=41 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Dload((opcode - 38).into()));
            },
            42..=45 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Aload((opcode - 42).into()));
            },
            46 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Iaload);
            },
            47 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Laload);
            },
            48 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Faload);
            },
            49 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Daload);
            },
            50 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Aaload);
            },
            51 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Baload);
            },
            52 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Caload);
            },
            53 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Saload);
            },
            54 => {
                opcode_sizes.push(2);
                let (new_content, index) = be_u8(current_content)?;
                current_content = new_content;
                opcodes.push(OpCode::Istore(index.into()));
            },
            55 => {
                opcode_sizes.push(2);
                let (new_content, index) = be_u8(current_content)?;
                current_content = new_content;
                opcodes.push(OpCode::Lstore(index.into()));
            },
            56 => {
                opcode_sizes.push(2);
                let (new_content, index) = be_u8(current_content)?;
                current_content = new_content;
                opcodes.push(OpCode::Fstore(index.into()));
            },
            57 => {
                opcode_sizes.push(2);
                let (new_content, index) = be_u8(current_content)?;
                current_content = new_content;
                opcodes.push(OpCode::Dstore(index.into()));
            },
            58 => {
                opcode_sizes.push(2);
                let (new_content, index) = be_u8(current_content)?;
                current_content = new_content;
                opcodes.push(OpCode::Astore(index.into()));
            },
            59..=62 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Istore((opcode - 59).into()));
            },
            63..=66 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Lstore((opcode - 63).into()));
            },
            67..=70 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Istore((opcode - 67).into()));
            },
            71..=74 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Dstore((opcode - 71).into()));
            },
            75..=78 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Astore((opcode - 75).into()));
            },
            79 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Iastore);
            },
            80 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Lastore);
            },
            81 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Fastore);
            },
            82 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Dastore);
            },
            83 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Aastore);
            },
            84 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Bastore);
            },
            85 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Castore);
            },
            86 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Sastore);
            },
            87 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Pop);
            },
            88 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Pop2);
            },
            89 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Dup(Dup::Dup));
            },
            90 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Dup(Dup::X1));
            },
            91 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Dup(Dup::X2));
            },
            92 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Dup2(Dup::Dup));
            },
            93 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Dup2(Dup::X1));
            },
            94 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Dup2(Dup::X2));
            },
            95 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Swap);
            },
            96 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Iadd);
            },
            97 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Ladd);
            },
            98 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Fadd);
            },
            99 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Dadd);
            },
            100 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Isub);
            },
            101 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Lsub);
            },
            102 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Fsub);
            },
            103 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Dsub);
            },
            104 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Imul);
            },
            105 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Lmul);
            },
            106 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Fmul);
            },
            107 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Dmul);
            },
            108 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Idiv);
            },
            109 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Ldiv);
            },
            110 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Fdiv);
            },
            111 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Ddiv);
            },
            112 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Irem);
            },
            113 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Lrem);
            },
            114 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Frem);
            },
            115 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Drem);
            },
            116 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Ineg);
            },
            117 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Lneg);
            },
            118 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Fneg);
            },
            119 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Dneg);
            },
            120 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Ishl);
            },
            121 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Lshl);
            },
            122 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Ishr);
            },
            123 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Lshr);
            },
            124 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Iushr);
            },
            125 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Lushr);
            },
            126 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Iand);
            },
            127 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Land);
            },
            128 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Ior);
            },
            129 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Lor);
            },
            130 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Ixor);
            },
            131 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Lxor);
            },
            132 => {
                opcode_sizes.push(3);
                let (new_content, index) = be_u8(current_content)?;
                let (new_content, constant) = be_i8(new_content)?;
                current_content = new_content;
                opcodes.push(OpCode::Iinc {
                    index: index.into(),
                    constant: constant.into(),
                });
            },
            133 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::I2l);
            },
            134 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::I2f);
            },
            135 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::I2d);
            },
            136 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::L2i);
            },
            137 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::L2f);
            },
            138 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::L2d);
            },
            139 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::F2i);
            },
            140 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::F2l);
            },
            141 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::F2d);
            },
            142 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::D2i);
            },
            143 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::D2l);
            },
            144 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::D2f);
            },
            145 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::I2b);
            },
            146 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::I2c);
            },
            147 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::I2s);
            },
            148 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Lcmp);
            },
            149 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Fcmp(FloatCmp::Pl));
            },
            150 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Fcmp(FloatCmp::Pg));
            },
            151 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Dcmp(FloatCmp::Pl));
            },
            152 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Dcmp(FloatCmp::Pg));
            },
            153 => {
                opcode_sizes.push(3);
                let (new_content, index) = be_i16(current_content)?;
                current_content = new_content;
                let (offset, direction) =
                    signed_offset_to_usize_and_direction(index.into());
                opcodes.push(OpCode::IfEq(offset, direction));
            },
            154 => {
                opcode_sizes.push(3);
                let (new_content, index) = be_i16(current_content)?;
                current_content = new_content;
                let (offset, direction) =
                    signed_offset_to_usize_and_direction(index.into());
                opcodes.push(OpCode::IfNe(offset, direction));
            },
            155 => {
                opcode_sizes.push(3);
                let (new_content, index) = be_i16(current_content)?;
                current_content = new_content;
                let (offset, direction) =
                    signed_offset_to_usize_and_direction(index.into());
                opcodes.push(OpCode::IfLt(offset, direction));
            },
            156 => {
                opcode_sizes.push(3);
                let (new_content, index) = be_i16(current_content)?;
                current_content = new_content;
                let (offset, direction) =
                    signed_offset_to_usize_and_direction(index.into());
                opcodes.push(OpCode::IfGe(offset, direction));
            },
            157 => {
                opcode_sizes.push(3);
                let (new_content, index) = be_i16(current_content)?;
                current_content = new_content;
                let (offset, direction) =
                    signed_offset_to_usize_and_direction(index.into());
                opcodes.push(OpCode::IfGt(offset, direction));
            },
            158 => {
                opcode_sizes.push(3);
                let (new_content, index) = be_i16(current_content)?;
                current_content = new_content;
                let (offset, direction) =
                    signed_offset_to_usize_and_direction(index.into());
                opcodes.push(OpCode::IfLe(offset, direction));
            },
            159 => {
                opcode_sizes.push(3);
                let (new_content, index) = be_i16(current_content)?;
                current_content = new_content;
                let (offset, direction) =
                    signed_offset_to_usize_and_direction(index.into());
                opcodes.push(OpCode::IficmpEq(offset, direction));
            },
            160 => {
                opcode_sizes.push(3);
                let (new_content, index) = be_i16(current_content)?;
                current_content = new_content;
                let (offset, direction) =
                    signed_offset_to_usize_and_direction(index.into());
                opcodes.push(OpCode::IficmpNe(offset, direction));
            },
            161 => {
                opcode_sizes.push(3);
                let (new_content, index) = be_i16(current_content)?;
                current_content = new_content;
                let (offset, direction) =
                    signed_offset_to_usize_and_direction(index.into());
                opcodes.push(OpCode::IficmpLt(offset, direction));
            },
            162 => {
                opcode_sizes.push(3);
                let (new_content, index) = be_i16(current_content)?;
                current_content = new_content;
                let (offset, direction) =
                    signed_offset_to_usize_and_direction(index.into());
                opcodes.push(OpCode::IficmpGe(offset, direction));
            },
            163 => {
                opcode_sizes.push(3);
                let (new_content, index) = be_i16(current_content)?;
                current_content = new_content;
                let (offset, direction) =
                    signed_offset_to_usize_and_direction(index.into());
                opcodes.push(OpCode::IficmpGt(offset, direction));
            },
            164 => {
                opcode_sizes.push(3);
                let (new_content, index) = be_i16(current_content)?;
                current_content = new_content;
                let (offset, direction) =
                    signed_offset_to_usize_and_direction(index.into());
                opcodes.push(OpCode::IficmpLe(offset, direction));
            },
            165 => {
                opcode_sizes.push(3);
                let (new_content, index) = be_i16(current_content)?;
                current_content = new_content;
                let (offset, direction) =
                    signed_offset_to_usize_and_direction(index.into());
                opcodes.push(OpCode::IfacmpEq(offset, direction));
            },
            166 => {
                opcode_sizes.push(3);
                let (new_content, index) = be_i16(current_content)?;
                current_content = new_content;
                let (offset, direction) =
                    signed_offset_to_usize_and_direction(index.into());
                opcodes.push(OpCode::IfacmpNe(offset, direction));
            },
            167 => {
                opcode_sizes.push(3);
                let (new_content, index) = be_i16(current_content)?;
                current_content = new_content;
                let (offset, direction) =
                    signed_offset_to_usize_and_direction(index.into());
                opcodes.push(OpCode::Goto(offset, direction));
            },
            168 => {
                opcode_sizes.push(3);
                let (new_content, index) = be_i16(current_content)?;
                current_content = new_content;
                let (offset, direction) =
                    signed_offset_to_usize_and_direction(index.into());
                opcodes.push(OpCode::Jsr(offset, direction));
            },
            169 => {
                opcode_sizes.push(2);
                let (new_content, index) = be_u8(current_content)?;
                current_content = new_content;
                opcodes.push(OpCode::Ret(index.into()));
            },
            170 => {
                todo!("Tabeleswitch will not be supported")
            },
            171 => {
                todo!("LookupSwitch will not be supported")
            },
            172 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Ireturn);
            },
            173 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Lreturn);
            },
            174 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Freturn);
            },
            175 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Dreturn);
            },
            176 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Areturn);
            },
            177 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Return);
            },
            178 => {
                opcode_sizes.push(3);
                let (new_content, field) = parse_static_field(
                    current_content,
                    class_file,
                    runtime_cp,
                    heap,
                )?;
                opcodes.push(OpCode::GetStatic(field));
                current_content = new_content;
            },
            179 => {
                opcode_sizes.push(3);
                let (new_content, field) = parse_static_field(
                    current_content,
                    class_file,
                    runtime_cp,
                    heap,
                )?;
                opcodes.push(OpCode::PutStatic(field));
                current_content = new_content;
            },
            180 => {
                opcode_sizes.push(3);
                todo!(
                    "GetField(Rc<dyn Any>), needs information \
on how to resolve fields at execution time"
                )
            },
            181 => {
                opcode_sizes.push(3);
                todo!(
                    "PutField(Rc<dyn Any>), needs information \
on how to resolve fields at execution time"
                )
            },
            182 => {
                opcode_sizes.push(3);
                let (new_content, method) = parse_cp_method_ref(
                    current_content,
                    class_file,
                    runtime_cp,
                    heap,
                )?;
                opcodes.push(OpCode::InvokeVirtual(method));
                current_content = new_content;
            },
            183 => {
                opcode_sizes.push(3);
                let (new_content, opcode) = parse_invokespecial(
                    current_content,
                    class_file,
                    runtime_cp,
                )?;
                opcodes.push(opcode);
                current_content = new_content;
            },
            184 => {
                opcode_sizes.push(3);
                let (new_content, method) = parse_cp_method_ref(
                    current_content,
                    class_file,
                    runtime_cp,
                    heap,
                )?;
                opcodes.push(OpCode::InvokeStatic(method));
                current_content = new_content;
            },
            185 => {
                opcode_sizes.push(5);
                todo!(
                    "InvokeInterface(Rc<dyn Any>), needs information \
on how to resolve Interface at execution time"
                )
            },
            186 => {
                opcode_sizes.push(5);
                todo!(
                    "InvokeDynamic(Rc<dyn Any>),, needs information \
on how to resolve at execution time"
                )
            },
            187 => {
                opcode_sizes.push(3);
                todo!(
                    "New(Rc<dyn Any>), needs information \
on how to resolve at execution time"
                )
            },
            188 => {
                opcode_sizes.push(2);
                let (new_content, value) = be_u8(current_content)?;
                current_content = new_content;
                opcodes.push(OpCode::NewArray(ArrayType::from_int(value)));
            },
            189 => {
                opcode_sizes.push(3);
                let (new_content, index) = be_u16(current_content)?;
                current_content = new_content;
                let cp_entry = &runtime_cp[remove_cp_offset(index as usize)];
                let class_name = cp_entry.as_class().unwrap();
                // get underlying component class for array
                let array_cls = if class_name.starts_with('[') {
                    // array
                    let (array_dim, kind) =
                        class_name.rsplit_once('[').unwrap();
                    let scalar_class = match kind {
                        "L" => {
                            let cls_name = &kind[1..kind.len() - 1];
                            ArrayReferenceKinds::Object(
                                heap.find_class(cls_name).unwrap().clone(),
                            )
                        },
                        "Z" => ArrayReferenceKinds::Boolean,
                        "B" => ArrayReferenceKinds::Byte,
                        "C" => ArrayReferenceKinds::Char,
                        "D" => ArrayReferenceKinds::Double,
                        "F" => ArrayReferenceKinds::Float,
                        "J" => ArrayReferenceKinds::Long,
                        "I" => ArrayReferenceKinds::Int,
                        "S" => ArrayReferenceKinds::Short,
                        _ => panic!(
                            "unexpected array class name: {}",
                            class_name
                        ),
                    };

                    // +1 for removed dim in rsplit
                    // +1 for implicit dim in op-code anewarray
                    let dim = array_dim.len() + 2;

                    heap.find_array_class(scalar_class, dim.try_into().unwrap())
                } else {
                    let scalar_class = heap.find_class(class_name).unwrap();
                    // object
                    heap.find_array_class(
                        ArrayReferenceKinds::Object(scalar_class.clone()),
                        1,
                    )
                }
                .unwrap();
                opcodes.push(OpCode::AnewArray(array_cls));
            },
            190 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::ArrayLength);
            },
            191 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Athrow);
            },
            192 => {
                opcode_sizes.push(3);
                todo!(
                    "Checkcast(Rc<dyn Any>), needs information \
on how to resolve at execution time"
                )
            },
            193 => {
                opcode_sizes.push(3);
                todo!(
                    "InstanceOf(Rc<dyn Any>), needs information \
on how to resolve at execution time"
                )
            },
            194 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Monitorenter);
            },
            195 => {
                opcode_sizes.push(1);
                opcodes.push(OpCode::Monitorexit);
            },
            196 => {
                let (new_content, opcode) = parse_wide(current_content)?;
                if let OpCode::Iinc {
                    index: _,
                    constant: _,
                } = opcode
                {
                    opcode_sizes.push(6);
                } else {
                    opcode_sizes.push(4);
                }
                current_content = new_content;
                opcodes.push(opcode);
            },
            197 => {
                opcode_sizes.push(4);
                let (new_content, index) = be_u16(current_content)?;
                let (new_content, dimensions) = be_u8(new_content)?;
                current_content = new_content;
                let cp_entry = &runtime_cp[remove_cp_offset(index as usize)];
                let class_name = cp_entry.as_class().unwrap();
                // get underlying component class for array
                let array_cls = if class_name.starts_with('[') {
                    // array
                    let (_, kind) = class_name.rsplit_once('[').unwrap();
                    match kind {
                        "L" => {
                            let cls_name = &kind[1..kind.len() - 1];
                            ArrayReferenceKinds::Object(
                                heap.find_class(cls_name).unwrap().clone(),
                            )
                        },
                        "Z" => ArrayReferenceKinds::Boolean,
                        "B" => ArrayReferenceKinds::Byte,
                        "C" => ArrayReferenceKinds::Char,
                        "D" => ArrayReferenceKinds::Double,
                        "F" => ArrayReferenceKinds::Float,
                        "J" => ArrayReferenceKinds::Long,
                        "I" => ArrayReferenceKinds::Int,
                        "S" => ArrayReferenceKinds::Short,
                        _ => panic!(
                            "unexpected array class name: {}",
                            class_name
                        ),
                    }
                } else {
                    ArrayReferenceKinds::Object(
                        heap.find_class(class_name).unwrap().clone(),
                    )
                };
                opcodes.push(OpCode::MultiAnewArray {
                    reference_kind: array_cls,
                    dimensions,
                });
            },
            198 => {
                opcode_sizes.push(3);
                let (new_content, index) = be_i16(current_content)?;
                current_content = new_content;
                let (offset, direction) =
                    signed_offset_to_usize_and_direction(index.into());
                opcodes.push(OpCode::IfNull(offset, direction));
            },
            199 => {
                opcode_sizes.push(3);
                let (new_content, index) = be_i16(current_content)?;
                current_content = new_content;
                let (offset, direction) =
                    signed_offset_to_usize_and_direction(index.into());
                opcodes.push(OpCode::IfNonNull(offset, direction));
            },
            200 => {
                opcode_sizes.push(5);
                let (new_content, index) = be_i32(current_content)?;
                current_content = new_content;
                let (offset, direction) =
                    signed_offset_to_usize_and_direction(index);
                opcodes.push(OpCode::Goto(offset, direction));
            },
            201 => {
                opcode_sizes.push(5);
                let (new_content, index) = be_i32(current_content)?;
                current_content = new_content;
                let (offset, direction) =
                    signed_offset_to_usize_and_direction(index);
                opcodes.push(OpCode::Jsr(offset, direction));
            },
            _ => {
                panic!("{} is not a valid Opcode", opcode)
            },
        }
    }
    parse_branch_offsets(&mut opcodes, opcode_sizes);
    Ok((current_content, opcodes))
}
