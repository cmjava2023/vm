pub mod access_flags;
pub mod builtin_classes;
pub mod bytecode_classes;

use core::fmt;
use std::{any::Any, borrow::Cow, cell::RefCell, ops::Range, rc::Rc};

use crate::executor::{
    frame_stack::StackValue, local_variables::VariableValueOrValue, Frame,
    OpCode, RuntimeError,
};

macro_rules! class_identifier {
    // put primitive array variants at the top,
    // so that they're favored against package + class name
    // variant
    // (otherwise you end up with
    //   { package: ["int"], class_name: Plain("dim") }
    // )
    (byte, $dim:expr) => {
        crate::class::ClassIdentifier {
            package: ::std::borrow::Cow::from(&[][..]),
            class_name: crate::class::ClassName::Array {
                dimensions: $dim,
                name: crate::class::ArrayName::Byte,
            },
        }
    };
    (char, $dim:expr) => {
        crate::class::ClassIdentifier {
            package: ::std::borrow::Cow::from(&[][..]),
            class_name: crate::class::ClassName::Array {
                dimensions: $dim,
                name: crate::class::ArrayName::Char,
            },
        }
    };
    (double, $dim:expr) => {
        crate::class::ClassIdentifier {
            package: ::std::borrow::Cow::from(&[][..]),
            class_name: crate::class::ClassName::Array {
                dimensions: $dim,
                name: crate::class::ArrayName::Double,
            },
        }
    };
    (float, $dim:expr) => {
        crate::class::ClassIdentifier {
            package: ::std::borrow::Cow::from(&[][..]),
            class_name: crate::class::ClassName::Array {
                dimensions: $dim,
                name: crate::class::ArrayName::Float,
            },
        }
    };
    (int, $dim:expr) => {
        crate::class::ClassIdentifier {
            package: ::std::borrow::Cow::from(&[][..]),
            class_name: crate::class::ClassName::Array {
                dimensions: $dim,
                name: crate::class::ArrayName::Int,
            },
        }
    };
    (long, $dim:expr) => {
        crate::class::ClassIdentifier {
            package: ::std::borrow::Cow::from(&[][..]),
            class_name: crate::class::ClassName::Array {
                dimensions: $dim,
                name: crate::class::ArrayName::Long,
            },
        }
    };
    (short, $dim:expr) => {
        crate::class::ClassIdentifier {
            package: ::std::borrow::Cow::from(&[][..]),
            class_name: crate::class::ClassName::Array {
                dimensions: $dim,
                name: crate::class::ArrayName::Short,
            },
        }
    };
    (bool, $dim:expr) => {
        crate::class::ClassIdentifier {
            package: ::std::borrow::Cow::from(&[][..]),
            class_name: crate::class::ClassName::Array {
                dimensions: $dim,
                name: crate::class::ArrayName::Boolean,
            },
        }
    };
    ($c:ident) => {
        crate::class::ClassIdentifier {
            package: ::std::borrow::Cow::from(&[][..]),
            class_name: crate::class::ClassName::Plain(
                ::std::borrow::Cow::from(stringify!($c))
            ),
        }
    };
    ($($p:ident)/+, $c:ident) => {
        crate::class::ClassIdentifier {
            package: ::std::borrow::Cow::from(vec![$(
                ::std::borrow::Cow::from(stringify!($p)),
            )*]),
            class_name: crate::class::ClassName::Plain(
                ::std::borrow::Cow::from(stringify!($c))
            ),
        }
    };
    ($dim:literal, $($p:ident)/+, $c:ident) => {
        crate::class::ClassIdentifier {
            package: ::std::borrow::Cow::from(vec![$(
                ::std::borrow::Cow::from(stringify!($p)),
            )*]),
            class_name: crate::class::ClassName::Array {
                dimensions: $dim,
                name: crate::class::ArrayName::Class(
                    ::std::borrow::Cow::from(stringify!($c))
                ),
            },
        }
    };
    ($dim:literal, $c:ident) => {
        crate::class::ClassIdentifier {
            package: ::std::borrow::Cow::from(&[][..]),
            class_name: crate::class::ClassName::Array {
                dimensions: $dim,
                name: crate::class::ArrayName::Class(
                    ::std::borrow::Cow::from(stringify!($c))
                ),
            },
        }
    };
}

pub(crate) use class_identifier;
use enumflags2::BitFlags;

