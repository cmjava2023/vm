use std::{any::Any, cell::RefCell, rc::Rc};

use crate::{
    class::{Class, ClassInstance, Field},
    executor::RuntimeError,
};

pub type ObjectArray = Array<ObjectArrayKind>;
pub type ObjectArrayInstance = ArrayInstance<ObjectArrayKind>;

pub struct ObjectArrayKind {
    array_class_name: String,
}

impl ObjectArrayKind {
    pub fn new(class: Rc<dyn Class>) -> ObjectArrayKind {
        ObjectArrayKind {
            array_class_name: format!(
                "[L{}/{};",
                class.package(),
                class.name()
            ),
        }
    }
}

impl ArrayKind for ObjectArrayKind {
    type Value = Option<Rc<dyn ClassInstance>>;

    fn class_name(&self) -> &str {
        &self.array_class_name
    }

    fn default_val(&self) -> Self::Value {
        None
    }
}

pub type ByteArray = Array<ByteArrayKind>;
pub type ByteArrayInstance = ArrayInstance<ByteArrayKind>;
#[derive(Default)]
pub struct ByteArrayKind {}

impl ArrayKind for ByteArrayKind {
    type Value = i8;

    fn class_name(&self) -> &str {
        "[B"
    }

    fn default_val(&self) -> Self::Value {
        0
    }
}

pub type BoolArray = Array<BoolArrayKind>;
pub type BoolArrayInstance = ArrayInstance<BoolArrayKind>;
#[derive(Default)]
pub struct BoolArrayKind {}

impl ArrayKind for BoolArrayKind {
    type Value = bool;

    fn class_name(&self) -> &str {
        "[Z"
    }

    fn default_val(&self) -> Self::Value {
        false
    }
}

pub type CharArray = Array<CharArrayKind>;
pub type CharArrayInstance = ArrayInstance<CharArrayKind>;
#[derive(Default)]
pub struct CharArrayKind {}

impl ArrayKind for CharArrayKind {
    type Value = u16;

    fn class_name(&self) -> &str {
        "[C"
    }

    fn default_val(&self) -> Self::Value {
        0
    }
}

pub type DoubleArray = Array<DoubleArrayKind>;
pub type DoubleArrayInstance = ArrayInstance<DoubleArrayKind>;
#[derive(Default)]
pub struct DoubleArrayKind {}

impl ArrayKind for DoubleArrayKind {
    type Value = f64;

    fn class_name(&self) -> &str {
        "[D"
    }

    fn default_val(&self) -> Self::Value {
        0.0
    }
}

pub type FloatArray = Array<FloatArrayKind>;
pub type FloatArrayInstance = ArrayInstance<FloatArrayKind>;
#[derive(Default)]
pub struct FloatArrayKind {}

impl ArrayKind for FloatArrayKind {
    type Value = f64;

    fn class_name(&self) -> &str {
        "[F"
    }

    fn default_val(&self) -> Self::Value {
        0.0
    }
}

pub type IntArray = Array<IntArrayKind>;
pub type IntArrayInstance = ArrayInstance<IntArrayKind>;
#[derive(Default)]
pub struct IntArrayKind {}

impl ArrayKind for IntArrayKind {
    type Value = i32;

    fn class_name(&self) -> &str {
        "[I"
    }

    fn default_val(&self) -> Self::Value {
        0
    }
}

pub type LongArray = Array<LongArrayKind>;
pub type LongArrayInstance = ArrayInstance<LongArrayKind>;
#[derive(Default)]
pub struct LongArrayKind {}

impl ArrayKind for LongArrayKind {
    type Value = i64;

    fn class_name(&self) -> &str {
        "[J"
    }

    fn default_val(&self) -> Self::Value {
        0
    }
}

pub type ShortArray = Array<ShortArrayKind>;
pub type ShortArrayInstance = ArrayInstance<ShortArrayKind>;
#[derive(Default)]
pub struct ShortArrayKind {}

impl ArrayKind for ShortArrayKind {
    type Value = i16;

    fn class_name(&self) -> &str {
        "[S"
    }

    fn default_val(&self) -> Self::Value {
        0
    }
}

