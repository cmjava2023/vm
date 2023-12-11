use std::{any::Any, ops::Neg, rc::Rc};

use crate::{
    class::{
        builtin_classes::array::{
            BoolArray, BoolArrayInstance, ByteArray, ByteArrayInstance,
            CharArray, CharArrayInstance, DoubleArray, DoubleArrayInstance,
            FloatArray, FloatArrayInstance, IntArray, IntArrayInstance,
            LongArray, LongArrayInstance, ObjectArray, ObjectArrayInstance,
            ShortArray, ShortArrayInstance,
        },
        Class, ClassInstance, Field, Method,
    },
    classloader::cp_decoder::RuntimeCPEntry,
    executor::{
        frame_stack::StackValue, local_variables::VariableValueOrValue, Frame,
        Update,
    },
    heap::Heap,
};

#[derive(Clone, Debug)]
pub enum ArrayReferenceKinds {
    Boolean,
    Byte,
    Char,
    Double,
    Float,
    Long,
    Int,
    Short,
    Object(Rc<dyn Class>),
}

#[derive(Clone, Debug)]
pub enum Ldc {
    Int(i32),
    Float(f32),
    String(Rc<dyn ClassInstance>),
    Class(Rc<dyn Class>),
    Method(Rc<Method>),
    Long(i64),
    Double(f64),
}

#[derive(Clone, Copy, Debug)]
pub enum OffsetDirection {
    Forward,
    Backward,
}

#[derive(Clone, Debug)]
pub enum FloatCmp {
    Pg,
    Pl,
}

#[derive(Clone, Debug)]
pub enum Dup {
    Dup,
    X1,
    X2,
}

#[derive(Clone, Debug)]
#[repr(u8)]
pub enum ArrayType {
    Boolean = 4,
    Char = 5,
    Float = 6,
    Double = 7,
    Byte = 8,
    Short = 9,
    Int = 10,
    Long = 11,
}

impl ArrayType {
    pub fn from_int(value: u8) -> ArrayType {
        match value {
            4 => ArrayType::Boolean,
            5 => ArrayType::Char,
            6 => ArrayType::Float,
            7 => ArrayType::Double,
            8 => ArrayType::Byte,
            9 => ArrayType::Short,
            10 => ArrayType::Int,
            11 => ArrayType::Long,
            _ => panic!("{} is not a valid ArrayType", value),
        }
    }
}

