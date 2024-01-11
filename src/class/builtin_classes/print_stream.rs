use std::{any::Any, rc::Rc};

use crate::{
    class::{
        builtin_classes::StringInstance, class_identifier, ArgumentKind, Class,
        ClassIdentifier, ClassInstance, Field, FieldDescriptor, Method,
        MethodCode, RustMethodReturn, SimpleArgumentKind,
    },
    executor::{local_variables::VariableValueOrValue, Frame},
};

pub struct PrintStream {
    class_identifier: ClassIdentifier,
    object_class: Rc<dyn Class>,
    methods: Vec<Rc<Method>>,
}

impl PrintStream {
    pub fn new(object_class: Rc<dyn Class>) -> PrintStream {
        PrintStream {
            object_class,
            class_identifier: class_identifier!(java / io, PrintStream),
            methods: vec![
                Rc::new(Method {
                    code: MethodCode::Rust(println_object),
                    name: "println".to_owned(),
                    parameters: vec![ArgumentKind::Simple(
                        SimpleArgumentKind::Class(
                            "java/lang/Object".to_string(),
                        ),
                    )],
                    return_type: None,
                    is_static: false,
                }),
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
            object_instance: self
                .object_class
                .new_instance(self.object_class.clone()),
        }
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
    let boolean = match boolean {
        VariableValueOrValue::Int(b) => b,
        _ => panic!("local variables have int (boolean) to print at index 1"),
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
    let c = match c {
        VariableValueOrValue::Int(c) => c,
        _ => panic!("local variables have int (char) to print at index 1"),
    };
    println!(
        "{}",
        char::from_u32(c as u32).unwrap_or(char::REPLACEMENT_CHARACTER)
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
    let int: i32 = int.try_into().unwrap();
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

fn println_object(frame: &mut Frame) -> RustMethodReturn {
    let object: Option<Rc<dyn ClassInstance>> =
        frame.local_variables.get(1).try_into().unwrap();
    match object {
        None => println!("null"),
        Some(object) => {
            println!("{}@{:p}", object.class().class_identifier(), object)
        },
    }

    RustMethodReturn::Void
}

impl Class for PrintStream {
    fn methods(&self) -> &[Rc<Method>] {
        self.methods.as_slice()
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

pub struct PrintStreamInstance {
    class: Rc<dyn Class>,
    object_instance: Rc<dyn ClassInstance>,
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

    fn parent_instance(&self) -> Option<Rc<dyn ClassInstance>> {
        Some(self.object_instance.clone())
    }
}
