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
            methods: vec![
                Rc::new(Method {
                    code: MethodCode::Rust(println),
                    name: "println".to_owned(),
                    parameters: vec![ArgumentKind::Simple(
                        SimpleArgumentKind::Class(
                            "java/lang/String".to_string(),
                        ),
                    )],
                    return_type: None,
                    is_static: false,
                }),
                Rc::new(Method {
                    code: MethodCode::Rust(println_boolean),
                    name: "println".to_owned(),
                    parameters: vec![ArgumentKind::Simple(
                        SimpleArgumentKind::Boolean,
                    )],
                    return_type: None,
                    is_static: false,
                }),
                Rc::new(Method {
                    code: MethodCode::Rust(println_char),
                    name: "println".to_owned(),
                    parameters: vec![ArgumentKind::Simple(
                        SimpleArgumentKind::Char,
                    )],
                    return_type: None,
                    is_static: false,
                }),
                Rc::new(Method {
                    code: MethodCode::Rust(println_double),
                    name: "println".to_owned(),
                    parameters: vec![ArgumentKind::Simple(
                        SimpleArgumentKind::Double,
                    )],
                    return_type: None,
                    is_static: false,
                }),
                Rc::new(Method {
                    code: MethodCode::Rust(println_float),
                    name: "println".to_owned(),
                    parameters: vec![ArgumentKind::Simple(
                        SimpleArgumentKind::Float,
                    )],
                    return_type: None,
                    is_static: false,
                }),
                Rc::new(Method {
                    code: MethodCode::Rust(println_int),
                    name: "println".to_owned(),
                    parameters: vec![ArgumentKind::Simple(
                        SimpleArgumentKind::Int,
                    )],
                    return_type: None,
                    is_static: false,
                }),
                Rc::new(Method {
                    code: MethodCode::Rust(println_long),
                    name: "println".to_owned(),
                    parameters: vec![ArgumentKind::Simple(
                        SimpleArgumentKind::Long,
                    )],
                    return_type: None,
                    is_static: false,
                }),
            ],
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
        _ => panic!("local variables have string to print at index 1"),
    };
    let b: &StringInstance = match string.as_any().downcast_ref() {
        Some(s) => s,
        None => panic!("paramter is not a string but {:?}", string),
    };
    println!("{}", b.string);

    RustMethodReturn::Void
}

fn println_boolean(frame: &mut Frame) -> RustMethodReturn {
    let boolean = frame.local_variables.get(1);
    let boolean: u8 = match boolean {
        VariableValueOrValue::Boolean(b) => b,
        _ => panic!("local variables have boolean to print at index 1"),
    };
    match boolean {
        0 => println!("false"),
        1 => println!("true"),
        _ => panic!("invalid boolean value encoding: '{}'", boolean),
    }

    RustMethodReturn::Void
}

fn println_char(frame: &mut Frame) -> RustMethodReturn {
    let c = frame.local_variables.get(1);
    let c: u16 = match c {
        VariableValueOrValue::Char(c) => c,
        _ => panic!("local variables have char to print at index 1"),
    };
    println!(
        "{}",
        char::from_u32(c.into()).unwrap_or(char::REPLACEMENT_CHARACTER)
    );

    RustMethodReturn::Void
}

fn println_double(frame: &mut Frame) -> RustMethodReturn {
    let double = frame.local_variables.get(1);
    let double: f64 = match double {
        VariableValueOrValue::Double(d) => d,
        _ => panic!("local variables have double to print at index 1"),
    };
    println!("{}", double);

    RustMethodReturn::Void
}

fn println_float(frame: &mut Frame) -> RustMethodReturn {
    let float = frame.local_variables.get(1);
    let float: f32 = match float {
        VariableValueOrValue::Float(f) => f,
        _ => panic!("local variables have float to print at index 1"),
    };
    println!("{}", float);

    RustMethodReturn::Void
}

fn println_int(frame: &mut Frame) -> RustMethodReturn {
    let int = frame.local_variables.get(1);
    let int: i32 = int.as_computation_int().unwrap();
    println!("{}", int);

    RustMethodReturn::Void
}

fn println_long(frame: &mut Frame) -> RustMethodReturn {
    let long = frame.local_variables.get(1);
    let long: i64 = match long {
        VariableValueOrValue::Long(l) => l,
        _ => panic!("local variables have long to print at index 1"),
    };
    println!("{}", long);

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
