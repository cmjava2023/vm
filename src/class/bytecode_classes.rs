use std::{any::Any, cell::RefCell, rc::Rc};

use super::access_flags::ClassAccessFlag;
use crate::class::{
    BytecodeClass, Class, ClassInstance, Field, FieldDescriptor, FieldKind,
    FieldValue, Method,
};

impl Class for BytecodeClass {
    fn methods(&self) -> &[Rc<Method>] {
        self.methods.as_slice()
    }

    fn static_fields(&self) -> &[Rc<super::Field>] {
        self.static_fields.as_slice()
    }

    fn instance_fields(&self) -> &[FieldDescriptor] {
        self.instance_fields.as_slice()
    }

    fn class_identifier(&self) -> &super::ClassIdentifier {
        &self.class_identifier
    }

    fn super_class(&self) -> Option<Rc<dyn Class>> {
        Some(self.super_class.clone())
    }

    fn interfaces(&self) -> &[Rc<dyn std::any::Any>] {
        self.interfaces.as_slice()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn has_acc_super(&self) -> bool {
        self.access_flags.contains(ClassAccessFlag::Super)
    }

    fn new_instance(&self, cls: Rc<dyn Class>) -> Rc<dyn ClassInstance> {
        // make sure that self and cls really are equal
        let _cls_ref: &Self =
            cls.as_ref().as_any().downcast_ref::<Self>().unwrap();

        let instance_fields: Vec<Rc<Field>> = self
            .instance_fields
            .iter()
            .map(|f| {
                let default_val = match f.kind {
                    FieldKind::Byte => FieldValue::byte(),
                    FieldKind::Short => FieldValue::short(),
                    FieldKind::Int => FieldValue::int(),
                    FieldKind::Long => FieldValue::long(),
                    FieldKind::Char => FieldValue::char(),
                    FieldKind::Float => FieldValue::float(),
                    FieldKind::Double => FieldValue::double(),
                    FieldKind::Boolean => FieldValue::boolean(),
                    FieldKind::Reference => FieldValue::reference(),
                };
                Rc::new(Field {
                    name: f.name.clone(),
                    value: RefCell::new(default_val),
                })
            })
            .collect();

        let parent_instance =
            self.super_class.new_instance(self.super_class.clone());

        Rc::new(BytecodeClassInstance {
            class: cls,
            instance_fields,
            parent_instance,
        })
    }
}

impl ClassInstance for BytecodeClassInstance {
    fn class(&self) -> Rc<dyn Class> {
        self.class.clone()
    }

    fn instance_fields(&self) -> &[Rc<Field>] {
        self.instance_fields.as_slice()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn parent_instance(&self) -> Option<Rc<dyn ClassInstance>> {
        Some(self.parent_instance.clone())
    }
}

pub struct BytecodeClassInstance {
    pub class: Rc<dyn Class>,
    pub instance_fields: Vec<Rc<Field>>,
    pub parent_instance: Rc<dyn ClassInstance>,
}
