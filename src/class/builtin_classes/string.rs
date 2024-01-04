use std::{any::Any, rc::Rc};

use crate::class::{
    class_identifier, Class, ClassIdentifier, ClassInstance, Field, Method,
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
