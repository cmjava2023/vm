use std::{any::Any, cell::RefCell, rc::Rc};

use crate::{
    class::{
        class_identifier, ArrayName, Class, ClassIdentifier, ClassInstance,
        ClassName, Field, FieldDescriptor,
    },
    executor::RuntimeError,
};

pub type ObjectArray = Array<ObjectArrayKind>;
pub type ObjectArrayInstance = ArrayInstance<ObjectArrayKind>;

pub struct ObjectArrayKind {
    array_class_identifier: ClassIdentifier,
}

impl ObjectArrayKind {
    pub fn new(class: Rc<dyn Class>) -> ObjectArrayKind {
        let package = class.class_identifier().package.clone();
        match &class.class_identifier().class_name {
            ClassName::Array { dimensions, name } => ObjectArrayKind {
                array_class_identifier: ClassIdentifier {
                    package,
                    class_name: ClassName::Array {
                        dimensions: 1 + dimensions,
                        name: name.clone(),
                    },
                },
            },
            ClassName::Plain(class_name) => ObjectArrayKind {
                array_class_identifier: ClassIdentifier {
                    package,
                    class_name: ClassName::Array {
                        dimensions: 1,
                        name: ArrayName::Class(class_name.clone()),
                    },
                },
            },
        }
    }
}

impl ArrayKind for ObjectArrayKind {
    type Value = Option<Rc<dyn ClassInstance>>;

    fn class_identifier(&self) -> &ClassIdentifier {
        &self.array_class_identifier
    }

    fn default_val(&self) -> Self::Value {
        None
    }
}

pub type ByteArray = Array<ByteArrayKind>;
pub type ByteArrayInstance = ArrayInstance<ByteArrayKind>;
pub struct ByteArrayKind {
    class_identifier: ClassIdentifier,
}

impl ByteArrayKind {
    pub fn new() -> Self {
        ByteArrayKind {
            class_identifier: class_identifier!(byte, 1),
        }
    }
}

impl Default for ByteArrayKind {
    fn default() -> Self {
        ByteArrayKind::new()
    }
}

impl ArrayKind for ByteArrayKind {
    type Value = i8;

    fn class_identifier(&self) -> &ClassIdentifier {
        &self.class_identifier
    }

    fn default_val(&self) -> Self::Value {
        0
    }
}

pub type BoolArray = Array<BoolArrayKind>;
pub type BoolArrayInstance = ArrayInstance<BoolArrayKind>;
pub struct BoolArrayKind {
    class_identifier: ClassIdentifier,
}

impl BoolArrayKind {
    pub fn new() -> Self {
        BoolArrayKind {
            class_identifier: class_identifier!(bool, 1),
        }
    }
}

impl Default for BoolArrayKind {
    fn default() -> Self {
        BoolArrayKind::new()
    }
}

impl ArrayKind for BoolArrayKind {
    type Value = bool;

    fn class_identifier(&self) -> &ClassIdentifier {
        &self.class_identifier
    }

    fn default_val(&self) -> Self::Value {
        false
    }
}

pub type CharArray = Array<CharArrayKind>;
pub type CharArrayInstance = ArrayInstance<CharArrayKind>;
pub struct CharArrayKind {
    class_identifier: ClassIdentifier,
}

impl CharArrayKind {
    pub fn new() -> Self {
        CharArrayKind {
            class_identifier: class_identifier!(char, 1),
        }
    }
}

impl Default for CharArrayKind {
    fn default() -> Self {
        CharArrayKind::new()
    }
}

impl ArrayKind for CharArrayKind {
    type Value = u16;

    fn class_identifier(&self) -> &ClassIdentifier {
        &self.class_identifier
    }

    fn default_val(&self) -> Self::Value {
        0
    }
}

pub type DoubleArray = Array<DoubleArrayKind>;
pub type DoubleArrayInstance = ArrayInstance<DoubleArrayKind>;
pub struct DoubleArrayKind {
    class_identifier: ClassIdentifier,
}

