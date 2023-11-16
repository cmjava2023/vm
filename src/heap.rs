use std::{collections::HashMap, rc::Rc};

use crate::class::{
    builtin_classes::{PrintStream, StringClass, StringInstance, SystemClass},
    Class,
};

pub struct Heap {
    string_class: Rc<StringClass>,
    classes: HashMap<String, Rc<dyn Class>>,
}

impl Heap {
    pub fn new() -> Heap {
        let string_class = Rc::new(StringClass::default());
        let print_stream_class = Rc::new(PrintStream::default());
        let system_class = Rc::new(SystemClass::new(&print_stream_class));

        let mut classes: HashMap<String, Rc<dyn Class>> = HashMap::new();
        classes.insert("java/lang/String".to_string(), string_class.clone());
        classes.insert("java/io/PrintStream".to_string(), print_stream_class);
        classes.insert("java/lang/System".to_string(), system_class);

        Heap {
            string_class,
            classes,
        }
    }

    pub fn new_string(&self, string: String) -> StringInstance {
        self.string_class.new_instance(string)
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