pub trait ArrayKind {
    type Value: Clone;

    fn class_name(&self) -> &str;

    fn default_val(&self) -> Self::Value;
}

pub struct Array<K> {
    kind: K,
}

pub struct ArrayInstance<K: ArrayKind> {
    class: Rc<dyn Class>,
    values: RefCell<Vec<K::Value>>,
}

impl<K: ArrayKind + Default + 'static> Default for Array<K> {
    fn default() -> Self {
        Array::new(K::default())
    }
}

impl<K: ArrayKind + 'static> Array<K> {
    pub fn new(kind: K) -> Array<K> {
        Array { kind }
    }

    pub fn new_instance(self: &Rc<Self>, length: usize) -> ArrayInstance<K> {
        ArrayInstance {
            class: self.clone(),
            values: RefCell::new(vec![self.kind.default_val(); length]),
        }
    }

    /// self and cls must be the same!
    #[allow(clippy::needless_arbitrary_self_type)]
    pub fn new_instance_from_ref(
        self: &Self,
        length: usize,
        cls: Rc<dyn Class>,
    ) -> Result<ArrayInstance<K>, RuntimeError> {
        // make sure that self and cls really are equal
        let _cls_ref: &Self = cls.as_ref().try_into()?;

        Ok(ArrayInstance {
            class: cls,
            values: RefCell::new(vec![self.kind.default_val(); length]),
        })
    }
}

impl<K: ArrayKind + 'static> Class for Array<K> {
    fn methods(&self) -> &[Rc<crate::class::Method>] {
        &[]
    }

    fn static_fields(&self) -> &[Rc<crate::class::Field>] {
        &[]
    }

    fn instance_fields(&self) -> &[String] {
        &[]
    }

    fn package(&self) -> &str {
        ""
    }

    fn name(&self) -> &str {
        self.kind.class_name()
    }

    fn super_class(&self) -> Option<Rc<dyn Class>> {
        None
    }

    fn interfaces(&self) -> &[Rc<dyn std::any::Any>] {
        &[]
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<K: ArrayKind + 'static> ClassInstance for ArrayInstance<K> {
    fn class(&self) -> Rc<dyn Class> {
        self.class.clone()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn instance_fields(&self) -> &[Rc<Field>] {
        &[]
    }
}

impl<K: ArrayKind> ArrayInstance<K> {
    pub fn length(&self) -> usize {
        // java arrays have a fixed length,
        // use the underlying vector's capacity
        // to store that information
        self.values.borrow().capacity()
    }

    pub fn set(
        &self,
        index: usize,
        value: K::Value,
    ) -> Result<(), RuntimeError> {
        if index > self.values.borrow().capacity() {
            return Err(RuntimeError::ArrayIndexOutOfBounds {
                length: self.values.borrow().capacity(),
                index,
            });
        }

        self.values.borrow_mut()[index] = value;

        Ok(())
    }

    pub fn get(&self, index: usize) -> Result<K::Value, RuntimeError> {
        if index > self.values.borrow().capacity() {
            return Err(RuntimeError::ArrayIndexOutOfBounds {
                length: self.values.borrow().capacity(),
                index,
            });
        }

        Ok(self.values.borrow()[index].clone())
    }
}

impl<'a, K: ArrayKind + 'static> TryFrom<&'a dyn ClassInstance>
    for &'a ArrayInstance<K>
{
    type Error = RuntimeError;

    fn try_from(array: &'a dyn ClassInstance) -> Result<Self, Self::Error> {
        match array.as_any().downcast_ref::<ArrayInstance<K>>() {
            Some(array) => Ok(array),
            None => Err(RuntimeError::InvalidType {
                expected: "array",
                actual: "unknown",
            }),
        }
    }
}

impl<'a, K: ArrayKind + 'static> TryFrom<&'a dyn Class> for &'a Array<K> {
    type Error = RuntimeError;

    fn try_from(array: &'a dyn Class) -> Result<Self, Self::Error> {
        match array.as_any().downcast_ref::<Array<K>>() {
            Some(array) => Ok(array),
            None => Err(RuntimeError::InvalidType {
                expected: "array",
                actual: "unknown",
            }),
        }
    }
}
