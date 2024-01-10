use std::{any::Any, cmp::Ordering, ops::Neg, rc::Rc};

use crate::{
    class::{
        builtin_classes::array::{
            BoolArray, BoolArrayInstance, ByteArray, ByteArrayInstance,
            CharArray, CharArrayInstance, DoubleArray, DoubleArrayInstance,
            FloatArray, FloatArrayInstance, IntArray, IntArrayInstance,
            LongArray, LongArrayInstance, ObjectArray, ObjectArrayInstance,
            ShortArray, ShortArrayInstance,
        },
        ArgumentKind, ArrayName, Class, ClassIdentifier, ClassInstance,
        ClassName, Field, FieldValue, Method,
    },
    executor::{
        frame_stack::StackValue, local_variables::VariableValueOrValue, Frame,
        ReturnValue, Update,
    },
    heap::Heap,
};

/// Explicitly compare only the data part of fat/trait/dyn Trait pointers.
#[allow(clippy::ptr_eq)]
fn trait_pointer_eq<T: ?Sized, U: ?Sized>(p: *const T, q: *const U) -> bool {
    (p as *const ()) == (q as *const ())
}

#[derive(Clone, Debug)]
pub struct MethodDescriptor {
    pub name: String,
    pub descriptor: (Vec<ArgumentKind>, Option<ArgumentKind>),
}

#[derive(Clone, Debug)]
pub struct SymbolicMethod {
    pub class_name: ClassIdentifier,
    pub descriptor: MethodDescriptor,
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
    GetField {
        class: ClassIdentifier,
        field_name: String,
    },
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
    InvokeSpecial(SymbolicMethod),
    InvokeStatic(SymbolicMethod),
    InvokeVirtual(SymbolicMethod),
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
    MultiAnewArray(ClassIdentifier),
    // classname
    New(ClassIdentifier),
    NewArray(ArrayType),
    Nop,
    Pop,
    Pop2,
    PutField {
        class: ClassIdentifier,
        field_name: String,
    },
    PutStatic(Rc<Field>),
    Ret(usize),
    Return,
    Saload,
    Sastore,
    Sipush(i32),
    Swap,
    // unsupported for now
    Tableswitch,
    // Wide: not needed,
    // since it only effects how indicies are parsed from the byteocde.
    // This has already happened for any values represented by this enum.
}

