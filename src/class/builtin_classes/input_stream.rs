//! TODO: Replace with abstract class once this feature exists

use std::{any::Any, rc::Rc};

use super::FileInputStream;
use crate::class::{
    class_identifier, Class, ClassIdentifier, ClassInstance, Field,
    FieldDescriptor, Method,
};

pub struct InputStream {
    class_identifier: ClassIdentifier,
    object_class: Rc<dyn Class>,
    file_input_stream: FileInputStream,
}

impl InputStream {
    pub fn new(object_class: Rc<dyn Class>) -> Self {
        InputStream {
            class_identifier: class_identifier!(java / io, InputStream),
            file_input_stream: FileInputStream::new(object_class.clone()),
            object_class,
        }
    }
}

impl InputStream {
    pub fn new_instance(self: &Rc<Self>) -> InputStreamInstance {
        InputStreamInstance {
            class: self.clone(),
            object_instance: self
                .object_class
                .new_instance(self.object_class.clone()),
        }
    }
}

impl Class for InputStream {
    fn methods(&self) -> &[Rc<Method>] {
        self.file_input_stream.methods()
    }

    fn static_fields(&self) -> &[Rc<Field>] {
        self.file_input_stream.static_fields()
    }

    fn instance_fields(&self) -> &[FieldDescriptor] {
        self.file_input_stream.instance_fields()
    }

    fn class_identifier(&self) -> &crate::class::ClassIdentifier {
        &self.class_identifier
    }

    fn super_class(&self) -> Option<Rc<dyn Class>> {
        Some(self.object_class.clone())
    }

    fn interfaces(&self) -> &[Rc<dyn std::any::Any>] {
        self.file_input_stream.interfaces()
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

pub struct InputStreamInstance {
    class: Rc<dyn Class>,
    object_instance: Rc<dyn ClassInstance>,
}

impl ClassInstance for InputStreamInstance {
    fn class(&self) -> Rc<dyn Class> {
        self.class.clone()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn instance_fields(&self) -> &[Rc<Field>] {
        &[]
    }

    fn parent_instance(&self) -> Option<Rc<dyn ClassInstance>> {
        Some(self.object_instance.clone())
    }
}