use self::access_flags::ClassAccessFlag;

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct ClassIdentifier {
    pub package: Cow<'static, [Cow<'static, str>]>,
    pub class_name: ClassName,
}

impl fmt::Display for ClassIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let ClassName::Array { dimensions, .. } = &self.class_name {
            for _ in 0..*dimensions {
                write!(f, "[")?;
            }
        }
        if self.package.len() > 0 {
            write!(f, "{}/", self.package.join("/"))?;
        }

        match &self.class_name {
            ClassName::Array { name, .. } => match name {
                ArrayName::Byte => write!(f, "B"),
                ArrayName::Char => write!(f, "C"),
                ArrayName::Double => write!(f, "D"),
                ArrayName::Float => write!(f, "F"),
                ArrayName::Int => write!(f, "I"),
                ArrayName::Long => write!(f, "J"),
                ArrayName::Class(name) => write!(f, "{}", name),
                ArrayName::Short => write!(f, "S"),
                ArrayName::Boolean => write!(f, "Z"),
            },
            ClassName::Plain(name) => write!(f, "{}", name),
        }
    }
}

impl ClassIdentifier {
    /// Returns the dimension and array class name of this identifier.
    ///
    /// # Panics
    ///
    /// This function will panic if called with an identifier that
    /// is a [ClassName::Plain] identifier.
    pub fn get_array_class_name(&self) -> (usize, &ArrayName) {
        match &self.class_name {
            ClassName::Array { dimensions, name } => (*dimensions, name),
            ClassName::Plain(_) => panic!(
                "{:?} is a ClassName::Plain identifier, \
but expected a ClassName::Array",
                self
            ),
        }
    }

