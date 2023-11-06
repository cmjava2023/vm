use std::rc::Rc;

use crate::class::{BytecodeClass, Class, Method};

impl Class for BytecodeClass {
    fn methods(&self) -> &[Method] {
        self.methods.as_slice()
    }

    fn static_fields(&self) -> &[super::Field] {
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
}
