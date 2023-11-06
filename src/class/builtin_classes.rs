use std::rc::Rc;

use crate::class::{Class, Method};

#[deprecated(note = "use struct from executer in the future")]
pub struct Frame {}

#[deprecated(note = "use enum from executer in the future")]
pub enum Update {
    None,
}

pub struct PrintStream {}

fn println(frame: &Frame) -> Update {
    println!("replace me");
    Update::None
}

impl Class for PrintStream {
    fn methods(&self) -> &[super::Method] {
        &[Method::Rust(println)]
    }

    fn static_fields(&self) -> &[super::Field] {
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
