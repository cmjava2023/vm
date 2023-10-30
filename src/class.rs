// TODO refer to OpCode from execute
#[deprecated(note = "will be replaced with OpCode from execute")]
pub enum OpCode {}

pub enum Method {
    Bytecode(BytecodeMethod),
    // TODO pass execution frame (i.e. stack and local variables)
    // TODO return value?
    Rust(Box<dyn Fn()>),
}

pub trait Class {
    fn methods(&self) -> &[&Method];
    fn static_fields(&self) -> &[&Field];
    fn instance_fields(&self) -> &[&str];
    // TODO flags
    fn package(&self) -> &str;
    fn name(&self) -> &str;
    fn super_class(&self) -> Option<&dyn Class>;
    // TODO how are interfaces represented?
    fn interfaces(&self) -> &[&dyn std::any::Any];
    // TODO attributes
}

pub struct BytecodeClass<'a> {
    methods: Vec<BytecodeMethod>,
    static_fields: Vec<Field<'a>>,
    instance_fields: Vec<String>,
    // TODO flags
    package: String,
    name: String,
    super_class: Option<&'a dyn Class>,
    // TODO how are interfaces represented?
    interfaces: Vec<&'a dyn std::any::Any>,
    // TODO attributes
}

pub struct Field<'a> {
    name: String,
    // TODO flags
    // TODO attributes
    // TODO data type
    value: FieldValue<'a>,
}

pub enum FieldValue<'a> {
    // Primitve Types
    //   Integral Types
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Char(u16),
    //    Floating-Point Types
    Float(f32),
    Double(f64),
    //    Other
    Boolean(u8),
    // Reference Types
    // TODO different reference types (array, interface)
    Reference(Option<&'a ClassInstance<'a>>),
}

impl<'a> FieldValue<'a> {
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

pub struct BytecodeMethod {
    pub name: String,
    // TODO paramter
    // TODO return type
    // TODO flags
    // TODO attributes
    pub code: Code,
}

pub struct Code {
    pub stack_depth: u32,
    pub local_variable_count: u32,
    // TODO exceptions
    // TODO attributes
    pub byte_code: Vec<OpCode>,
}

pub struct ClassInstance<'a> {
    pub class: &'a dyn Class,
    pub instance_fields: Vec<Field<'a>>,
}