#[derive(Clone, Debug)]
pub enum OpCode {
    Aaload,
    Aastore,
    AconstNull,
    /// Load reference from `index` in local variable array to stack.
    Aload(usize),
    // "[[I"
    // ArrayObject::new(ArrayObject::new(IntArray))
    //
    // executor: ArrayObject::new(anewarray_conent).new_instance()
    AnewArray(Rc<dyn Class>),
    Areturn,
    ArrayLength,
    /// Store reference to `index` in local variable array from stack.
    Astore(usize),
    Athrow,
    Baload,
    Bastore,
    /// Value to push onto stack
    Bipush(i32),
    Caload,
    Castore,
    Checkcast(Rc<dyn Any>),
    D2f,
    D2i,
    D2l,
    Dadd,
    Daload,
    Dastore,
    Dcmp(FloatCmp),
    Dconst(f64),
    Ddiv,
    Dload(usize),
    Dmul,
    Dneg,
    Drem,
    Dreturn,
    /// Dstore_ are converted,
    /// index into local variables
    Dstore(usize),
    Dsub,
    Dup(Dup),
    Dup2(Dup),
    F2d,
    F2i,
    F2l,
    Fadd,
    Faload,
    Fastore,
    Fcmp(FloatCmp),
    Fconst(f32),
    Fdiv,
    Fload(usize),
    Fmul,
    Fneg,
    Frem,
    Freturn,
    /// Fstore_ are converted,
    /// index into local variables
    Fstore(usize),
    Fsub,
    // TODO(FW): how are instance fields stored
    // to resolve them at execution time
    GetField(Rc<dyn Any>),
    GetStatic(Rc<Field>),
    Goto(usize, OffsetDirection),
    I2b,
    I2c,
    I2d,
    I2f,
    I2l,
    I2s,
    Iadd,
    Iaload,
    Iand,
    Iastore,
    /// Value to push onto stack
    Iconst(i32),
    Idiv,
    IfacmpNe(usize, OffsetDirection),
    IfacmpEq(usize, OffsetDirection),
    IficmpEq(usize, OffsetDirection),
    IficmpNe(usize, OffsetDirection),
    IficmpLt(usize, OffsetDirection),
    IficmpGe(usize, OffsetDirection),
    IficmpGt(usize, OffsetDirection),
    IficmpLe(usize, OffsetDirection),
    IfEq(usize, OffsetDirection),
    IfNe(usize, OffsetDirection),
    IfLt(usize, OffsetDirection),
    IfGe(usize, OffsetDirection),
    IfGt(usize, OffsetDirection),
    IfLe(usize, OffsetDirection),
    IfNonNull(usize, OffsetDirection),
    IfNull(usize, OffsetDirection),
    Iinc {
        index: usize,
        constant: i32,
    },
    /// Iload_ are converted,
    /// index into local variables
    Iload(usize),
    Imul,
    Ineg,
    InstanceOf(Rc<dyn Any>),
    InvokeDynamic(Rc<dyn Any>),
    InvokeInterface(Rc<dyn Any>),
    InvokeSpecial(RuntimeCPEntry), // Placeholder, to enable bytecode parsing
    InvokeStatic(Rc<Method>),
    InvokeVirtual(Rc<Method>),
    Ior,
    Irem,
    Ireturn,
    Ishl,
    Ishr,
    /// Istore_ are converted,
    /// index into local variables
    Istore(usize),
    Isub,
    Iushr,
    Ixor,
    Jsr(usize, OffsetDirection),
    L2d,
    L2f,
    L2i,
    Ladd,
    Laload,
    Land,
    Lastore,
    Lcmp,
    Lconst(i64),
    Ldc(Ldc),
    Ldiv,
    Lload(usize),
    Lmul,
    Lneg,
    // unsupported for now
    Lookupswitch,
    Lor,
    Lrem,
    Lreturn,
    Lshl,
    Lshr,
    /// Lstore_ are converted,
    /// index into local variables
    Lstore(usize),
    Lsub,
    Lushr,
    Lxor,
    // definitely unsupported
    Monitorenter,
    // definitely unsupported
    Monitorexit,
    MultiAnewArray {
        reference_kind: ArrayReferenceKinds,
        dimensions: u8,
    },
    New(Rc<dyn Any>),
    NewArray(ArrayType),
    Nop,
    Pop,
    Pop2,
    // TODO(FW): how are instance fields stored
    // to resolve them at execution time
    PutField(Rc<dyn Any>),
    PutStatic(Rc<Field>),
    Ret(usize),
    Return,
    Saload,
    Sastore,
    Sipush(i16),
    Swap,
    // unsupported for now
    Tableswitch,
    // Wide: not needed,
    // since it only effects how indicies are parsed from the byteocde.
    // This has already happened for any values represented by this enum.
}

