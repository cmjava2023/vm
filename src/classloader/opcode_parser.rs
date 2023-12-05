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
        op_code::{ArrayType, Dup, FloatCmp, Ldc, OffsetDirection},
        OpCode,
    },
    heap::Heap,
};

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
            0 => {
                opcodes.push(OpCode::Nop);
            },
            1 => {
                opcodes.push(OpCode::AconstNull);
            },
            2..=8 => opcodes.push(OpCode::Iconst(-1 + (i32::from(opcode) - 2))),
            9 | 10 => opcodes.push(OpCode::Lconst(i64::from(opcode) - 9)),
            11..=13 => opcodes.push(OpCode::Fconst(f32::from(opcode) - 11_f32)),
            14 | 15 => opcodes.push(OpCode::Dconst(f64::from(opcode) - 14_f64)),
            16 => {
                let (new_content, byte_value) = be_i8(current_content)?;
                current_content = new_content;
                opcodes.push(OpCode::Bipush(byte_value.into()));
            },
            17 => {
                let (new_content, byte_value) = be_i16(current_content)?;
                current_content = new_content;
                opcodes.push(OpCode::Sipush(byte_value));
            },
            18 => {
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
                let (new_content, index) = be_u8(current_content)?;
                current_content = new_content;
                opcodes.push(OpCode::Iload(index.into()));
            },
            22 => {
                let (new_content, index) = be_u8(current_content)?;
                current_content = new_content;
                opcodes.push(OpCode::Lload(index.into()));
            },
            23 => {
                let (new_content, index) = be_u8(current_content)?;
                current_content = new_content;
                opcodes.push(OpCode::Fload(index.into()));
            },
            24 => {
                let (new_content, index) = be_u8(current_content)?;
                current_content = new_content;
                opcodes.push(OpCode::Dload(index.into()));
            },
            25 => {
                let (new_content, index) = be_u8(current_content)?;
                current_content = new_content;
                opcodes.push(OpCode::Aload(index.into()));
            },
            26..=29 => opcodes.push(OpCode::Iload((opcode - 26).into())),
            30..=33 => opcodes.push(OpCode::Lload((opcode - 30).into())),
            34..=37 => opcodes.push(OpCode::Fload((opcode - 34).into())),
            38..=41 => opcodes.push(OpCode::Dload((opcode - 38).into())),
            42..=45 => {
                opcodes.push(OpCode::Aload((opcode - 42).into()));
            },
            46 => {
                opcodes.push(OpCode::Iaload);
            },
            47 => {
                opcodes.push(OpCode::Laload);
            },
            48 => {
                opcodes.push(OpCode::Faload);
            },
            49 => {
                opcodes.push(OpCode::Daload);
            },
            50 => {
                opcodes.push(OpCode::Aalod);
            },
            51 => {
                opcodes.push(OpCode::Baload);
            },
            52 => {
                opcodes.push(OpCode::Caload);
            },
            53 => {
                opcodes.push(OpCode::Saload);
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
            58 => {
                let (new_content, index) = be_u8(current_content)?;
                current_content = new_content;
                opcodes.push(OpCode::Astore(index.into()));
            },
            59..=62 => opcodes.push(OpCode::Istore((opcode - 59).into())),
            63..=66 => opcodes.push(OpCode::Lstore((opcode - 63).into())),
            67..=70 => opcodes.push(OpCode::Istore((opcode - 67).into())),
            71..=74 => opcodes.push(OpCode::Dstore((opcode - 71).into())),
            75..=78 => opcodes.push(OpCode::Astore((opcode - 75).into())),
            79 => {
                opcodes.push(OpCode::Iastore);
            },
            80 => {
                opcodes.push(OpCode::Lastore);
            },
            81 => {
                opcodes.push(OpCode::Fastore);
            },
            82 => {
                opcodes.push(OpCode::Dastore);
            },
            83 => {
                opcodes.push(OpCode::Aastore);
            },
            84 => {
                opcodes.push(OpCode::Bastore);
            },
            85 => {
                opcodes.push(OpCode::Castore);
            },
            86 => {
                opcodes.push(OpCode::Sastore);
            },
            87 => {
                opcodes.push(OpCode::Pop);
            },
            88 => {
                opcodes.push(OpCode::Pop2);
            },
            89 => {
                opcodes.push(OpCode::Dup(Dup::Dup));
            },
            90 => {
                opcodes.push(OpCode::Dup(Dup::X1));
            },
            91 => {
                opcodes.push(OpCode::Dup(Dup::X2));
            },
            92 => {
                opcodes.push(OpCode::Dup2(Dup::Dup));
            },
            93 => {
                opcodes.push(OpCode::Dup2(Dup::X1));
            },
            94 => {
                opcodes.push(OpCode::Dup2(Dup::X2));
            },
            95 => {
                opcodes.push(OpCode::Swap);
            },
            96 => {
                opcodes.push(OpCode::Iadd);
            },
            97 => {
                opcodes.push(OpCode::Ladd);
            },
            98 => {
                opcodes.push(OpCode::Fadd);
            },
            99 => {
                opcodes.push(OpCode::Dadd);
            },
            100 => {
                opcodes.push(OpCode::Isub);
            },
            101 => {
                opcodes.push(OpCode::Lsub);
            },
            102 => {
                opcodes.push(OpCode::Fsub);
            },
            103 => {
                opcodes.push(OpCode::Dsub);
            },
            104 => {
                opcodes.push(OpCode::Imul);
            },
            105 => {
                opcodes.push(OpCode::Lmul);
            },
            106 => {
                opcodes.push(OpCode::Fmul);
            },
            107 => {
                opcodes.push(OpCode::Dmul);
            },
            108 => {
                opcodes.push(OpCode::Idiv);
            },
            109 => {
                opcodes.push(OpCode::Ldiv);
            },
            110 => {
                opcodes.push(OpCode::Fdiv);
            },
            111 => {
                opcodes.push(OpCode::Ddiv);
            },
            112 => {
                opcodes.push(OpCode::Irem);
            },
            113 => {
                opcodes.push(OpCode::Lrem);
            },
            114 => {
                opcodes.push(OpCode::Frem);
            },
            115 => {
                opcodes.push(OpCode::Drem);
            },
            116 => {
                opcodes.push(OpCode::Ineg);
            },
            117 => {
                opcodes.push(OpCode::Lneg);
            },
            118 => {
                opcodes.push(OpCode::Fneg);
            },
            119 => {
                opcodes.push(OpCode::Dneg);
            },
            120 => {
                opcodes.push(OpCode::Ishl);
            },
            121 => {
                opcodes.push(OpCode::Lshl);
            },
            122 => {
                opcodes.push(OpCode::Ishr);
            },
            123 => {
                opcodes.push(OpCode::Lshr);
            },
            124 => {
                opcodes.push(OpCode::Iushr);
            },
            125 => {
                opcodes.push(OpCode::Lushr);
            },
            126 => {
                opcodes.push(OpCode::Iand);
            },
            127 => {
                opcodes.push(OpCode::Land);
            },
            128 => {
                opcodes.push(OpCode::Ior);
            },
            129 => {
                opcodes.push(OpCode::Lor);
            },
            130 => {
                opcodes.push(OpCode::Ixor);
            },
            131 => {
                opcodes.push(OpCode::Lxor);
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
            136 => {
                opcodes.push(OpCode::L2i);
            },
            137 => {
                opcodes.push(OpCode::L2f);
            },
            138 => {
                opcodes.push(OpCode::L2d);
            },
            139 => {
                opcodes.push(OpCode::F2i);
            },
            140 => {
                opcodes.push(OpCode::F2l);
            },
            141 => {
                opcodes.push(OpCode::F2d);
            },
            142 => {
                opcodes.push(OpCode::D2i);
            },
            143 => {
                opcodes.push(OpCode::D2l);
            },
            144 => {
                opcodes.push(OpCode::D2f);
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
            148 => {
                opcodes.push(OpCode::Lcmp);
            },
            149 => {
                opcodes.push(OpCode::Fcmp(FloatCmp::Pl));
            },
            150 => {
                opcodes.push(OpCode::Fcmp(FloatCmp::Pg));
            },
            151 => {
                opcodes.push(OpCode::Dcmp(FloatCmp::Pl));
            },
            152 => {
                opcodes.push(OpCode::Dcmp(FloatCmp::Pg));
            },
            153 => {
                let (new_content, index) = be_i16(current_content)?;
                current_content = new_content;
                let (offset, direction) =
                    signed_offset_to_usize_and_direction(index.into());
                opcodes.push(OpCode::IfEq(offset, direction));
            },
            154 => {
                let (new_content, index) = be_i16(current_content)?;
                current_content = new_content;
                let (offset, direction) =
                    signed_offset_to_usize_and_direction(index.into());
                opcodes.push(OpCode::IfNe(offset, direction));
            },
            155 => {
                let (new_content, index) = be_i16(current_content)?;
                current_content = new_content;
                let (offset, direction) =
                    signed_offset_to_usize_and_direction(index.into());
                opcodes.push(OpCode::IfLt(offset, direction));
            },
            156 => {
                let (new_content, index) = be_i16(current_content)?;
                current_content = new_content;
                let (offset, direction) =
                    signed_offset_to_usize_and_direction(index.into());
                opcodes.push(OpCode::IfGe(offset, direction));
            },
            157 => {
                let (new_content, index) = be_i16(current_content)?;
                current_content = new_content;
                let (offset, direction) =
                    signed_offset_to_usize_and_direction(index.into());
                opcodes.push(OpCode::IfGt(offset, direction));
            },
            158 => {
                let (new_content, index) = be_i16(current_content)?;
                current_content = new_content;
                let (offset, direction) =
                    signed_offset_to_usize_and_direction(index.into());
                opcodes.push(OpCode::IfLe(offset, direction));
            },
            159 => {
                let (new_content, index) = be_i16(current_content)?;
                current_content = new_content;
                let (offset, direction) =
                    signed_offset_to_usize_and_direction(index.into());
                opcodes.push(OpCode::IficmpEq(offset, direction));
            },
            160 => {
                let (new_content, index) = be_i16(current_content)?;
                current_content = new_content;
                let (offset, direction) =
                    signed_offset_to_usize_and_direction(index.into());
                opcodes.push(OpCode::IficmpNe(offset, direction));
            },
            161 => {
                let (new_content, index) = be_i16(current_content)?;
                current_content = new_content;
                let (offset, direction) =
                    signed_offset_to_usize_and_direction(index.into());
                opcodes.push(OpCode::IficmpLt(offset, direction));
            },
            162 => {
                let (new_content, index) = be_i16(current_content)?;
                current_content = new_content;
                let (offset, direction) =
                    signed_offset_to_usize_and_direction(index.into());
                opcodes.push(OpCode::IficmpGe(offset, direction));
            },
            163 => {
                let (new_content, index) = be_i16(current_content)?;
                current_content = new_content;
                let (offset, direction) =
                    signed_offset_to_usize_and_direction(index.into());
                opcodes.push(OpCode::IficmpGt(offset, direction));
            },
            164 => {
                let (new_content, index) = be_i16(current_content)?;
                current_content = new_content;
                let (offset, direction) =
                    signed_offset_to_usize_and_direction(index.into());
                opcodes.push(OpCode::IficmpLe(offset, direction));
            },
            165 => {
                let (new_content, index) = be_i16(current_content)?;
                current_content = new_content;
                let (offset, direction) =
                    signed_offset_to_usize_and_direction(index.into());
                opcodes.push(OpCode::IfacmpEq(offset, direction));
            },
            166 => {
                let (new_content, index) = be_i16(current_content)?;
                current_content = new_content;
                let (offset, direction) =
                    signed_offset_to_usize_and_direction(index.into());
                opcodes.push(OpCode::IfacmpNe(offset, direction));
            },
            167 => {
                let (new_content, index) = be_i16(current_content)?;
                current_content = new_content;
                let (offset, direction) =
                    signed_offset_to_usize_and_direction(index.into());
                opcodes.push(OpCode::Goto(offset, direction));
            },
            168 => {
                let (new_content, index) = be_i16(current_content)?;
                current_content = new_content;
                let (offset, direction) =
                    signed_offset_to_usize_and_direction(index.into());
                opcodes.push(OpCode::Jsr(offset, direction));
            },
            169 => {
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
                opcodes.push(OpCode::Ireturn);
            },
            173 => {
                opcodes.push(OpCode::Lreturn);
            },
            174 => {
                opcodes.push(OpCode::Freturn);
            },
            175 => {
                opcodes.push(OpCode::Dreturn);
            },
            176 => {
                opcodes.push(OpCode::Areturn);
            },
            177 => {
                opcodes.push(OpCode::Return);
            },
            178 => {
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
                todo!(
                    "GetField(Rc<dyn Any>), needs information
                    on how to resolve fields at execution time"
                )
            },
            181 => {
                todo!(
                    "PutField(Rc<dyn Any>), needs information
                    on how to resolve fields at execution time"
                )
            },
            182 => {
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
                let (new_content, opcode) = parse_invokespecial(
                    current_content,
                    class_file,
                    runtime_cp,
                )?;
                opcodes.push(opcode);
                current_content = new_content;
            },
            184 => {
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
                todo!(
                    "InvokeInterface(Rc<dyn Any>), needs information
                    on how to resolve Interface at execution time"
                )
            },
            186 => {
                todo!(
                    "InvokeDynamic(Rc<dyn Any>),, needs information
                    on how to resolve at execution time"
                )
            },
            187 => {
                todo!(
                    "New(Rc<dyn Any>), needs information
                    on how to resolve at execution time"
                )
            },
            188 => {
                let (new_content, value) = be_u8(current_content)?;
                current_content = new_content;
                opcodes.push(OpCode::NewArray(ArrayType::from_int(value)));
            },
            189 => {
                todo!(
                    "AnewArray(Rc<dyn Any>),, needs information
                    on how to resolve at execution time"
                )
            },
            190 => {
                opcodes.push(OpCode::ArrayLength);
            },
            191 => {
                opcodes.push(OpCode::Athrow);
            },
            192 => {
                todo!(
                    "Checkcast(Rc<dyn Any>), needs information
                     on how to resolve at execution time"
                )
            },
            193 => {
                todo!(
                    "InstanceOf(Rc<dyn Any>), needs information
                     on how to resolve at execution time"
                )
            },
            194 => {
                opcodes.push(OpCode::Monitorenter);
            },
            195 => {
                opcodes.push(OpCode::Monitorexit);
            },
            196 => {
                todo!("Opcode wide is not implemented yet")
            },
            197 => {
                todo!("MultiAnewArray")
            },
            198 => {
                let (new_content, index) = be_i16(current_content)?;
                current_content = new_content;
                let (offset, direction) =
                    signed_offset_to_usize_and_direction(index.into());
                opcodes.push(OpCode::IfNull(offset, direction));
            },
            199 => {
                let (new_content, index) = be_i16(current_content)?;
                current_content = new_content;
                let (offset, direction) =
                    signed_offset_to_usize_and_direction(index.into());
                opcodes.push(OpCode::IfNonNull(offset, direction));
            },
            200 => {
                let (new_content, index) = be_i32(current_content)?;
                current_content = new_content;
                let (offset, direction) =
                    signed_offset_to_usize_and_direction(index);
                opcodes.push(OpCode::Goto(offset, direction));
            },
            201 => {
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

    Ok((current_content, opcodes))
}
