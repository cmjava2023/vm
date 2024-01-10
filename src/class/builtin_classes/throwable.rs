use std::{any::Any, cell::OnceCell, rc::Rc};

use crate::{
    class::{
        class_identifier, ArgumentKind, Class, ClassIdentifier, ClassInstance,
        Field, FieldDescriptor, FieldValue, Method, MethodCode,
        RustMethodReturn, SimpleArgumentKind,
    },
    executor::{local_variables::VariableValueOrValue, Frame},
};

pub struct ThrowableClass {
    class_identifier: ClassIdentifier,
    object_class: Rc<dyn Class>,
    methods: Vec<Rc<Method>>,
}

impl ThrowableClass {
    pub fn new(object_class: Rc<dyn Class>) -> Self {
        Self {
            object_class,
            class_identifier: class_identifier!(java / lang, Throwable),
            methods: vec![
                Rc::new(Method {
                    code: MethodCode::Rust(init),
                    name: "<init>".to_owned(),
                    parameters: vec![ArgumentKind::Simple(
                        SimpleArgumentKind::Class(
                            "java/lang/String".to_string(),
                        ),
                    )],
                    return_type: None,
                    is_static: false,
                }),
                Rc::new(Method {
                    code: MethodCode::Rust(get_message),
                    name: "getMessage".to_owned(),
                    parameters: vec![],
                    return_type: Some(ArgumentKind::Simple(
                        SimpleArgumentKind::Class(
                            "java/lang/String".to_string(),
                        ),
                    )),
                    is_static: false,
                }),
            ],
        }
    }
}

fn get_message(frame: &mut Frame) -> RustMethodReturn {
    let instance: Rc<dyn ClassInstance> = match frame.local_variables.get(0) {
        VariableValueOrValue::Reference(s) => s.expect("null pointer"),
        _ => panic!("local variables have reference at index 0"),
    };
    let instance = match instance.as_any().downcast_ref::<ThrowableInstance>() {
        Some(i) => i,
        None => panic!("got {:?} expected Throwable", instance),
    };

    let message = instance
        .message
        .get()
        .expect("message has been initialized");

    RustMethodReturn::Value(FieldValue::Reference(message.clone()))
}

fn init(frame: &mut Frame) -> RustMethodReturn {
    let instance: Rc<dyn ClassInstance> = match frame.local_variables.get(0) {
        VariableValueOrValue::Reference(s) => s.expect("null pointer"),
        _ => panic!("local variables have reference at index 0"),
    };
    let message: Option<Rc<dyn ClassInstance>> =
        match frame.local_variables.get(1) {
            VariableValueOrValue::Reference(s) => s,
            _ => panic!("local variables have message at index 1"),
        };

    let instance = match instance.as_any().downcast_ref::<ThrowableInstance>() {
        Some(i) => i,
        None => panic!("got {:?} expected Throwable", instance),
    };

    instance
        .message
        .set(message)
        .expect("message has not been set");

    RustMethodReturn::Void
}

impl Class for ThrowableClass {
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

        Rc::new(ThrowableInstance {
            class: cls.clone(),
            object_instance: self
                .object_class
                .new_instance(self.object_class.clone()),
            message: OnceCell::new(),
        })
    }
}

pub struct ThrowableInstance {
    class: Rc<dyn Class>,
    object_instance: Rc<dyn ClassInstance>,
    message: OnceCell<Option<Rc<dyn ClassInstance>>>,
}

impl ClassInstance for ThrowableInstance {
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
