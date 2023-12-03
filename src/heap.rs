use std::{collections::HashMap, rc::Rc};

use crate::class::{
    builtin_classes::{
        array::{ByteArray, ByteArrayInstance},
        PrintStream, StringClass, StringInstance, SystemClass,
    },
    Class,
};

pub struct Heap {
    string_class: Rc<StringClass>,
    byte_array_class: Rc<ByteArray>,
    classes: HashMap<String, Rc<dyn Class>>,
}

impl Heap {
    pub fn new() -> Heap {
        let byte_array_class = Rc::new(ByteArray::default());

        let string_class = Rc::new(StringClass::default());
        let print_stream_class = Rc::new(PrintStream::default());
        let system_class = Rc::new(SystemClass::new(&print_stream_class));

        let mut classes: HashMap<String, Rc<dyn Class>> = HashMap::new();
        classes.insert("java/lang/String".to_string(), string_class.clone());
        classes.insert("java/io/PrintStream".to_string(), print_stream_class);
        classes.insert("java/lang/System".to_string(), system_class);
        classes.insert(
            byte_array_class.name().to_string(),
            byte_array_class.clone(),
        );

        Heap {
            string_class,
            byte_array_class,
            classes,
        }
    }

    pub fn new_string(&self, string: String) -> StringInstance {
        self.string_class.new_instance(string)
    }

    pub fn new_byte_array(&self, length: usize) -> ByteArrayInstance {
        self.byte_array_class.new_instance(length)
    }

    pub fn add_class(
        &mut self,
        fully_qualified_name: String,
        class: Rc<dyn Class>,
    ) {
        self.classes.insert(fully_qualified_name, class);
    }

    pub fn find_class(
        &self,
        fully_qualified_name: &str,
    ) -> Option<&Rc<dyn Class>> {
        self.classes.get(fully_qualified_name)
    }
}

impl Default for Heap {
    fn default() -> Self {
        Heap::new()
    }
}