impl OpCode {
    pub fn execute(&self, frame: &mut Frame, heap: &mut Heap) -> Update {
        match self {
            Self::Aaload => {
                let index = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();
                let array: Rc<dyn ClassInstance> =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                match array.as_any().downcast_ref::<ObjectArrayInstance>() {
                    Some(obj_array) => {
                        let obj =
                            obj_array.get(index.try_into().unwrap()).unwrap();
                        frame
                            .operand_stack
                            .push(StackValue::Reference(obj.clone()))
                            .unwrap();
                    },
                    None => panic!(
                        "Expected object array or multidimensional array \
on top of the stack, got: {:?}",
                        array
                    ),
                }
                Update::None
            },
            Self::Aastore => {
                let value: Option<Rc<dyn ClassInstance>> =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let index = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();
                let array: Rc<dyn ClassInstance> =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let array: &ObjectArrayInstance =
                    array.as_ref().try_into().unwrap();
                array.set(index.try_into().unwrap(), value).unwrap();
                Update::None
            },
            Self::Aload(index) => {
                frame
                    .operand_stack
                    .push(frame.local_variables.get(*index).into())
                    .unwrap();
                Update::None
            },
            Self::AnewArray(scalar_class) => {
                let size = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();

                // get underlying component class for array
                let array_cls = if scalar_class.name().starts_with('[') {
                    // array
                    let (array_dim, kind) =
                        scalar_class.name().rsplit_once('[').unwrap();
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
                            scalar_class.name()
                        ),
                    };

                    // +1 for removed dim in rsplit
                    // +1 for implicit dim in op-code anewarray
                    let dim = array_dim.len() + 2;

                    heap.find_array_class(scalar_class, dim.try_into().unwrap())
                } else {
                    // object
                    heap.find_array_class(
                        ArrayReferenceKinds::Object(scalar_class.clone()),
                        1,
                    )
                }
                .unwrap();

                // construct new array and put on stack
                let array_cls_for_ref = array_cls.clone();
                let array_ref: &ObjectArray =
                    array_cls_for_ref.as_ref().try_into().unwrap();
                let array_inst = array_ref
                    .new_instance_from_ref(size.try_into().unwrap(), array_cls)
                    .unwrap();
                frame
                    .operand_stack
                    .push(StackValue::Reference(Some(Rc::new(array_inst))))
                    .unwrap();

                Update::None
            },

            Self::ArrayLength => {
                let stack_value: Rc<dyn ClassInstance> =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();

                let length = if let Ok(array) =
                    TryInto::<&ObjectArrayInstance>::try_into(
                        stack_value.as_ref(),
                    ) {
                    array.length()
                } else if let Ok(array) =
                    TryInto::<&BoolArrayInstance>::try_into(
                        stack_value.as_ref(),
                    )
                {
                    array.length()
                } else if let Ok(array) =
                    TryInto::<&ByteArrayInstance>::try_into(
                        stack_value.as_ref(),
                    )
                {
                    array.length()
                } else if let Ok(array) =
                    TryInto::<&CharArrayInstance>::try_into(
                        stack_value.as_ref(),
                    )
                {
                    array.length()
                } else if let Ok(array) =
                    TryInto::<&DoubleArrayInstance>::try_into(
                        stack_value.as_ref(),
                    )
                {
                    array.length()
                } else if let Ok(array) =
                    TryInto::<&FloatArrayInstance>::try_into(
                        stack_value.as_ref(),
                    )
                {
                    array.length()
                } else if let Ok(array) =
                    TryInto::<&LongArrayInstance>::try_into(
                        stack_value.as_ref(),
                    )
                {
                    array.length()
                } else if let Ok(array) =
                    TryInto::<&IntArrayInstance>::try_into(stack_value.as_ref())
                {
                    array.length()
                } else if let Ok(array) =
                    TryInto::<&ShortArrayInstance>::try_into(
                        stack_value.as_ref(),
                    )
                {
                    array.length()
                } else {
                    panic!(
                        "expected array on top of stack, got: {:?}",
                        stack_value
                    )
                };

                frame
                    .operand_stack
                    .push(StackValue::Int(length.try_into().unwrap()))
                    .unwrap();

                Update::None
            },

            Self::Baload => {
                let index = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();
                let array: Rc<dyn ClassInstance> =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();

                if let Some(byte_array) =
                    array.as_any().downcast_ref::<ByteArrayInstance>()
                {
                    let obj =
                        byte_array.get(index.try_into().unwrap()).unwrap();
                    frame.operand_stack.push(StackValue::Byte(obj)).unwrap();
                } else if let Some(bool_array) =
                    array.as_any().downcast_ref::<BoolArrayInstance>()
                {
                    let obj =
                        bool_array.get(index.try_into().unwrap()).unwrap();
                    frame
                        .operand_stack
                        .push(StackValue::Boolean(if obj { 1 } else { 0 }))
                        .unwrap();
                } else {
                    panic!(
                        "Expected byte or bool array on top of the stack, \
got: {:?}",
                        array
                    )
                }

                Update::None
            },

