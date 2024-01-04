use std::{any::Any, rc::Rc};

use crate::class::{
    builtin_classes::{InputStream, PrintStream},
    class_identifier, Class, ClassIdentifier, Field, FieldValue, Method,
};

pub struct SystemClass {
    class_identifier: ClassIdentifier,
    fields: Vec<Rc<Field>>,
}

impl SystemClass {
    pub fn new(
        print_stream_class: &Rc<PrintStream>,
        // TODO: replace with FileInputStream when InputStream becomes abstract
        file_input_stream_class: &Rc<InputStream>,
    ) -> Self {
        let fields = vec![
            Rc::new(Field {
                name: "out".into(),
                value: FieldValue::Reference(Some(Rc::new(
                    print_stream_class.new_instance(),
                ))),
            }),
            Rc::new(Field {
                name: "in".into(),
                value: FieldValue::Reference(Some(Rc::new(
                    file_input_stream_class.new_instance(),
                ))),
            }),
        ];
        Self {
            class_identifier: class_identifier!(java / lang, System),
            fields,
        }
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

    fn class_identifier(&self) -> &crate::class::ClassIdentifier {
        &self.class_identifier
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
