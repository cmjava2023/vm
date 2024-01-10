use std::{any::Any, cell::RefCell, rc::Rc};

use crate::class::{
    builtin_classes::{InputStream, PrintStream},
    class_identifier, Class, ClassIdentifier, ClassInstance, Field,
    FieldDescriptor, FieldValue, Method,
};

pub struct SystemClass {
    class_identifier: ClassIdentifier,
    object_class: Rc<dyn Class>,
    fields: Vec<Rc<Field>>,
}

impl SystemClass {
    pub fn new(
        print_stream_class: &Rc<PrintStream>,
        // TODO: replace with FileInputStream when InputStream becomes abstract
        file_input_stream_class: &Rc<InputStream>,
        object_class: Rc<dyn Class>,
    ) -> Self {
        let fields = vec![
            Rc::new(Field {
                name: "out".into(),
                value: RefCell::new(FieldValue::Reference(Some(Rc::new(
                    print_stream_class.new_instance(),
                )))),
            }),
            Rc::new(Field {
                name: "in".into(),
                value: RefCell::new(FieldValue::Reference(Some(Rc::new(
                    file_input_stream_class.new_instance(),
                )))),
            }),
        ];
        Self {
            class_identifier: class_identifier!(java / lang, System),
            fields,
            object_class,
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

    fn instance_fields(&self) -> &[FieldDescriptor] {
        &[]
    }

    fn class_identifier(&self) -> &crate::class::ClassIdentifier {
        &self.class_identifier
    }

    fn super_class(&self) -> Option<Rc<dyn Class>> {
        Some(self.object_class.clone())
    }

    fn interfaces(&self) -> &[Rc<dyn std::any::Any>] {
        &[]
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn new_instance(&self, cls: Rc<dyn Class>) -> Rc<dyn ClassInstance> {
        // make sure that self and cls really are equal
        let _cls_ref: &Self =
            cls.as_ref().as_any().downcast_ref::<Self>().unwrap();

        todo!()
    }
}