    /// Converts this identifier into
    /// its package and array class name components.
    ///
    /// # Panics
    ///
    /// This function will panic if called with an identifier that
    /// is a [ClassName::Plain] identifier.
    pub fn into_array_identifier(
        self,
    ) -> (Cow<'static, [Cow<'static, str>]>, (usize, ArrayName)) {
        match self.class_name {
            ClassName::Array { dimensions, name } => {
                (self.package, (dimensions, name))
            },
            ClassName::Plain(_) => panic!(
                "{:?} is a ClassName::Plain identifier, \
but expected a ClassName::Array",
                self
            ),
        }
    }

    /// Returns the plain class name of this identifier.
    ///
    /// # Panics
    ///
    /// This function will panic if called with an identifier that
    /// is a [ClassName::Array] identifier.
    pub fn get_plain_name(&self) -> &Cow<'static, str> {
        match &self.class_name {
            ClassName::Plain(name) => name,
            ClassName::Array { .. } => panic!(
                "{:?} is a ClassName::ArrayName identifier, \
but expected a ClassName::Plain",
                self
            ),
        }
    }

    /// Converts this identifier into
    /// its package and plain class name components.
    ///
    /// # Panics
    ///
    /// This function will panic if called with an identifier that
    /// is a [ClassName::Array] identifier.
    pub fn into_plain_identifier(
        self,
    ) -> (Cow<'static, [Cow<'static, str>]>, Cow<'static, str>) {
        match self.class_name {
            ClassName::Plain(name) => (self.package, name),
            ClassName::Array { .. } => panic!(
                "{:?} is a ClassName::ArrayName identifier, \
but expected a ClassName::Plain",
                self
            ),
        }
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum ClassName {
    Array { dimensions: usize, name: ArrayName },
    Plain(Cow<'static, str>),
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum ArrayName {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Class(Cow<'static, str>),
    Short,
    Boolean,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArgumentKind {
    Simple(SimpleArgumentKind),
    Array {
        dimensions: usize,
        kind: SimpleArgumentKind,
    },
}
#[derive(Debug, Clone, PartialEq)]
pub enum SimpleArgumentKind {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Class(String),
    Short,
    Boolean,
}

#[derive(Debug, Clone)]
pub struct Method {
    pub code: MethodCode,
    pub name: String,
    pub parameters: Vec<ArgumentKind>,
    pub return_type: Option<ArgumentKind>,
    pub is_static: bool,
    // TODO flags
    // TODO attributes
}

#[derive(Debug, Clone)]
pub enum MethodCode {
    Bytecode(Code),
    Rust(for<'a> fn(&'a mut Frame) -> RustMethodReturn),
}

pub type ReturnValue = FieldValue;

pub enum RustMethodReturn {
    Void,
    Value(ReturnValue),
}

pub trait Class {
    fn methods(&self) -> &[Rc<Method>];
    fn static_fields(&self) -> &[Rc<Field>];
    fn instance_fields(&self) -> &[FieldDescriptor];
    // TODO flags
    fn class_identifier(&self) -> &ClassIdentifier;
    fn super_class(&self) -> Option<Rc<dyn Class>>;
    // TODO how are interfaces represented?
    fn interfaces(&self) -> &[Rc<dyn std::any::Any>];
    // TODO attributes

    fn as_any(&self) -> &dyn Any;

    /// self and cls must be the same!
    fn new_instance(&self, cls: Rc<dyn Class>) -> Rc<dyn ClassInstance>;

    fn has_acc_super(&self) -> bool {
        // true for any class version java 8 or higher
        // assumption: builtin classes are written against java 8 behavior
        // on invokevirtual,
        // while BytecodeClass overrides this
        true
    }
}

impl dyn Class {
    /// (Recursively) lookup method in self (and superclasses/interfaces).
    ///
    /// Returns the resolved method and the class this method is declared in.
    pub fn get_method(
        self: &Rc<Self>,
        method_name: &str,
        method_descriptor: (&[ArgumentKind], Option<&ArgumentKind>),
        recurse: bool,
    ) -> (Rc<Method>, Rc<dyn Class>) {
        match self.methods().iter().find(|element| {
            element.name == method_name
                && element.parameters == method_descriptor.0
                && element.return_type.as_ref() == method_descriptor.1
        }) {
            Some(m) => (m.clone(), self.clone()),
            None => match self.super_class() {
                Some(c) if recurse => {
                    c.get_method(method_name, method_descriptor, recurse)
                },
                _ => panic!(
                    "could not resolve method {} {:?}",
                    method_name, method_descriptor
                ),
            },
        }
    }

    pub fn get_static_field(&self, field_name: &str) -> Option<Rc<Field>> {
        self.static_fields()
            .iter()
            .find(|element| element.name == field_name)
            .cloned()
    }

    pub fn is_super_class_of(&self, other: &Rc<dyn Class>) -> bool {
        // idea: if self is superclass of other,
        // at some point other's parent must be self
        match other.super_class() {
            // other must be Object, so self cannot be superclass
            None => false,
            Some(other) => {
                if other.class_identifier() == self.class_identifier() {
                    true
                } else {
                    self.is_super_class_of(&other)
                }
            },
        }
    }

    pub fn is_sub_class_of(&self, other: &Rc<dyn Class>) -> bool {
        // idea: if self is subclass of other,
        // at some point self's parent must be other
        match self.super_class() {
            // self must be Object, so cannot be subclass
            None => false,
            Some(self_parent) => {
                if self_parent.class_identifier() == other.class_identifier() {
                    true
                } else {
                    self_parent.is_sub_class_of(other)
                }
            },
        }
    }
}

impl std::fmt::Debug for dyn Class {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Class '{}'", self.class_identifier())
    }
}

#[derive(Debug)]
pub struct BytecodeClass {
    pub methods: Vec<Rc<Method>>,
    pub static_fields: Vec<Rc<Field>>,
    pub instance_fields: Vec<FieldDescriptor>,
    // TODO flags
    pub class_identifier: ClassIdentifier,
    pub super_class: Rc<dyn Class>,
    // TODO how are interfaces represented?
    pub interfaces: Vec<Rc<dyn std::any::Any>>,
    pub access_flags: BitFlags<ClassAccessFlag>,
}

#[derive(Debug)]
pub struct FieldDescriptor {
    pub name: String,
    // TODO flags
    // TODO attributes
    pub kind: FieldKind,
}

#[derive(Debug)]
pub enum FieldKind {
    Byte,
    Short,
    Int,
    Long,
    Char,
    Float,
    Double,
    Boolean,
    Reference,
}

#[derive(Debug)]
pub struct Field {
    pub name: String,
    // TODO flags
    // TODO attributes
    // TODO data type
    pub value: RefCell<FieldValue>,
}

#[derive(Debug, Clone)]
pub enum FieldValue {
    // Primitive Types
    //   Integral Types
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    // UTF-16 encoded Unicode Code point in the Basic Multilingual Plane
    Char(u16),
    //    Floating-Point Types
    Float(f32),
    Double(f64),
    //    Other
    /// Encodes false as 0, true as 1.
    ///
    /// This is according to [the Java VM Spec](
    /// https://docs.oracle.com/javase/specs/jvms/se8/html/
    /// jvms-2.html#jvms-2.3.4
    /// )
    Boolean(u8),
    // Reference Types
    // TODO different reference types (array, interface)
    Reference(Option<Rc<dyn ClassInstance>>),
}

impl FieldValue {
    pub fn byte() -> Self {
        Self::Byte(0)
    }

    pub fn short() -> Self {
        Self::Short(0)
    }

    pub fn int() -> Self {
        Self::Int(0)
    }

    pub fn long() -> Self {
        Self::Long(0)
    }

    pub fn char() -> Self {
        Self::Char(0)
    }

    pub fn float() -> Self {
        Self::Float(0.0)
    }

    pub fn double() -> Self {
        Self::Double(0.0)
    }

    pub fn boolean() -> Self {
        Self::Boolean(0)
    }

    pub fn reference() -> Self {
        Self::Reference(None)
    }
}

#[derive(Debug, Clone)]
pub struct Code {
    pub stack_depth: usize,
    pub local_variable_count: usize,
    pub exception_table: Vec<ExceptionTable>,
    // TODO attributes
    pub byte_code: Vec<OpCode>,
}

#[derive(Debug, Clone)]
pub struct ExceptionTable {
    pub active: Range<usize>,
    pub handler_position: usize,
    pub catch_type: Option<ClassIdentifier>,
}

pub trait ClassInstance {
    fn as_any(&self) -> &dyn Any;
    fn class(&self) -> Rc<dyn Class>;
    fn instance_fields(&self) -> &[Rc<Field>];
    fn parent_instance(&self) -> Option<Rc<dyn ClassInstance>>;
}

impl dyn ClassInstance {
    pub fn get_field(&self, class: &ClassIdentifier, name: &str) -> Rc<Field> {
        let self_field = self.instance_fields().iter().find(|f| f.name == name);
        match self_field {
            Some(field) if self.class().class_identifier() == class => {
                field.clone()
            },
            _ => match self.parent_instance() {
                None => panic!("NoSuchFieldError"),
                Some(i) => i.get_field(class, name),
            },
        }
    }

    /// Execute f with self (or one of its parent instances) casted to PCI.
    ///
    /// This allows builtin classes
    /// to elegantly access their own custom data structure.
    pub fn with_parent_instance<
        PCI: ClassInstance + 'static,
        T,
        F: Fn(&PCI) -> T,
    >(
        &self,
        class_name: &str,
        f: F,
    ) -> T {
        if let Some(instance) = self.as_any().downcast_ref::<PCI>() {
            f(instance)
        } else {
            let mut instance = self.parent_instance().unwrap_or_else(|| {
                panic!(
                    "expected self to be instance of {} or have superclass, \
got {:?}",
                    class_name, self
                )
            });
            loop {
                match instance.as_any().downcast_ref::<PCI>() {
                    Some(i) => break f(i),
                    None => {
                        instance = match instance.parent_instance() {
                            Some(p) => p,
                            None => panic!(
                                "expected (sub)class instance of {}, got: {:?}",
                                class_name, self
                            ),
                        };
                    },
                }
            }
        }
    }
}

impl fmt::Debug for dyn ClassInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "instance of Class '{}'", self.class().class_identifier())
    }
}

impl TryFrom<StackValue> for Rc<dyn ClassInstance> {
    type Error = RuntimeError;

    fn try_from(value: StackValue) -> Result<Self, Self::Error> {
        match value.try_into()? {
            Some(r) => Ok(r),
            _ => Err(RuntimeError::NullPointer),
        }
    }
}

impl TryFrom<StackValue> for Option<Rc<dyn ClassInstance>> {
    type Error = RuntimeError;

    fn try_from(value: StackValue) -> Result<Self, Self::Error> {
        match value {
            StackValue::Reference(r) => Ok(r),
            _ => Err(RuntimeError::InvalidType {
                expected: "reference",
                actual: value.type_name(),
            }),
        }
    }
}

impl TryFrom<VariableValueOrValue> for Option<Rc<dyn ClassInstance>> {
    type Error = RuntimeError;

    fn try_from(value: VariableValueOrValue) -> Result<Self, Self::Error> {
        match value {
            VariableValueOrValue::Reference(r) => Ok(r),
            _ => Err(RuntimeError::InvalidType {
                expected: "reference",
                actual: value.type_name(),
            }),
        }
    }
}
