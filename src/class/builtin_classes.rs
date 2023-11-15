use std::rc::Rc;

use crate::{
    class::{Class, ClassInstance, Field, FieldValue, Method},
    executor::{Frame, Update},
};

pub struct PrintStream {
    methods: Vec<Rc<Method>>,
}

impl PrintStream {
    pub fn new() -> PrintStream {
        PrintStream {
            methods: vec![Rc::new(Method::Rust(println))],
        }
    }
}

impl Default for PrintStream {
    fn default() -> Self {
        PrintStream::new()
    }
}

fn println(frame: &mut Frame) -> Update {
    let string = frame.operand_stack.pop().expect("stack has value on top");
    let string: Rc<dyn ClassInstance> = string.try_into().unwrap();
    let b: &StringInstance = match string.as_any().downcast_ref() {
        Some(s) => s,
        None => panic!("stack reference is not a string"),
    };
    println!("{}", b.string);

    Update::None
}

impl Class for PrintStream {
    fn methods(&self) -> &[Rc<Method>] {
        self.methods.as_slice()
    }

    fn static_fields(&self) -> &[Rc<Field>] {
        &[]
    }

    fn instance_fields(&self) -> &[String] {
        &[]
    }

    fn package(&self) -> &str {
        "java/io"
    }

    fn name(&self) -> &str {
        "PrintStream"
    }

    fn super_class(&self) -> Option<Rc<dyn Class>> {
        None
    }

    fn interfaces(&self) -> &[Rc<dyn std::any::Any>] {
        &[]
    }
}

pub struct SystemClass {
    fields: Vec<Rc<Field>>,
}

impl SystemClass {
    pub fn new() -> Self {
        let fields = vec![Rc::new(Field {
            name: "out".into(),
            value: FieldValue::Reference(None),
        })];
        Self { fields }
    }
}

impl Default for SystemClass {
    fn default() -> Self {
        Self::new()
    }
}

impl Class for SystemClass {
    fn methods(&self) -> &[Rc<Method>] {
        &[]
    }

    fn static_fields(&self) -> &[Rc<Field>] {
        self.fields.as_slice()
    }

    fn instance_fields(&self) -> &[String] {
        &[]
    }

    fn package(&self) -> &str {
        "java/lang"
    }

    fn name(&self) -> &str {
        "System"
    }

    fn super_class(&self) -> Option<Rc<dyn Class>> {
        None
    }

    fn interfaces(&self) -> &[Rc<dyn std::any::Any>] {
        &[]
    }
}

pub struct ObjectClass {}

pub struct StringClass {}

impl StringClass {
    pub fn new() -> StringClass {
        StringClass {}
    }
}

impl Default for StringClass {
    fn default() -> Self {
        StringClass::new()
    }
}

impl Class for StringClass {
    fn methods(&self) -> &[Rc<Method>] {
        &[]
    }

    fn static_fields(&self) -> &[Rc<Field>] {
        &[]
    }

    fn instance_fields(&self) -> &[String] {
        &[]
    }

    fn package(&self) -> &str {
        "java/lang"
    }

    fn name(&self) -> &str {
        "String"
    }

    fn super_class(&self) -> Option<Rc<dyn Class>> {
        None
    }

    fn interfaces(&self) -> &[Rc<dyn std::any::Any>] {
        &[]
    }
}

pub struct StringInstance {
    class: Rc<dyn Class>,
    pub string: String,
}

impl ClassInstance for StringInstance {
    fn class(&self) -> Rc<dyn Class> {
        self.class.clone()
    }

    fn instance_fields(&self) -> &[Rc<Field>] {
        &[]
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