impl DoubleArrayKind {
    pub fn new() -> Self {
        DoubleArrayKind {
            class_identifier: class_identifier!(double, 1),
        }
    }
}

impl Default for DoubleArrayKind {
    fn default() -> Self {
        DoubleArrayKind::new()
    }
}

impl ArrayKind for DoubleArrayKind {
    type Value = f64;

    fn class_identifier(&self) -> &ClassIdentifier {
        &self.class_identifier
    }

    fn default_val(&self) -> Self::Value {
        0.0
    }
}

pub type FloatArray = Array<FloatArrayKind>;
pub type FloatArrayInstance = ArrayInstance<FloatArrayKind>;
pub struct FloatArrayKind {
    class_identifier: ClassIdentifier,
}

impl FloatArrayKind {
    pub fn new() -> Self {
        FloatArrayKind {
            class_identifier: class_identifier!(float, 1),
        }
    }
}

impl Default for FloatArrayKind {
    fn default() -> Self {
        FloatArrayKind::new()
    }
}

impl ArrayKind for FloatArrayKind {
    type Value = f32;

    fn class_identifier(&self) -> &ClassIdentifier {
        &self.class_identifier
    }

    fn default_val(&self) -> Self::Value {
        0.0
    }
}

pub type IntArray = Array<IntArrayKind>;
pub type IntArrayInstance = ArrayInstance<IntArrayKind>;
pub struct IntArrayKind {
    class_identifier: ClassIdentifier,
}

impl IntArrayKind {
    pub fn new() -> Self {
        IntArrayKind {
            class_identifier: class_identifier!(int, 1),
        }
    }
}

impl Default for IntArrayKind {
    fn default() -> Self {
        Self::new()
    }
}

impl ArrayKind for IntArrayKind {
    type Value = i32;

    fn class_identifier(&self) -> &ClassIdentifier {
        &self.class_identifier
    }

    fn default_val(&self) -> Self::Value {
        0
    }
}

pub type LongArray = Array<LongArrayKind>;
pub type LongArrayInstance = ArrayInstance<LongArrayKind>;
pub struct LongArrayKind {
    class_identifier: ClassIdentifier,
}

impl LongArrayKind {
    pub fn new() -> Self {
        LongArrayKind {
            class_identifier: class_identifier!(long, 1),
        }
    }
}

impl Default for LongArrayKind {
    fn default() -> Self {
        Self::new()
    }
}

impl ArrayKind for LongArrayKind {
    type Value = i64;

    fn class_identifier(&self) -> &ClassIdentifier {
        &self.class_identifier
    }

    fn default_val(&self) -> Self::Value {
        0
    }
}

pub type ShortArray = Array<ShortArrayKind>;
pub type ShortArrayInstance = ArrayInstance<ShortArrayKind>;
pub struct ShortArrayKind {
    class_identifier: ClassIdentifier,
}

impl ShortArrayKind {
    pub fn new() -> Self {
        Self {
            class_identifier: class_identifier!(short, 1),
        }
    }
}

impl Default for ShortArrayKind {
    fn default() -> Self {
        Self::new()
    }
}

impl ArrayKind for ShortArrayKind {
    type Value = i16;

    fn class_identifier(&self) -> &ClassIdentifier {
        &self.class_identifier
    }

    fn default_val(&self) -> Self::Value {
        0
    }
}

pub trait ArrayKind {
    type Value: Clone;

    fn class_identifier(&self) -> &ClassIdentifier;

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

    fn instance_fields(&self) -> &[FieldDescriptor] {
        &[]
    }

    fn class_identifier(&self) -> &ClassIdentifier {
        self.kind.class_identifier()
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

    fn new_instance(&self, cls: Rc<dyn Class>) -> Rc<dyn ClassInstance> {
        // make sure that self and cls really are equal
        let _cls_ref: &Self = cls.as_ref().try_into().unwrap();

        panic!("arrays cannot be created with new");
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