            Self::Bastore => {
                let value: i32 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();
                let index = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();
                let array: Rc<dyn ClassInstance> =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();

                if let Some(byte_array) =
                    array.as_any().downcast_ref::<ByteArrayInstance>()
                {
                    byte_array
                        .set(
                            index.try_into().unwrap(),
                            value.try_into().unwrap(),
                        )
                        .unwrap();
                } else if let Some(bool_array) =
                    array.as_any().downcast_ref::<BoolArrayInstance>()
                {
                    bool_array
                        .set(index.try_into().unwrap(), value == 1)
                        .unwrap();
                } else {
                    panic!(
                        "Expected byte or bool array on top of the stack, \
got: {:?}",
                        array
                    )
                }

                Update::None
            },

            Self::Bipush(v) => {
                frame.operand_stack.push(StackValue::Int(*v)).unwrap();
                Update::None
            },

            Self::Caload => {
                let index = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();
                let array: Rc<dyn ClassInstance> =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let char_array: &CharArrayInstance =
                    array.as_ref().try_into().unwrap();

                let obj = char_array.get(index.try_into().unwrap()).unwrap();
                frame.operand_stack.push(StackValue::Char(obj)).unwrap();

                Update::None
            },

            Self::Castore => {
                let value: StackValue = frame.operand_stack.pop().unwrap();
                let index = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();
                let array: Rc<dyn ClassInstance> =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();

                let char_value = value.as_computation_int().unwrap();
                let char_array: &CharArrayInstance =
                    array.as_ref().try_into().unwrap();
                char_array
                    .set(
                        index.try_into().unwrap(),
                        char_value.try_into().unwrap(),
                    )
                    .unwrap();
                Update::None
            },

            Self::Daload => {
                let index = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();
                let array: Rc<dyn ClassInstance> =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let array: &DoubleArrayInstance =
                    array.as_ref().try_into().unwrap();

                let obj = array.get(index.try_into().unwrap()).unwrap();
                frame.operand_stack.push(StackValue::Double(obj)).unwrap();

                Update::None
            },

            Self::Dastore => {
                let value: StackValue = frame.operand_stack.pop().unwrap();
                let index = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();
                let array: Rc<dyn ClassInstance> =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();

                let value = if let StackValue::Double(value) = value {
                    value
                } else {
                    panic!(
                        "Expected double value on top of the stack, \
got: {:?}",
                        value
                    );
                };
                let array: &DoubleArrayInstance =
                    array.as_ref().try_into().unwrap();
                array.set(index.try_into().unwrap(), value).unwrap();
                Update::None
            },

            // note: split this into multiple cases,
            // in case the types are supposed to be verified
            Self::Dload(index)
            | Self::Fload(index)
            | Self::Iload(index)
            | Self::Lload(index) => {
                frame
                    .operand_stack
                    .push(frame.local_variables.get(*index).into())
                    .unwrap();
                Update::None
            },

            // note: split this into multiple cases,
            // in case the types are supposed to be verified
            Self::Dstore(index)
            | Self::Fstore(index)
            | Self::Istore(index)
            | Self::Lstore(index) => {
                frame
                    .local_variables
                    .set(*index, frame.operand_stack.pop().unwrap().into());
                Update::None
            },

            Self::Faload => {
                let index = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();
                let array: Rc<dyn ClassInstance> =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let array: &FloatArrayInstance =
                    array.as_ref().try_into().unwrap();

                let obj = array.get(index.try_into().unwrap()).unwrap();
                frame.operand_stack.push(StackValue::Float(obj)).unwrap();

                Update::None
            },

            Self::Fastore => {
                let value: StackValue = frame.operand_stack.pop().unwrap();
                let index = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();
                let array: Rc<dyn ClassInstance> =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();

                let value = if let StackValue::Float(value) = value {
                    value
                } else {
                    panic!(
                        "Expected float value on top of the stack, \
got: {:?}",
                        value
                    );
                };
                let array: &FloatArrayInstance =
                    array.as_ref().try_into().unwrap();
                array.set(index.try_into().unwrap(), value).unwrap();
                Update::None
            },

