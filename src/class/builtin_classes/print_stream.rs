use std::rc::Rc;

use crate::{
    class::{
        builtin_classes::StringInstance, ArgumentKind, Class, ClassInstance,
        Field, Method, MethodCode, RustMethodReturn, SimpleArgumentKind,
    },
    executor::{local_variables::VariableValueOrValue, Frame},
};

pub struct PrintStream {
    methods: Vec<Rc<Method>>,
}

impl PrintStream {
    pub fn new() -> PrintStream {
        PrintStream {
            methods: vec![Rc::new(Method {
                code: MethodCode::Rust(println),
                name: "println".to_owned(),
                parameters: vec![ArgumentKind::Simple(
                    SimpleArgumentKind::Class("java/lang/String".to_string()),
                )],
                return_type: None,
                is_static: false,
            })],
        }
    }

    pub fn new_instance(self: &Rc<Self>) -> PrintStreamInstance {
        PrintStreamInstance {
            class: self.clone(),
        }
    }
}

impl Default for PrintStream {
    fn default() -> Self {
        PrintStream::new()
    }
}

fn println(frame: &mut Frame) -> RustMethodReturn {
    let string = frame.local_variables.get(1);
    let string: Rc<dyn ClassInstance> = match string {
        VariableValueOrValue::Reference(s) => s.expect("null pointer"),
        _ => panic!("local variables have reference on top"),
    };
    let b: &StringInstance = match string.as_any().downcast_ref() {
        Some(s) => s,
        None => panic!("stack reference is not a string"),
    };
    println!("{}", b.string);

    RustMethodReturn::Void
}

impl Class for PrintStream {
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
        "PrintStream"
    }

    fn super_class(&self) -> Option<Rc<dyn Class>> {
        None
    }

    fn interfaces(&self) -> &[Rc<dyn std::any::Any>] {
        &[]
    }
}

pub struct PrintStreamInstance {
    class: Rc<dyn Class>,
}

impl ClassInstance for PrintStreamInstance {
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
