use std::{any::Any, rc::Rc};

use crate::class::{
    builtin_classes::PrintStream, Class, Field, FieldValue, Method,
};

pub struct SystemClass {
    fields: Vec<Rc<Field>>,
}

impl SystemClass {
    pub fn new(print_stream_class: &Rc<PrintStream>) -> Self {
        let fields = vec![Rc::new(Field {
            name: "out".into(),
            value: FieldValue::Reference(Some(Rc::new(
                print_stream_class.new_instance(),
            ))),
        })];
        Self { fields }
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

    fn as_any(&self) -> &dyn Any {
        self
    }
}