impl OpCode {
    pub fn execute(
        &self,
        frame: &mut Frame,
        heap: &mut Heap,
        _current_class: &Rc<dyn Class>,
    ) -> Update {
        match self {
            Self::Aaload => {
                let index: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
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
                let index: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
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
            Self::AnewArray(array_cls) => {
                let size: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();

                // construct new array and put on stack
                let array_cls_for_ref = array_cls.clone();
                let array_ref: &ObjectArray =
                    array_cls_for_ref.as_ref().try_into().unwrap();
                let array_inst = array_ref
                    .new_instance_from_ref(
                        size.try_into().unwrap(),
                        array_cls.clone(),
                    )
                    .unwrap();
                frame
                    .operand_stack
                    .push(StackValue::Reference(Some(Rc::new(array_inst))))
                    .unwrap();

                Update::None
            },

            Self::Areturn => {
                let retval: Option<Rc<dyn ClassInstance>> =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();

                Update::Return(ReturnValue::Reference(retval))
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

            Self::Athrow => {
                let exception: Rc<dyn ClassInstance> =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();

                Update::Exception(exception)
            },

            Self::Baload => {
                let index: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let array: Rc<dyn ClassInstance> =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();

                if let Some(byte_array) =
                    array.as_any().downcast_ref::<ByteArrayInstance>()
                {
                    let obj =
                        byte_array.get(index.try_into().unwrap()).unwrap();
                    frame
                        .operand_stack
                        .push(StackValue::Int(obj.into()))
                        .unwrap();
                } else if let Some(bool_array) =
                    array.as_any().downcast_ref::<BoolArrayInstance>()
                {
                    let obj =
                        bool_array.get(index.try_into().unwrap()).unwrap();
                    frame
                        .operand_stack
                        .push(StackValue::Int(if obj { 1 } else { 0 }))
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
                let value: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let index: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
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

            Self::Bipush(v) | Self::Sipush(v) => {
                frame.operand_stack.push(StackValue::Int(*v)).unwrap();
                Update::None
            },

            Self::Caload => {
                let index: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let array: Rc<dyn ClassInstance> =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let char_array: &CharArrayInstance =
                    array.as_ref().try_into().unwrap();

                let obj = char_array.get(index.try_into().unwrap()).unwrap();
                frame
                    .operand_stack
                    .push(StackValue::Int(obj.into()))
                    .unwrap();

                Update::None
            },

            Self::Castore => {
                let value: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let index: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let array: Rc<dyn ClassInstance> =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();

                let char_array: &CharArrayInstance =
                    array.as_ref().try_into().unwrap();
                char_array
                    .set(index.try_into().unwrap(), value.try_into().unwrap())
                    .unwrap();
                Update::None
            },

            Self::D2f => {
                let val = if let StackValue::Double(d) =
                    frame.operand_stack.pop().unwrap()
                {
                    d
                } else {
                    panic!("expected double on top");
                };

                frame
                    .operand_stack
                    .push(StackValue::Float(val as f32))
                    .unwrap();

                Update::None
            },

            Self::D2i => {
                let val = if let StackValue::Double(d) =
                    frame.operand_stack.pop().unwrap()
                {
                    d
                } else {
                    panic!("expected double on top");
                };

                frame
                    .operand_stack
                    .push(StackValue::Int(val as i32))
                    .unwrap();

                Update::None
            },

            Self::D2l => {
                let val = if let StackValue::Double(d) =
                    frame.operand_stack.pop().unwrap()
                {
                    d
                } else {
                    panic!("expected double on top");
                };

                frame
                    .operand_stack
                    .push(StackValue::Long(val as i64))
                    .unwrap();

                Update::None
            },

            Self::Dadd => {
                let op2 = if let StackValue::Double(d) =
                    frame.operand_stack.pop().unwrap()
                {
                    d
                } else {
                    panic!("expected double on top");
                };
                let op1 = if let StackValue::Double(d) =
                    frame.operand_stack.pop().unwrap()
                {
                    d
                } else {
                    panic!("expected double on top");
                };

                frame
                    .operand_stack
                    .push(StackValue::Double(op1 + op2))
                    .unwrap();

                Update::None
            },

            Self::Daload => {
                let index: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
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
                let index: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
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

            Self::Dcmp(nan_handling) => {
                let op2 = if let StackValue::Double(d) =
                    frame.operand_stack.pop().unwrap()
                {
                    d
                } else {
                    panic!("expected double on top");
                };
                let op1 = if let StackValue::Double(d) =
                    frame.operand_stack.pop().unwrap()
                {
                    d
                } else {
                    panic!("expected double on top");
                };

                match op1.partial_cmp(&op2) {
                    Some(Ordering::Greater) => {
                        frame.operand_stack.push(StackValue::Int(1)).unwrap();
                    },
                    Some(Ordering::Equal) => {
                        frame.operand_stack.push(StackValue::Int(0)).unwrap();
                    },
                    Some(Ordering::Less) => {
                        frame.operand_stack.push(StackValue::Int(-1)).unwrap();
                    },
                    None => match nan_handling {
                        FloatCmp::Pg => frame
                            .operand_stack
                            .push(StackValue::Int(1))
                            .unwrap(),
                        FloatCmp::Pl => frame
                            .operand_stack
                            .push(StackValue::Int(-1))
                            .unwrap(),
                    },
                }

                Update::None
            },

            Self::Dconst(d) => {
                frame.operand_stack.push(StackValue::Double(*d)).unwrap();

                Update::None
            },

            Self::Ddiv => {
                let op2 = if let StackValue::Double(d) =
                    frame.operand_stack.pop().unwrap()
                {
                    d
                } else {
                    panic!("expected double on top");
                };
                let op1 = if let StackValue::Double(d) =
                    frame.operand_stack.pop().unwrap()
                {
                    d
                } else {
                    panic!("expected double on top");
                };

                frame
                    .operand_stack
                    .push(StackValue::Double(op1 / op2))
                    .unwrap();

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

            Self::Dmul => {
                let op2 = if let StackValue::Double(d) =
                    frame.operand_stack.pop().unwrap()
                {
                    d
                } else {
                    panic!("expected double on top");
                };
                let op1 = if let StackValue::Double(d) =
                    frame.operand_stack.pop().unwrap()
                {
                    d
                } else {
                    panic!("expected double on top");
                };

                frame
                    .operand_stack
                    .push(StackValue::Double(op1 * op2))
                    .unwrap();

                Update::None
            },

            Self::Dneg => {
                let val = if let StackValue::Double(d) =
                    frame.operand_stack.pop().unwrap()
                {
                    d
                } else {
                    panic!("expected double on top");
                };

                frame.operand_stack.push(StackValue::Double(-val)).unwrap();

                Update::None
            },

            Self::Drem => {
                let op2 = if let StackValue::Double(d) =
                    frame.operand_stack.pop().unwrap()
                {
                    d
                } else {
                    panic!("expected double on top");
                };
                let op1 = if let StackValue::Double(d) =
                    frame.operand_stack.pop().unwrap()
                {
                    d
                } else {
                    panic!("expected double on top");
                };

                frame
                    .operand_stack
                    .push(StackValue::Double(op1 % op2))
                    .unwrap();

                Update::None
            },

            Self::Dreturn => {
                let retval = if let StackValue::Double(d) =
                    frame.operand_stack.pop().unwrap()
                {
                    d
                } else {
                    panic!("expected double on top");
                };

                Update::Return(ReturnValue::Double(retval))
            },

            // note: split this into multiple cases,
            // in case the types are supposed to be verified
            Self::Astore(index)
            | Self::Dstore(index)
            | Self::Fstore(index)
            | Self::Istore(index)
            | Self::Lstore(index) => {
                frame
                    .local_variables
                    .set(*index, frame.operand_stack.pop().unwrap().into());
                Update::None
            },

            Self::Dsub => {
                let op2 = if let StackValue::Double(d) =
                    frame.operand_stack.pop().unwrap()
                {
                    d
                } else {
                    panic!("expected double on top");
                };
                let op1 = if let StackValue::Double(d) =
                    frame.operand_stack.pop().unwrap()
                {
                    d
                } else {
                    panic!("expected double on top");
                };

                frame
                    .operand_stack
                    .push(StackValue::Double(op1 - op2))
                    .unwrap();

                Update::None
            },

            Self::Dup(Dup::Dup) => {
                let val = frame.operand_stack.pop().unwrap();
                frame.operand_stack.push(val.clone()).unwrap();
                frame.operand_stack.push(val).unwrap();
                Update::None
            },

            Self::F2d => {
                let val = if let StackValue::Float(f) =
                    frame.operand_stack.pop().unwrap()
                {
                    f
                } else {
                    panic!("float on stack");
                };

                frame
                    .operand_stack
                    .push(StackValue::Double(val.into()))
                    .unwrap();

                Update::None
            },

            Self::F2i => {
                let val = if let StackValue::Float(f) =
                    frame.operand_stack.pop().unwrap()
                {
                    f
                } else {
                    panic!("float on stack");
                };

                frame
                    .operand_stack
                    .push(StackValue::Int(val as i32))
                    .unwrap();

                Update::None
            },

            Self::F2l => {
                let val = if let StackValue::Float(f) =
                    frame.operand_stack.pop().unwrap()
                {
                    f
                } else {
                    panic!("float on stack");
                };

                frame
                    .operand_stack
                    .push(StackValue::Long(val as i64))
                    .unwrap();

                Update::None
            },

            Self::Fadd => {
                let op2 = if let StackValue::Float(f) =
                    frame.operand_stack.pop().unwrap()
                {
                    f
                } else {
                    panic!("float on top of stack");
                };
                let op1 = if let StackValue::Float(f) =
                    frame.operand_stack.pop().unwrap()
                {
                    f
                } else {
                    panic!("float on top of stack");
                };

                frame
                    .operand_stack
                    .push(StackValue::Float(op1 + op2))
                    .unwrap();

                Update::None
            },

            Self::Faload => {
                let index: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
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
                let index: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
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

            Self::Fcmp(nan_handling) => {
                let op2 = if let StackValue::Float(d) =
                    frame.operand_stack.pop().unwrap()
                {
                    d
                } else {
                    panic!("expected float on top");
                };
                let op1 = if let StackValue::Float(d) =
                    frame.operand_stack.pop().unwrap()
                {
                    d
                } else {
                    panic!("expected float on top");
                };

                match op1.partial_cmp(&op2) {
                    Some(Ordering::Greater) => {
                        frame.operand_stack.push(StackValue::Int(1)).unwrap();
                    },
                    Some(Ordering::Equal) => {
                        frame.operand_stack.push(StackValue::Int(0)).unwrap();
                    },
                    Some(Ordering::Less) => {
                        frame.operand_stack.push(StackValue::Int(-1)).unwrap();
                    },
                    None => match nan_handling {
                        FloatCmp::Pg => frame
                            .operand_stack
                            .push(StackValue::Int(1))
                            .unwrap(),
                        FloatCmp::Pl => frame
                            .operand_stack
                            .push(StackValue::Int(-1))
                            .unwrap(),
                    },
                }

                Update::None
            },

            Self::Fconst(f) => {
                frame.operand_stack.push(StackValue::Float(*f)).unwrap();

                Update::None
            },

            Self::Fdiv => {
                let op2 = if let StackValue::Float(f) =
                    frame.operand_stack.pop().unwrap()
                {
                    f
                } else {
                    panic!("float on top of stack");
                };
                let op1 = if let StackValue::Float(f) =
                    frame.operand_stack.pop().unwrap()
                {
                    f
                } else {
                    panic!("float on top of stack");
                };

                frame
                    .operand_stack
                    .push(StackValue::Float(op1 / op2))
                    .unwrap();

                Update::None
            },

            Self::Fmul => {
                let op2 = if let StackValue::Float(f) =
                    frame.operand_stack.pop().unwrap()
                {
                    f
                } else {
                    panic!("float on top of stack");
                };
                let op1 = if let StackValue::Float(f) =
                    frame.operand_stack.pop().unwrap()
                {
                    f
                } else {
                    panic!("float on top of stack");
                };

                frame
                    .operand_stack
                    .push(StackValue::Float(op1 * op2))
                    .unwrap();

                Update::None
            },

            Self::Fneg => {
                let val = if let StackValue::Float(f) =
                    frame.operand_stack.pop().unwrap()
                {
                    f
                } else {
                    panic!("float on top of stack");
                };

                frame.operand_stack.push(StackValue::Float(-val)).unwrap();

                Update::None
            },

            Self::Frem => {
                let op2 = if let StackValue::Float(f) =
                    frame.operand_stack.pop().unwrap()
                {
                    f
                } else {
                    panic!("float on top of stack");
                };
                let op1 = if let StackValue::Float(f) =
                    frame.operand_stack.pop().unwrap()
                {
                    f
                } else {
                    panic!("float on top of stack");
                };

                frame
                    .operand_stack
                    .push(StackValue::Float(op1 % op2))
                    .unwrap();

                Update::None
            },

            Self::Freturn => {
                let retval = if let StackValue::Float(d) =
                    frame.operand_stack.pop().unwrap()
                {
                    d
                } else {
                    panic!("expected float on top");
                };

                Update::Return(ReturnValue::Float(retval))
            },

            Self::Fsub => {
                let op2 = if let StackValue::Float(f) =
                    frame.operand_stack.pop().unwrap()
                {
                    f
                } else {
                    panic!("float on top of stack");
                };
                let op1 = if let StackValue::Float(f) =
                    frame.operand_stack.pop().unwrap()
                {
                    f
                } else {
                    panic!("float on top of stack");
                };

                frame
                    .operand_stack
                    .push(StackValue::Float(op1 - op2))
                    .unwrap();

                Update::None
            },

            Self::GetField { field_name, class } => {
                let objectref: Rc<dyn ClassInstance> =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();

                let field = objectref.get_field(class, field_name);
                frame
                    .operand_stack
                    .push(field.value.clone().into_inner().into())
                    .unwrap();

                Update::None
            },

            Self::GetStatic(field) => {
                frame
                    .operand_stack
                    .push(field.value.clone().into_inner().into())
                    .unwrap();
                Update::None
            },

            Self::Goto(position, direction) => {
                Update::GoTo(*position, *direction)
            },

            Self::Iaload => {
                let index: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let array: Rc<dyn ClassInstance> =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let array: &IntArrayInstance =
                    array.as_ref().try_into().unwrap();

                let obj = array.get(index.try_into().unwrap()).unwrap();
                frame.operand_stack.push(StackValue::Int(obj)).unwrap();

                Update::None
            },

            Self::Iadd => {
                let op2: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let op1: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();

                let result: i32 = op1.wrapping_add(op2);

                // result is always int,
                // for byte/short/etc it will be explicitly casted
                // by the following bytecode op
                // (generated by compiler)
                frame.operand_stack.push(StackValue::Int(result)).unwrap();

                Update::None
            },

            Self::Iastore => {
                let value: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let index: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let array: Rc<dyn ClassInstance> =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();

                let array: &IntArrayInstance =
                    array.as_ref().try_into().unwrap();
                array.set(index.try_into().unwrap(), value).unwrap();
                Update::None
            },

            Self::Iand => {
                let op2: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let op1: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();

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
                let op2: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let op1: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();

                // TODO(FW): this panics for division by 0
                let result: i32 = op1.wrapping_div(op2);

                // result is always int,
                // for byte/short/etc it will be explicitly casted
                // by the following bytecode op
                // (generated by compiler)
                frame.operand_stack.push(StackValue::Int(result)).unwrap();

                Update::None
            },
            Self::IfacmpNe(size, direction) => {
                let op2: Option<Rc<dyn ClassInstance>> =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let op1: Option<Rc<dyn ClassInstance>> =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                match (op1, op2) {
                    (Some(_), None) | (None, Some(_)) => {
                        Update::GoTo(*size, *direction)
                    },
                    (None, None) => Update::None,
                    (Some(op1), Some(op2)) => {
                        if !trait_pointer_eq(op1.as_ref(), op2.as_ref()) {
                            Update::GoTo(*size, *direction)
                        } else {
                            Update::None
                        }
                    },
                }
            },

            Self::IfacmpEq(size, direction) => {
                let op2: Option<Rc<dyn ClassInstance>> =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let op1: Option<Rc<dyn ClassInstance>> =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                match (op1, op2) {
                    (Some(_), None) | (None, Some(_)) => Update::None,
                    (None, None) => Update::GoTo(*size, *direction),
                    (Some(op1), Some(op2)) => {
                        if trait_pointer_eq(op1.as_ref(), op2.as_ref()) {
                            Update::GoTo(*size, *direction)
                        } else {
                            Update::None
                        }
                    },
                }
            },

            Self::IficmpEq(size, direction) => {
                let op2: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let op1: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                if op1 == op2 {
                    Update::GoTo(*size, *direction)
                } else {
                    Update::None
                }
            },

            Self::IficmpNe(size, direction) => {
                let op2: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let op1: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                if op1 != op2 {
                    Update::GoTo(*size, *direction)
                } else {
                    Update::None
                }
            },

            Self::IficmpLt(size, direction) => {
                let op2: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let op1: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                if op1 < op2 {
                    Update::GoTo(*size, *direction)
                } else {
                    Update::None
                }
            },

            Self::IficmpGt(size, direction) => {
                let op2: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let op1: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                if op1 > op2 {
                    Update::GoTo(*size, *direction)
                } else {
                    Update::None
                }
            },

            Self::IficmpLe(size, direction) => {
                let op2: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let op1: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                if op1 <= op2 {
                    Update::GoTo(*size, *direction)
                } else {
                    Update::None
                }
            },

            Self::IficmpGe(size, direction) => {
                let op2: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let op1: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                if op1 >= op2 {
                    Update::GoTo(*size, *direction)
                } else {
                    Update::None
                }
            },

            Self::IfEq(size, direction) => {
                let op1: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                if op1 == 0 {
                    Update::GoTo(*size, *direction)
                } else {
                    Update::None
                }
            },

            Self::IfNe(size, direction) => {
                let op1: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                if op1 != 0 {
                    Update::GoTo(*size, *direction)
                } else {
                    Update::None
                }
            },

            Self::IfLt(size, direction) => {
                let op1: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                if op1 < 0 {
                    Update::GoTo(*size, *direction)
                } else {
                    Update::None
                }
            },

            Self::IfGt(size, direction) => {
                let op1: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                if op1 > 0 {
                    Update::GoTo(*size, *direction)
                } else {
                    Update::None
                }
            },

            Self::IfLe(size, direction) => {
                let op1: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                if op1 <= 0 {
                    Update::GoTo(*size, *direction)
                } else {
                    Update::None
                }
            },

            Self::IfGe(size, direction) => {
                let op1: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                if op1 >= 0 {
                    Update::GoTo(*size, *direction)
                } else {
                    Update::None
                }
            },

            Self::IfNonNull(size, direction) => {
                let op1: Option<Rc<dyn ClassInstance>> =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                if op1.is_some() {
                    Update::GoTo(*size, *direction)
                } else {
                    Update::None
                }
            },

            Self::IfNull(size, direction) => {
                let op1: Option<Rc<dyn ClassInstance>> =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                if op1.is_none() {
                    Update::GoTo(*size, *direction)
                } else {
                    Update::None
                }
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
                let op2: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let op1: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();

                let result: i32 = op1.wrapping_mul(op2);

                frame.operand_stack.push(StackValue::Int(result)).unwrap();

                Update::None
            },

            Self::Ineg => {
                let op1: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();

                let result: i32 = op1.neg();

                frame.operand_stack.push(StackValue::Int(result)).unwrap();

                Update::None
            },

            Self::InvokeSpecial(method) => {
                let class =
                    heap.find_class(&method.class_name).unwrap_or_else(|| {
                        panic!("class for method call {:?} exists", method)
                    });
                let method = class
                    .get_method(
                        &method.descriptor.name,
                        (
                            &method.descriptor.descriptor.0,
                            method.descriptor.descriptor.1.as_ref(),
                        ),
                    )
                    .unwrap();

                Update::MethodCall {
                    method,
                    is_static: false,
                    defining_class: class.clone(),
                }
            },

            Self::InvokeStatic(method) => {
                let class = heap.find_class(&method.class_name).unwrap();
                let method = class
                    .get_method(
                        &method.descriptor.name,
                        (
                            &method.descriptor.descriptor.0,
                            method.descriptor.descriptor.1.as_ref(),
                        ),
                    )
                    .unwrap();

                Update::MethodCall {
                    method,
                    is_static: true,
                    defining_class: class.clone(),
                }
            },

            Self::InvokeVirtual(method) => {
                let class = heap.find_class(&method.class_name).unwrap();
                let method = class
                    .get_method(
                        &method.descriptor.name,
                        (
                            &method.descriptor.descriptor.0,
                            method.descriptor.descriptor.1.as_ref(),
                        ),
                    )
                    .unwrap();

                Update::MethodCall {
                    method,
                    is_static: false,
                    defining_class: class.clone(),
                }
            },

            Self::Ior => {
                let op2: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let op1: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();

                let result: i32 = op1 | op2;

                frame.operand_stack.push(StackValue::Int(result)).unwrap();

                Update::None
            },

            Self::Irem => {
                let op2: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let op1: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();

                let result: i32 = op1.wrapping_rem(op2);

                frame.operand_stack.push(StackValue::Int(result)).unwrap();

                Update::None
            },

            Self::Ireturn => {
                let retval: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();

                Update::Return(ReturnValue::Int(retval))
            },

            Self::Ishl => {
                let op2: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let op1: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();

                let result: i32 = op1 << (op2 & 0x1f);

                frame.operand_stack.push(StackValue::Int(result)).unwrap();

                Update::None
            },

            Self::Ishr => {
                let op2: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let op1: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();

                let result: i32 = op1 >> (op2 & 0x1f);

                frame.operand_stack.push(StackValue::Int(result)).unwrap();

                Update::None
            },

            Self::Isub => {
                let op2: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let op1: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();

                let result: i32 = op1.wrapping_sub(op2);

                frame.operand_stack.push(StackValue::Int(result)).unwrap();

                Update::None
            },

            Self::Iushr => {
                let op2: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let op1: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();

                // perform shift with zero extension
                // by casting to an unsigned value before the shift
                let result: i32 = ((op1 as u32) >> (op2 & 0x1f)) as i32;

                frame.operand_stack.push(StackValue::Int(result)).unwrap();

                Update::None
            },

            Self::Ixor => {
                let op2: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let op1: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();

                let result: i32 = op1 ^ op2;

                frame.operand_stack.push(StackValue::Int(result)).unwrap();

                Update::None
            },

            Self::L2d => {
                let val = if let StackValue::Long(l) =
                    frame.operand_stack.pop().unwrap()
                {
                    l
                } else {
                    panic!("long on stack");
                };

                frame
                    .operand_stack
                    .push(StackValue::Double(val as f64))
                    .unwrap();

                Update::None
            },

            Self::L2f => {
                let val = if let StackValue::Long(l) =
                    frame.operand_stack.pop().unwrap()
                {
                    l
                } else {
                    panic!("long on stack");
                };

                frame
                    .operand_stack
                    .push(StackValue::Float(val as f32))
                    .unwrap();

                Update::None
            },

            Self::L2i => {
                let val = if let StackValue::Long(l) =
                    frame.operand_stack.pop().unwrap()
                {
                    l
                } else {
                    panic!("long on stack");
                };

                frame
                    .operand_stack
                    .push(StackValue::Int(val as i32))
                    .unwrap();

                Update::None
            },

            Self::I2b => {
                let int = frame.operand_stack.pop().expect("stack has value");
                let int = match int {
                    StackValue::Int(i) => i,
                    _ => panic!("expect int stack value, got '{:?}'", int),
                };
                let byte: i8 = int as i8;
                frame
                    .operand_stack
                    .push(StackValue::Int(byte.into()))
                    .unwrap();
                Update::None
            },
            Self::I2c => {
                let int = frame.operand_stack.pop().expect("stack has value");
                let int = match int {
                    StackValue::Int(i) => i,
                    _ => panic!("expect int stack value, got '{:?}'", int),
                };
                let c: u16 = int as u16;
                frame.operand_stack.push(StackValue::Int(c as i32)).unwrap();
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
                frame
                    .operand_stack
                    .push(StackValue::Int(short.into()))
                    .unwrap();
                Update::None
            },

            Self::MultiAnewArray(identifier) => {
                let mut array_lens: Vec<usize> = vec![0]; // 1-based

                let (dimensions, _) = identifier.get_array_class_name();
                // stack contains as many counts as there are dimensions
                // inner most dimension is on top of stack, outer most on bottom
                for _ in 0..dimensions {
                    let dim: i32 =
                        frame.operand_stack.pop().unwrap().try_into().unwrap();
                    array_lens.push(dim.try_into().unwrap());
                }

                let outer_array = init_array(
                    heap,
                    identifier,
                    array_lens[Into::<usize>::into(dimensions)],
                );
                init_array_rec(
                    heap,
                    identifier,
                    &array_lens,
                    outer_array.clone(),
                );

                frame
                    .operand_stack
                    .push(StackValue::Reference(Some(outer_array)))
                    .unwrap();

                Update::None
            },

            Self::New(class_identifier) => {
                let class = heap.find_class(class_identifier).unwrap();

                let instance = class.new_instance(class.clone());

                frame
                    .operand_stack
                    .push(StackValue::Reference(Some(instance)))
                    .unwrap();

                Update::None
            },

            Self::Ladd => {
                let op2 = if let StackValue::Long(l) =
                    frame.operand_stack.pop().unwrap()
                {
                    l
                } else {
                    panic!("expected long value");
                };
                let op1 = if let StackValue::Long(l) =
                    frame.operand_stack.pop().unwrap()
                {
                    l
                } else {
                    panic!("expected long value");
                };

                frame
                    .operand_stack
                    .push(StackValue::Long(op1.wrapping_add(op2)))
                    .unwrap();

                Update::None
            },

            Self::Laload => {
                let index: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let array: Rc<dyn ClassInstance> =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let array: &LongArrayInstance =
                    array.as_ref().try_into().unwrap();

                let obj = array.get(index.try_into().unwrap()).unwrap();
                frame.operand_stack.push(StackValue::Long(obj)).unwrap();

                Update::None
            },

            Self::Land => {
                let op2 = if let StackValue::Long(l) =
                    frame.operand_stack.pop().unwrap()
                {
                    l
                } else {
                    panic!("expected long value");
                };
                let op1 = if let StackValue::Long(l) =
                    frame.operand_stack.pop().unwrap()
                {
                    l
                } else {
                    panic!("expected long value");
                };

                frame
                    .operand_stack
                    .push(StackValue::Long(op1 & op2))
                    .unwrap();

                Update::None
            },

            Self::Lastore => {
                let value: StackValue = frame.operand_stack.pop().unwrap();
                let index: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
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

            Self::Lcmp => {
                let op2 = if let StackValue::Long(l) =
                    frame.operand_stack.pop().unwrap()
                {
                    l
                } else {
                    panic!("expected long value");
                };
                let op1 = if let StackValue::Long(l) =
                    frame.operand_stack.pop().unwrap()
                {
                    l
                } else {
                    panic!("expected long value");
                };

                match op1.cmp(&op2) {
                    Ordering::Greater => {
                        frame.operand_stack.push(StackValue::Int(1)).unwrap()
                    },
                    Ordering::Equal => {
                        frame.operand_stack.push(StackValue::Int(0)).unwrap()
                    },
                    Ordering::Less => {
                        frame.operand_stack.push(StackValue::Int(-1)).unwrap()
                    },
                };

                Update::None
            },

            Self::Lconst(l) => {
                frame.operand_stack.push(StackValue::Long(*l)).unwrap();

                Update::None
            },

            Self::Ldc(Ldc::Int(i)) => {
                frame.operand_stack.push(StackValue::Int(*i)).unwrap();
                Update::None
            },
            Self::Ldc(Ldc::Long(l)) => {
                frame.operand_stack.push(StackValue::Long(*l)).unwrap();
                Update::None
            },
            Self::Ldc(Ldc::Float(f)) => {
                frame.operand_stack.push(StackValue::Float(*f)).unwrap();
                Update::None
            },
            Self::Ldc(Ldc::Double(d)) => {
                frame.operand_stack.push(StackValue::Double(*d)).unwrap();
                Update::None
            },
            Self::Ldc(Ldc::String(s)) => {
                frame
                    .operand_stack
                    .push(StackValue::Reference(Some(s.clone())))
                    .unwrap();
                Update::None
            },

            Self::Ldiv => {
                let op2 = if let StackValue::Long(l) =
                    frame.operand_stack.pop().unwrap()
                {
                    l
                } else {
                    panic!("expected long value");
                };
                let op1 = if let StackValue::Long(l) =
                    frame.operand_stack.pop().unwrap()
                {
                    l
                } else {
                    panic!("expected long value");
                };

                frame
                    .operand_stack
                    .push(StackValue::Long(op1.wrapping_div(op2)))
                    .unwrap();

                Update::None
            },

            Self::Lmul => {
                let op2 = if let StackValue::Long(l) =
                    frame.operand_stack.pop().unwrap()
                {
                    l
                } else {
                    panic!("expected long value");
                };
                let op1 = if let StackValue::Long(l) =
                    frame.operand_stack.pop().unwrap()
                {
                    l
                } else {
                    panic!("expected long value");
                };

                frame
                    .operand_stack
                    .push(StackValue::Long(op1.wrapping_mul(op2)))
                    .unwrap();

                Update::None
            },

            Self::Lneg => {
                let val = if let StackValue::Long(l) =
                    frame.operand_stack.pop().unwrap()
                {
                    l
                } else {
                    panic!("expected long value");
                };

                frame
                    .operand_stack
                    .push(StackValue::Long(val.wrapping_neg()))
                    .unwrap();

                Update::None
            },

            Self::Lor => {
                let op2 = if let StackValue::Long(l) =
                    frame.operand_stack.pop().unwrap()
                {
                    l
                } else {
                    panic!("expected long value");
                };
                let op1 = if let StackValue::Long(l) =
                    frame.operand_stack.pop().unwrap()
                {
                    l
                } else {
                    panic!("expected long value");
                };

                frame
                    .operand_stack
                    .push(StackValue::Long(op1 | op2))
                    .unwrap();

                Update::None
            },

            Self::Lrem => {
                let op2 = if let StackValue::Long(l) =
                    frame.operand_stack.pop().unwrap()
                {
                    l
                } else {
                    panic!("expected long value");
                };
                let op1 = if let StackValue::Long(l) =
                    frame.operand_stack.pop().unwrap()
                {
                    l
                } else {
                    panic!("expected long value");
                };

                frame
                    .operand_stack
                    .push(StackValue::Long(op1.wrapping_rem(op2)))
                    .unwrap();

                Update::None
            },

            Self::Lreturn => {
                let retval = if let StackValue::Long(l) =
                    frame.operand_stack.pop().unwrap()
                {
                    l
                } else {
                    panic!("expected long on top");
                };

                Update::Return(ReturnValue::Long(retval))
            },

            Self::Lshl => {
                let op2: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let op1 = if let StackValue::Long(l) =
                    frame.operand_stack.pop().unwrap()
                {
                    l
                } else {
                    panic!("expected long value");
                };

                frame
                    .operand_stack
                    .push(StackValue::Long(op1 << (op2 & 0x3f)))
                    .unwrap();

                Update::None
            },

            Self::Lshr => {
                let op2: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let op1 = if let StackValue::Long(l) =
                    frame.operand_stack.pop().unwrap()
                {
                    l
                } else {
                    panic!("expected long value");
                };

                frame
                    .operand_stack
                    .push(StackValue::Long(op1 >> (op2 & 0x3f)))
                    .unwrap();

                Update::None
            },

            Self::Lsub => {
                let op2 = if let StackValue::Long(l) =
                    frame.operand_stack.pop().unwrap()
                {
                    l
                } else {
                    panic!("expected long value");
                };
                let op1 = if let StackValue::Long(l) =
                    frame.operand_stack.pop().unwrap()
                {
                    l
                } else {
                    panic!("expected long value");
                };

                frame
                    .operand_stack
                    .push(StackValue::Long(op1.wrapping_sub(op2)))
                    .unwrap();

                Update::None
            },

            Self::Lushr => {
                let op2: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let op1 = if let StackValue::Long(l) =
                    frame.operand_stack.pop().unwrap()
                {
                    l
                } else {
                    panic!("expected long value");
                };

                // perform shift with zero extension
                // by casting to an unsigned value before the shift
                let result: i64 = ((op1 as u64) >> (op2 & 0x3f)) as i64;

                frame.operand_stack.push(StackValue::Long(result)).unwrap();

                Update::None
            },

            Self::Lxor => {
                let op2 = if let StackValue::Long(l) =
                    frame.operand_stack.pop().unwrap()
                {
                    l
                } else {
                    panic!("expected long value");
                };
                let op1 = if let StackValue::Long(l) =
                    frame.operand_stack.pop().unwrap()
                {
                    l
                } else {
                    panic!("expected long value");
                };

                frame
                    .operand_stack
                    .push(StackValue::Long(op1 ^ op2))
                    .unwrap();

                Update::None
            },

            Self::NewArray(array_type) => {
                let size: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();

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

            Self::Nop => Update::None,

            Self::PutField { field_name, class } => {
                let value: StackValue = frame.operand_stack.pop().unwrap();
                let objectref: Rc<dyn ClassInstance> =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();

                let field = objectref.get_field(class, field_name);

                field.value.replace_with(|old| match old {
                    FieldValue::Byte(_) => match value {
                        StackValue::Int(b) => FieldValue::Byte(b as i8),
                        _ => panic!(),
                    },
                    FieldValue::Short(_) => match value {
                        StackValue::Int(s) => FieldValue::Short(s as i16),
                        _ => panic!(),
                    },
                    FieldValue::Int(_) => match value {
                        StackValue::Int(i) => FieldValue::Int(i),
                        _ => panic!(),
                    },
                    FieldValue::Long(_) => match value {
                        StackValue::Long(l) => FieldValue::Long(l),
                        _ => panic!(),
                    },
                    FieldValue::Char(_) => match value {
                        StackValue::Int(c) => FieldValue::Char(c as u16),
                        _ => panic!(),
                    },
                    FieldValue::Float(_) => match value {
                        StackValue::Float(f) => FieldValue::Float(f),
                        _ => panic!(),
                    },
                    FieldValue::Double(_) => match value {
                        StackValue::Double(d) => FieldValue::Double(d),
                        _ => panic!(),
                    },
                    FieldValue::Boolean(_) => match value {
                        StackValue::Int(b) => FieldValue::Boolean(b as u8),
                        _ => panic!(),
                    },
                    FieldValue::Reference(_) => match value {
                        StackValue::Reference(r) => FieldValue::Reference(r),
                        _ => panic!(),
                    },
                });

                Update::None
            },

            Self::Return => Update::Return(ReturnValue::Void),

            Self::Saload => {
                let index: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let array: Rc<dyn ClassInstance> =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let array: &ShortArrayInstance =
                    array.as_ref().try_into().unwrap();

                let obj = array.get(index.try_into().unwrap()).unwrap();
                frame
                    .operand_stack
                    .push(StackValue::Int(obj.into()))
                    .unwrap();

                Update::None
            },

            Self::Sastore => {
                let value: StackValue = frame.operand_stack.pop().unwrap();
                let index: i32 =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();
                let array: Rc<dyn ClassInstance> =
                    frame.operand_stack.pop().unwrap().try_into().unwrap();

                let value: i32 = value.try_into().unwrap();
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
    array_identifier: &ClassIdentifier,
    array_len: usize,
) -> Rc<dyn ClassInstance> {
    let array_cls = heap.find_array_class(array_identifier).unwrap();
    let (dim_count, component_type) = array_identifier.get_array_class_name();
    if dim_count > 1 || matches!(component_type, ArrayName::Class(_)) {
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
            ArrayName::Boolean => {
                let array_cls_for_ref = array_cls.clone();
                let array_ref: &BoolArray =
                    array_cls_for_ref.as_ref().try_into().unwrap();
                let array_inst = array_ref
                    .new_instance_from_ref(array_len, array_cls)
                    .unwrap();

                Rc::new(array_inst)
            },
            ArrayName::Byte => {
                let array_cls_for_ref = array_cls.clone();
                let array_ref: &ByteArray =
                    array_cls_for_ref.as_ref().try_into().unwrap();
                let array_inst = array_ref
                    .new_instance_from_ref(array_len, array_cls)
                    .unwrap();

                Rc::new(array_inst)
            },
            ArrayName::Char => {
                let array_cls_for_ref = array_cls.clone();
                let array_ref: &CharArray =
                    array_cls_for_ref.as_ref().try_into().unwrap();
                let array_inst = array_ref
                    .new_instance_from_ref(array_len, array_cls)
                    .unwrap();

                Rc::new(array_inst)
            },
            ArrayName::Double => {
                let array_cls_for_ref = array_cls.clone();
                let array_ref: &DoubleArray =
                    array_cls_for_ref.as_ref().try_into().unwrap();
                let array_inst = array_ref
                    .new_instance_from_ref(array_len, array_cls)
                    .unwrap();

                Rc::new(array_inst)
            },
            ArrayName::Float => {
                let array_cls_for_ref = array_cls.clone();
                let array_ref: &FloatArray =
                    array_cls_for_ref.as_ref().try_into().unwrap();
                let array_inst = array_ref
                    .new_instance_from_ref(array_len, array_cls)
                    .unwrap();

                Rc::new(array_inst)
            },
            ArrayName::Long => {
                let array_cls_for_ref = array_cls.clone();
                let array_ref: &LongArray =
                    array_cls_for_ref.as_ref().try_into().unwrap();
                let array_inst = array_ref
                    .new_instance_from_ref(array_len, array_cls)
                    .unwrap();

                Rc::new(array_inst)
            },
            ArrayName::Int => {
                let array_cls_for_ref = array_cls.clone();
                let array_ref: &IntArray =
                    array_cls_for_ref.as_ref().try_into().unwrap();
                let array_inst = array_ref
                    .new_instance_from_ref(array_len, array_cls)
                    .unwrap();

                Rc::new(array_inst)
            },
            ArrayName::Short => {
                let array_cls_for_ref = array_cls.clone();
                let array_ref: &ShortArray =
                    array_cls_for_ref.as_ref().try_into().unwrap();
                let array_inst = array_ref
                    .new_instance_from_ref(array_len, array_cls)
                    .unwrap();

                Rc::new(array_inst)
            },
            ArrayName::Class(_) => {
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
    array_identifier: &ClassIdentifier,
    array_lens: &[usize],
    outer_array: Rc<dyn ClassInstance>,
) {
    let (package, (dim, component_type)) =
        array_identifier.clone().into_array_identifier();

    if dim == 1 {
        // rec exit
        return;
    }

    let new_identifier = ClassIdentifier {
        package,
        class_name: ClassName::Array {
            dimensions: dim - 1,
            name: component_type.clone(),
        },
    };

    let outer_array: &ObjectArrayInstance =
        outer_array.as_ref().try_into().unwrap();
    for i in 0..array_lens[Into::<usize>::into(dim)] {
        let inner_array = init_array(
            heap,
            &new_identifier,
            array_lens[Into::<usize>::into(dim - 1)],
        );
        // rec enter
        init_array_rec(heap, &new_identifier, array_lens, inner_array.clone());
        outer_array.set(i, Some(inner_array)).unwrap();
    }
}
