use std::{
    any::Any,
    io::{self, Read},
    rc::Rc,
};

use crate::{
    class::{
        ArgumentKind, Class, ClassInstance, Field, FieldValue, Method,
        MethodCode, RustMethodReturn, SimpleArgumentKind,
    },
    executor::Frame,
};

pub struct FileInputStream {
    methods: Vec<Rc<Method>>,
}

impl FileInputStream {
    pub fn new() -> FileInputStream {
        FileInputStream {
            methods: vec![Rc::new(Method {
                code: MethodCode::Rust(read),
                name: "read".to_owned(),
                parameters: vec![],
                return_type: Some(ArgumentKind::Simple(
                    SimpleArgumentKind::Int,
                )),
                is_static: false,
            })],
        }
    }

    pub fn new_instance(self: &Rc<Self>) -> FileInputStreamInstance {
        FileInputStreamInstance {
            class: self.clone(),
        }
    }
}

impl Default for FileInputStream {
    fn default() -> Self {
        FileInputStream::new()
    }
}

fn read(_frame: &mut Frame) -> RustMethodReturn {
    let input = io::stdin()
        .bytes()
        .next()
        .expect("some input on stdin")
        .unwrap();
    RustMethodReturn::Value(FieldValue::Int(input.into()))
}

impl Class for FileInputStream {
    fn methods(&self) -> &[Rc<Method>] {
        self.methods.as_slice()
    }

    fn static_fields(&self) -> &[Rc<Field>] {
        &[]
    }

    fn instance_fields(&self) -> &[String] {
        &[]
    }

    fn package(&self) -> &str {
        "java/io"
    }

    fn name(&self) -> &str {
        "FileInputStream"
    }

    fn super_class(&self) -> Option<Rc<dyn Class>> {
        // TODO inherit InputStream
        None
    }

    fn interfaces(&self) -> &[Rc<dyn std::any::Any>] {
        &[]
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct FileInputStreamInstance {
    class: Rc<dyn Class>,
}

impl ClassInstance for FileInputStreamInstance {
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
