use std::{any::Any, rc::Rc};

use crate::class::{
    class_identifier, Class, ClassIdentifier, ClassInstance, Field,
    FieldDescriptor, Method,
};

pub struct StringClass {
    class_identifier: ClassIdentifier,
}

impl StringClass {
    pub fn new() -> StringClass {
        StringClass {
            class_identifier: class_identifier!(java / lang, String),
        }
    }

    pub fn new_instance(self: &Rc<Self>, string: String) -> StringInstance {
        StringInstance {
            class: self.clone(),
            string,
        }
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

    fn instance_fields(&self) -> &[FieldDescriptor] {
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

    fn new_instance(&self, cls: Rc<dyn Class>) -> Rc<dyn ClassInstance> {
        // make sure that self and cls really are equal
        let _cls_ref: &Self =
            cls.as_ref().as_any().downcast_ref::<Self>().unwrap();

        todo!()
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
