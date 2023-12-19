//! TODO: Replace with abstract class once this feature exists

use std::{any::Any, rc::Rc};

use super::FileInputStream;
use crate::class::{Class, ClassInstance, Field, Method};

#[derive(Default)]
pub struct InputStream {
    file_input_stream: FileInputStream,
}

impl InputStream {
    pub fn new_instance(self: &Rc<Self>) -> InputStreamInstance {
        InputStreamInstance {
            class: self.clone(),
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

    fn instance_fields(&self) -> &[String] {
        self.file_input_stream.instance_fields()
    }

    fn package(&self) -> &str {
        "java/io"
    }

    fn name(&self) -> &str {
        "InputStream"
    }

    fn super_class(&self) -> Option<Rc<dyn Class>> {
        None
    }

    fn interfaces(&self) -> &[Rc<dyn std::any::Any>] {
        self.file_input_stream.interfaces()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct InputStreamInstance {
    class: Rc<dyn Class>,
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
}