            Self::GetStatic(field) => {
                frame
                    .operand_stack
                    .push(field.value.clone().into())
                    .unwrap();
                Update::None
            },

            Self::Iaload => {
                let index = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();
                let array: Rc<dyn ClassInstance> =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let array: &IntArrayInstance =
                    array.as_ref().try_into().unwrap();

                let obj = array.get(index.try_into().unwrap()).unwrap();
                frame.operand_stack.push(StackValue::Int(obj)).unwrap();

                Update::None
            },

            Self::Iadd => {
                let op2 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();
                let op1 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();

                let result: i32 = op1.wrapping_add(op2);

                // result is always int,
                // for byte/short/etc it will be explicitly casted
                // by the following bytecode op
                // (generated by compiler)
                frame.operand_stack.push(StackValue::Int(result)).unwrap();

                Update::None
            },

            Self::Iastore => {
                let value: StackValue = frame.operand_stack.pop().unwrap();
                let index = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();
                let array: Rc<dyn ClassInstance> =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();

                let value = value.as_computation_int().unwrap();
                let array: &IntArrayInstance =
                    array.as_ref().try_into().unwrap();
                array.set(index.try_into().unwrap(), value).unwrap();
                Update::None
            },

            Self::Iand => {
                let op2 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();
                let op1 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();

                let result: i32 = op1 & op2;

                // result is always int,
                // for byte/short/etc it will be explicitly casted
                // by the following bytecode op
                // (generated by compiler)
                frame.operand_stack.push(StackValue::Int(result)).unwrap();

                Update::None
            },

            Self::Iconst(val) => {
                frame.operand_stack.push(StackValue::Int(*val)).unwrap();
                Update::None
            },

            Self::Idiv => {
                let op2 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();
                let op1 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();

                // TODO(FW): this panics for division by 0
                let result: i32 = op1.wrapping_div(op2);

                // result is always int,
                // for byte/short/etc it will be explicitly casted
                // by the following bytecode op
                // (generated by compiler)
                frame.operand_stack.push(StackValue::Int(result)).unwrap();

                Update::None
            },

            Self::Iinc { index, constant } => {
                let op1 = frame.local_variables.get(*index);
                let op1 = match op1 {
                    VariableValueOrValue::Int(i) => i,
                    _ => panic!("expect int at {} got '{:?}'", index, op1),
                };

                let result: i32 = op1 + constant;

                frame
                    .local_variables
                    .set(*index, VariableValueOrValue::Int(result));

                Update::None
            },

            Self::Imul => {
                let op2 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();
                let op1 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();

                let result: i32 = op1.wrapping_mul(op2);

                frame.operand_stack.push(StackValue::Int(result)).unwrap();

                Update::None
            },

            Self::Ineg => {
                let op1 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();

                let result: i32 = op1.neg();

                frame.operand_stack.push(StackValue::Int(result)).unwrap();

                Update::None
            },

            Self::InvokeVirtual(method) => Update::MethodCall(method.clone()),

            Self::Ior => {
                let op2 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();
                let op1 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();

                let result: i32 = op1 | op2;

                frame.operand_stack.push(StackValue::Int(result)).unwrap();

                Update::None
            },

            Self::Irem => {
                let op2 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();
                let op1 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();

                let result: i32 = op1.wrapping_rem(op2);

                frame.operand_stack.push(StackValue::Int(result)).unwrap();

                Update::None
            },

            Self::Ishl => {
                let op2 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();
                let op1 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();

                let result: i32 = op1 << (op2 & 0x1f);

                frame.operand_stack.push(StackValue::Int(result)).unwrap();

                Update::None
            },

            Self::Ishr => {
                let op2 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();
                let op1 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();

                let result: i32 = op1 >> (op2 & 0x1f);

                frame.operand_stack.push(StackValue::Int(result)).unwrap();

                Update::None
            },

