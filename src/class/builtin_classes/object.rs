use std::{any::Any, rc::Rc};

use crate::{
    class::{
        class_identifier, Class, ClassIdentifier, ClassInstance, Field,
        FieldDescriptor, Method, MethodCode, RustMethodReturn,
    },
    executor::Frame,
};

pub struct ObjectClass {
    class_identifier: ClassIdentifier,
    methods: Vec<Rc<Method>>,
}

impl ObjectClass {
    pub fn new() -> Self {
        Self {
            class_identifier: class_identifier!(java / lang, Object),
            methods: vec![Rc::new(Method {
                code: MethodCode::Rust(init),
                name: "<init>".to_owned(),
                parameters: vec![],
                return_type: None,
                is_static: false,
            })],
        }
    }
}

impl Default for ObjectClass {
    fn default() -> Self {
        Self::new()
    }
}

fn init(_frame: &mut Frame) -> RustMethodReturn {
    RustMethodReturn::Void
}

impl Class for ObjectClass {
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

pub struct ObjectInstance {
    class: Rc<dyn Class>,
}

impl ClassInstance for ObjectInstance {
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
