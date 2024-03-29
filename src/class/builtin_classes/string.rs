use std::{any::Any, rc::Rc};

use crate::class::{
    class_identifier, Class, ClassIdentifier, ClassInstance, Field,
    FieldDescriptor, Method,
};

pub struct StringClass {
    class_identifier: ClassIdentifier,
    object_class: Rc<dyn Class>,
}

impl StringClass {
    pub fn new(object_class: Rc<dyn Class>) -> StringClass {
        StringClass {
            class_identifier: class_identifier!(java / lang, String),
            object_class,
        }
    }

    pub fn new_instance(self: &Rc<Self>, string: String) -> StringInstance {
        StringInstance {
            class: self.clone(),
            object_instance: self
                .object_class
                .new_instance(self.object_class.clone()),
            string,
        }
    }
}

impl Class for StringClass {
    fn methods(&self) -> &[Rc<Method>] {
        &[]
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
        Some(self.object_class.clone())
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

pub struct StringInstance {
    class: Rc<dyn Class>,
    object_instance: Rc<dyn ClassInstance>,
    pub string: String,
}

impl ClassInstance for StringInstance {
    fn class(&self) -> Rc<dyn Class> {
        self.class.clone()
    }

    fn instance_fields(&self) -> &[Rc<Field>] {
        &[]
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn parent_instance(&self) -> Option<Rc<dyn ClassInstance>> {
        Some(self.object_instance.clone())
    }
}