            Self::Isub => {
                let op2 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();
                let op1 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();

                let result: i32 = op1.wrapping_sub(op2);

                frame.operand_stack.push(StackValue::Int(result)).unwrap();

                Update::None
            },

            Self::Iushr => {
                let op2 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();
                let op1 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();

                // perform shift with zero extension
                // by casting to an unsigned value before the shift
                let result: i32 = ((op1 as u32) >> (op2 & 0x1f)) as i32;

                frame.operand_stack.push(StackValue::Int(result)).unwrap();

                Update::None
            },

            Self::Ixor => {
                let op2 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();
                let op1 = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();

                let result: i32 = op1 ^ op2;

                frame.operand_stack.push(StackValue::Int(result)).unwrap();

                Update::None
            },

            Self::I2b => {
                let int = frame.operand_stack.pop().expect("stack has value");
                let int = match int {
                    StackValue::Int(i) => i,
                    _ => panic!("expect int stack value, got '{:?}'", int),
                };
                let byte: i8 = int as i8;
                frame.operand_stack.push(StackValue::Byte(byte)).unwrap();
                Update::None
            },
            Self::I2c => {
                let int = frame.operand_stack.pop().expect("stack has value");
                let int = match int {
                    StackValue::Int(i) => i,
                    _ => panic!("expect int stack value, got '{:?}'", int),
                };
                let c: u16 = int as u16;
                frame.operand_stack.push(StackValue::Char(c)).unwrap();
                Update::None
            },
            Self::I2d => {
                let int = frame.operand_stack.pop().expect("stack has value");
                let int = match int {
                    StackValue::Int(i) => i,
                    _ => panic!("expect int stack value, got '{:?}'", int),
                };
                let double: f64 = int.into();
                frame
                    .operand_stack
                    .push(StackValue::Double(double))
                    .unwrap();
                Update::None
            },
            Self::I2f => {
                let int = frame.operand_stack.pop().expect("stack has value");
                let int = match int {
                    StackValue::Int(i) => i,
                    _ => panic!("expect int stack value, got '{:?}'", int),
                };
                let float: f32 = int as f32;
                frame.operand_stack.push(StackValue::Float(float)).unwrap();
                Update::None
            },
            Self::I2l => {
                let int = frame.operand_stack.pop().expect("stack has value");
                let int = match int {
                    StackValue::Int(i) => i,
                    _ => panic!("expect int stack value, got '{:?}'", int),
                };
                let long: i64 = int.into();
                frame.operand_stack.push(StackValue::Long(long)).unwrap();
                Update::None
            },
            Self::I2s => {
                let int = frame.operand_stack.pop().expect("stack has value");
                let int = match int {
                    StackValue::Int(i) => i,
                    _ => panic!("expect int stack value, got '{:?}'", int),
                };
                let short: i16 = int as i16;
                frame.operand_stack.push(StackValue::Short(short)).unwrap();
                Update::None
            },

            Self::MultiAnewArray {
                reference_kind,
                dimensions,
            } => {
                let mut array_lens: Vec<usize> = vec![0]; // 1-based

                // stack contains as many counts as there are dimensions
                // inner most dimension is on top of stack, outer most on bottom
                for _ in 0..*dimensions {
                    array_lens.push(
                        frame
                            .operand_stack
                            .pop()
                            .unwrap()
                            .as_computation_int()
                            .unwrap()
                            .try_into()
                            .unwrap(),
                    );
                }

                let outer_array = init_array(
                    heap,
                    reference_kind.clone(),
                    *dimensions,
                    array_lens[Into::<usize>::into(*dimensions)],
                );
                init_array_rec(
                    heap,
                    reference_kind.clone(),
                    &array_lens,
                    *dimensions,
                    outer_array.clone(),
                );

                frame
                    .operand_stack
                    .push(StackValue::Reference(Some(outer_array)))
                    .unwrap();

                Update::None
            },

