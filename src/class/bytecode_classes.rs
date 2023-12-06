use std::{any::Any, rc::Rc};

use crate::class::{BytecodeClass, Class, ClassInstance, Field, Method};

impl Class for BytecodeClass {
    fn methods(&self) -> &[Rc<Method>] {
        self.methods.as_slice()
    }

    fn static_fields(&self) -> &[Rc<super::Field>] {
        self.static_fields.as_slice()
    }

    fn instance_fields(&self) -> &[String] {
        self.instance_fields.as_slice()
    }

    fn package(&self) -> &str {
        self.package.as_str()
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn super_class(&self) -> Option<Rc<dyn Class>> {
        self.super_class.clone()
    }

    fn interfaces(&self) -> &[Rc<dyn std::any::Any>] {
        self.interfaces.as_slice()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ClassInstance for BytecodeClassInstance {
    fn class(&self) -> Rc<dyn Class> {
        self.class.clone()
    }

    fn instance_fields(&self) -> &[Rc<Field>] {
        self.instance_fields.as_slice()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct BytecodeClassInstance {
    pub class: Rc<dyn Class>,
    pub instance_fields: Vec<Rc<Field>>,
}
