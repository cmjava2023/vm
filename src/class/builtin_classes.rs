use std::rc::Rc;

use crate::{
    class::{Class, Field, FieldValue, Method},
    executor::{Frame, Update},
};

pub struct PrintStream {}

fn println(_frame: &Frame) -> Update {
    println!("replace me");
    Update::None
}

impl Class for PrintStream {
    fn methods(&self) -> &[Method] {
        &[Method::Rust(println)]
    }

    fn static_fields(&self) -> &[Field] {
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
    fields: Vec<Field>,
}

impl SystemClass {
    pub fn new() -> Self {
        let fields = vec![Field {
            name: "out".into(),
            value: FieldValue::Reference(None),
        }];
        Self { fields }
    }
}

impl Default for SystemClass {
    fn default() -> Self {
        Self::new()
    }
}

impl Class for SystemClass {
    fn methods(&self) -> &[Method] {
        &[]
    }

    fn static_fields(&self) -> &[Field] {
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