            Self::Laload => {
                let index = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();
                let array: Rc<dyn ClassInstance> =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let array: &LongArrayInstance =
                    array.as_ref().try_into().unwrap();

                let obj = array.get(index.try_into().unwrap()).unwrap();
                frame.operand_stack.push(StackValue::Long(obj)).unwrap();

                Update::None
            },

            Self::Lastore => {
                let value: StackValue = frame.operand_stack.pop().unwrap();
                let index = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();
                let array: Rc<dyn ClassInstance> =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();

                let value = if let StackValue::Long(value) = value {
                    value
                } else {
                    panic!(
                        "Expected long value on top of the stack, \
got: {:?}",
                        value
                    );
                };
                let array: &LongArrayInstance =
                    array.as_ref().try_into().unwrap();
                array.set(index.try_into().unwrap(), value).unwrap();
                Update::None
            },

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

            Self::NewArray(array_type) => {
                let size = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();

                let array: Rc<dyn ClassInstance> = match array_type {
                    ArrayType::Boolean => {
                        let array =
                            heap.new_boolean_array(size.try_into().unwrap());
                        Rc::new(array)
                    },

                    ArrayType::Char => {
                        let array =
                            heap.new_char_array(size.try_into().unwrap());
                        Rc::new(array)
                    },

                    ArrayType::Float => {
                        let array =
                            heap.new_float_array(size.try_into().unwrap());
                        Rc::new(array)
                    },

                    ArrayType::Double => {
                        let array =
                            heap.new_double_array(size.try_into().unwrap());
                        Rc::new(array)
                    },

                    ArrayType::Byte => {
                        let array =
                            heap.new_byte_array(size.try_into().unwrap());
                        Rc::new(array)
                    },

                    ArrayType::Short => {
                        let array =
                            heap.new_short_array(size.try_into().unwrap());
                        Rc::new(array)
                    },

                    ArrayType::Int => {
                        let array =
                            heap.new_int_array(size.try_into().unwrap());
                        Rc::new(array)
                    },

                    ArrayType::Long => {
                        let array =
                            heap.new_long_array(size.try_into().unwrap());
                        Rc::new(array)
                    },
                };

                frame
                    .operand_stack
                    .push(StackValue::Reference(Some(array)))
                    .unwrap();

                Update::None
            },

            Self::Return => Update::Return,

            Self::Saload => {
                let index = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();
                let array: Rc<dyn ClassInstance> =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let array: &ShortArrayInstance =
                    array.as_ref().try_into().unwrap();

                let obj = array.get(index.try_into().unwrap()).unwrap();
                frame.operand_stack.push(StackValue::Short(obj)).unwrap();

                Update::None
            },

            Self::Sastore => {
                let value: StackValue = frame.operand_stack.pop().unwrap();
                let index = frame
                    .operand_stack
                    .pop()
                    .unwrap()
                    .as_computation_int()
                    .unwrap();
                let array: Rc<dyn ClassInstance> =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();

                let value = value.as_computation_int().unwrap();
                let array: &ShortArrayInstance =
                    array.as_ref().try_into().unwrap();
                array
                    .set(index.try_into().unwrap(), value.try_into().unwrap())
                    .unwrap();
                Update::None
            },

            _ => todo!("Missing OpCode implementation for: {:?}", self),
        }
    }
}

