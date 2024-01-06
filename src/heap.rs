use std::{collections::HashMap, rc::Rc};

use crate::class::{
    builtin_classes::{
        array::{
            Array, BoolArray, BoolArrayInstance, ByteArray, ByteArrayInstance,
            CharArray, CharArrayInstance, DoubleArray, DoubleArrayInstance,
            FloatArray, FloatArrayInstance, IntArray, IntArrayInstance,
            LongArray, LongArrayInstance, ObjectArrayKind, ShortArray,
            ShortArrayInstance,
        },
        FileInputStream, InputStream, ObjectClass, PrintStream, StringClass,
        StringInstance, SystemClass,
    },
    ArrayName, Class, ClassIdentifier, ClassName,
};

pub struct Heap {
    string_class: Rc<StringClass>,
    boolean_array_class: Rc<BoolArray>,
    byte_array_class: Rc<ByteArray>,
    char_array_class: Rc<CharArray>,
    double_array_class: Rc<DoubleArray>,
    float_array_class: Rc<FloatArray>,
    long_array_class: Rc<LongArray>,
    int_array_class: Rc<IntArray>,
    short_array_class: Rc<ShortArray>,
    classes: HashMap<ClassIdentifier, Rc<dyn Class>>,
}

impl Heap {
    pub fn new() -> Heap {
        let boolean_array_class = Rc::new(BoolArray::default());
        let byte_array_class = Rc::new(ByteArray::default());
        let char_array_class = Rc::new(CharArray::default());
        let double_array_class = Rc::new(DoubleArray::default());
        let float_array_class = Rc::new(FloatArray::default());
        let long_array_class = Rc::new(LongArray::default());
        let int_array_class = Rc::new(IntArray::default());
        let short_array_class = Rc::new(ShortArray::default());

        let string_class = Rc::new(StringClass::default());
        let print_stream_class = Rc::new(PrintStream::default());
        let input_stream_class = Rc::new(InputStream::default());
        let file_input_stream_class = Rc::new(FileInputStream::default());
        let system_class =
            Rc::new(SystemClass::new(&print_stream_class, &input_stream_class));
        let object_class = Rc::new(ObjectClass::default());

        let mut classes: HashMap<ClassIdentifier, Rc<dyn Class>> =
            HashMap::new();
        classes.insert(
            string_class.class_identifier().clone(),
            string_class.clone(),
        );
        classes.insert(
            print_stream_class.class_identifier().clone(),
            print_stream_class,
        );
        classes.insert(
            input_stream_class.class_identifier().clone(),
            input_stream_class,
        );
        classes.insert(
            file_input_stream_class.class_identifier().clone(),
            file_input_stream_class,
        );
        classes.insert(system_class.class_identifier().clone(), system_class);
        classes.insert(object_class.class_identifier().clone(), object_class);
        classes.insert(
            boolean_array_class.class_identifier().clone(),
            boolean_array_class.clone(),
        );
        classes.insert(
            char_array_class.class_identifier().clone(),
            char_array_class.clone(),
        );
        classes.insert(
            double_array_class.class_identifier().clone(),
            double_array_class.clone(),
        );
        classes.insert(
            float_array_class.class_identifier().clone(),
            float_array_class.clone(),
        );
        classes.insert(
            byte_array_class.class_identifier().clone(),
            byte_array_class.clone(),
        );
        classes.insert(
            long_array_class.class_identifier().clone(),
            long_array_class.clone(),
        );
        classes.insert(
            int_array_class.class_identifier().clone(),
            int_array_class.clone(),
        );
        classes.insert(
            short_array_class.class_identifier().clone(),
            short_array_class.clone(),
        );

        Heap {
            string_class,
            boolean_array_class,
            byte_array_class,
            char_array_class,
            double_array_class,
            float_array_class,
            long_array_class,
            int_array_class,
            short_array_class,
            classes,
        }
    }

    pub fn new_string(&self, string: String) -> StringInstance {
        self.string_class.new_instance(string)
    }

    pub fn new_boolean_array(&self, length: usize) -> BoolArrayInstance {
        self.boolean_array_class.new_instance(length)
    }

    pub fn new_byte_array(&self, length: usize) -> ByteArrayInstance {
        self.byte_array_class.new_instance(length)
    }

    pub fn new_char_array(&self, length: usize) -> CharArrayInstance {
        self.char_array_class.new_instance(length)
    }

    pub fn new_double_array(&self, length: usize) -> DoubleArrayInstance {
        self.double_array_class.new_instance(length)
    }

    pub fn new_float_array(&self, length: usize) -> FloatArrayInstance {
        self.float_array_class.new_instance(length)
    }

    pub fn new_long_array(&self, length: usize) -> LongArrayInstance {
        self.long_array_class.new_instance(length)
    }

    pub fn new_int_array(&self, length: usize) -> IntArrayInstance {
        self.int_array_class.new_instance(length)
    }

    pub fn new_short_array(&self, length: usize) -> ShortArrayInstance {
        self.short_array_class.new_instance(length)
    }

    pub fn add_class(
        &mut self,
        fully_qualified_name: ClassIdentifier,
        class: Rc<dyn Class>,
    ) {
        self.classes.insert(fully_qualified_name, class);
    }

    pub fn find_class(
        &self,
        fully_qualified_name: &ClassIdentifier,
    ) -> Option<&Rc<dyn Class>> {
        self.classes.get(fully_qualified_name)
    }

    pub fn find_array_class(
        &mut self,
        class_identifier: &ClassIdentifier,
    ) -> Option<Rc<dyn Class>> {
        // easy case: class already exists
        if let Some(array_class) = self.find_class(class_identifier) {
            Some(array_class.clone())
        } else {
            let (package, (dimensions, name)) =
                class_identifier.clone().into_array_identifier();
            // more difficult case: array class does not exist yet
            // So, it needs to be created.
            // To do that, perform the following steps:
            // 1. determine the class of values stored in the array
            //  1.1 either (dimension > 1): class of n-1 dimensional array
            //  1.2 or (dimension == 1): component class of array
            // 2. create a new ObjectArray class for the scalar class
            //      from step 1
            // 3. insert the new class into the heap and return it

            // step 1
            let scalar_class = if dimensions > 1 {
                let identifier_with_less_dim = ClassIdentifier {
                    package,
                    class_name: ClassName::Array {
                        dimensions: dimensions - 1,
                        name: name.clone(),
                    },
                };
                // step 1.1
                if let Some(c) =
                    self.find_array_class(&identifier_with_less_dim)
                {
                    c.clone()
                } else {
                    return None;
                }
            } else {
                // step 1.2
                match name {
                    ArrayName::Class(c) => {
                        let sclar_class_indentifier = ClassIdentifier {
                            package,
                            class_name: ClassName::Plain(c),
                        };
                        self.find_class(&sclar_class_indentifier)?.clone()
                    },
                    // Note: this assumes that
                    // all 1-dimenensional, primitive array classes
                    // are created during heap setup
                    _ => panic!(
                        "dimension is 1, \
but a primitve array kind (got '{:?}') \
would create a 2 dimensional array here!",
                        &name
                    ),
                }
            };

            // step 2
            let array_class =
                Rc::new(Array::new(ObjectArrayKind::new(scalar_class)));
            // step 3
            self.classes.insert(
                array_class.class_identifier().clone(),
                array_class.clone(),
            );
            Some(array_class)
        }
    }
}

impl Default for Heap {
    fn default() -> Self {
        Heap::new()
    }
}