/// Create new array with the type given by dim_count and component_type.
/// Only used in opcode multianewarray.
fn init_array(
    heap: &mut Heap,
    component_type: ArrayReferenceKinds,
    dim_count: u8,
    array_len: usize,
) -> Rc<dyn ClassInstance> {
    let array_cls = heap
        .find_array_class(component_type.clone(), dim_count)
        .unwrap();
    if dim_count > 1 || matches!(component_type, ArrayReferenceKinds::Object(_))
    {
        // array_cls must be an obj array
        // construct new array
        let array_cls_for_ref = array_cls.clone();
        let array_ref: &ObjectArray =
            array_cls_for_ref.as_ref().try_into().unwrap();
        let array_inst = array_ref
            .new_instance_from_ref(array_len, array_cls)
            .unwrap();

        Rc::new(array_inst)
    } else {
        // array_cls must be a primitive array
        // construct new array based on actual component_type
        match component_type {
            ArrayReferenceKinds::Boolean => {
                let array_cls_for_ref = array_cls.clone();
                let array_ref: &BoolArray =
                    array_cls_for_ref.as_ref().try_into().unwrap();
                let array_inst = array_ref
                    .new_instance_from_ref(array_len, array_cls)
                    .unwrap();

                Rc::new(array_inst)
            },
            ArrayReferenceKinds::Byte => {
                let array_cls_for_ref = array_cls.clone();
                let array_ref: &ByteArray =
                    array_cls_for_ref.as_ref().try_into().unwrap();
                let array_inst = array_ref
                    .new_instance_from_ref(array_len, array_cls)
                    .unwrap();

                Rc::new(array_inst)
            },
            ArrayReferenceKinds::Char => {
                let array_cls_for_ref = array_cls.clone();
                let array_ref: &CharArray =
                    array_cls_for_ref.as_ref().try_into().unwrap();
                let array_inst = array_ref
                    .new_instance_from_ref(array_len, array_cls)
                    .unwrap();

                Rc::new(array_inst)
            },
            ArrayReferenceKinds::Double => {
                let array_cls_for_ref = array_cls.clone();
                let array_ref: &DoubleArray =
                    array_cls_for_ref.as_ref().try_into().unwrap();
                let array_inst = array_ref
                    .new_instance_from_ref(array_len, array_cls)
                    .unwrap();

                Rc::new(array_inst)
            },
            ArrayReferenceKinds::Float => {
                let array_cls_for_ref = array_cls.clone();
                let array_ref: &FloatArray =
                    array_cls_for_ref.as_ref().try_into().unwrap();
                let array_inst = array_ref
                    .new_instance_from_ref(array_len, array_cls)
                    .unwrap();

                Rc::new(array_inst)
            },
            ArrayReferenceKinds::Long => {
                let array_cls_for_ref = array_cls.clone();
                let array_ref: &LongArray =
                    array_cls_for_ref.as_ref().try_into().unwrap();
                let array_inst = array_ref
                    .new_instance_from_ref(array_len, array_cls)
                    .unwrap();

                Rc::new(array_inst)
            },
            ArrayReferenceKinds::Int => {
                let array_cls_for_ref = array_cls.clone();
                let array_ref: &IntArray =
                    array_cls_for_ref.as_ref().try_into().unwrap();
                let array_inst = array_ref
                    .new_instance_from_ref(array_len, array_cls)
                    .unwrap();

                Rc::new(array_inst)
            },
            ArrayReferenceKinds::Short => {
                let array_cls_for_ref = array_cls.clone();
                let array_ref: &ShortArray =
                    array_cls_for_ref.as_ref().try_into().unwrap();
                let array_inst = array_ref
                    .new_instance_from_ref(array_len, array_cls)
                    .unwrap();

                Rc::new(array_inst)
            },
            ArrayReferenceKinds::Object(_) => {
                unreachable!("should be handled by if case above")
            },
        }
    }
}

/// Initialize given multidim array by recursively creating multidim arrays
/// in all sub-dimensions with the type given by dim and component_type.
/// Only used in opcode multianewarray.
/// array_lens is indexed 1-based!
fn init_array_rec(
    heap: &mut Heap,
    component_type: ArrayReferenceKinds,
    array_lens: &[usize],
    dim: u8,
    outer_array: Rc<dyn ClassInstance>,
) {
    if dim == 0 {
        // rec exit
        return;
    }

    let outer_array: &ObjectArrayInstance =
        outer_array.as_ref().try_into().unwrap();
    for i in 0..array_lens[Into::<usize>::into(dim)] {
        let inner_array = init_array(
            heap,
            component_type.clone(),
            dim - 1,
            array_lens[Into::<usize>::into(dim - 1)],
        );
        init_array_rec(
            heap,
            component_type.clone(),
            array_lens,
            dim - 1,
            inner_array.clone(),
        ); // rec enter
        outer_array.set(i, Some(inner_array)).unwrap();
    }
}
