use std::{collections::HashMap, rc::Rc};

use crate::{
    class::{
        builtin_classes::{
            array::{
                Array, BoolArray, BoolArrayInstance, ByteArray,
                ByteArrayInstance, CharArray, CharArrayInstance, DoubleArray,
                DoubleArrayInstance, FloatArray, FloatArrayInstance, IntArray,
                IntArrayInstance, LongArray, LongArrayInstance,
                ObjectArrayKind, ShortArray, ShortArrayInstance,
            },
            FileInputStream, InputStream, PrintStream, StringClass,
            StringInstance, SystemClass,
        },
        Class,
    },
    executor::op_code::ArrayReferenceKinds,
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
    classes: HashMap<String, Rc<dyn Class>>,
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

        let mut classes: HashMap<String, Rc<dyn Class>> = HashMap::new();
        classes.insert("java/lang/String".to_string(), string_class.clone());
        classes.insert("java/io/PrintStream".to_string(), print_stream_class);
        classes.insert("java/io/InputStream".to_string(), input_stream_class);
        classes.insert(
            "java/io/FileInputStream".to_string(),
            file_input_stream_class,
        );
        classes.insert("java/lang/System".to_string(), system_class);
        classes.insert(
            boolean_array_class.name().to_string(),
            boolean_array_class.clone(),
        );
        classes.insert(
            byte_array_class.name().to_string(),
            byte_array_class.clone(),
        );
        classes.insert(
            char_array_class.name().to_string(),
            char_array_class.clone(),
        );
        classes.insert(
            double_array_class.name().to_string(),
            double_array_class.clone(),
        );
        classes.insert(
            float_array_class.name().to_string(),
            float_array_class.clone(),
        );
        classes.insert(
            long_array_class.name().to_string(),
            long_array_class.clone(),
        );
        classes.insert(
            int_array_class.name().to_string(),
            int_array_class.clone(),
        );
        classes.insert(
            short_array_class.name().to_string(),
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

    pub fn find_array_class(
        &mut self,
        kind: ArrayReferenceKinds,
        dimensions: u8,
    ) -> Option<Rc<dyn Class>> {
        let mut class_name: String =
            vec!['['; dimensions.into()].into_iter().collect();
        if let Some(c) = match &kind {
            ArrayReferenceKinds::Boolean => Some('Z'),
            ArrayReferenceKinds::Byte => Some('B'),
            ArrayReferenceKinds::Char => Some('C'),
            ArrayReferenceKinds::Double => Some('D'),
            ArrayReferenceKinds::Float => Some('F'),
            ArrayReferenceKinds::Long => Some('J'),
            ArrayReferenceKinds::Int => Some('I'),
            ArrayReferenceKinds::Short => Some('S'),
            ArrayReferenceKinds::Object(c) => {
                class_name.push('L');
                class_name.push_str(c.package());
                class_name.push('/');
                class_name.push_str(c.name());
                class_name.push(';');
                None
            },
        } {
            class_name.push(c);
        };

        // easy case: class already exists
        if let Some(array_class) = self.find_class(&class_name) {
            Some(array_class.clone())
        } else {
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
                // step 1.1
                if let Some(c) = self.find_array_class(kind, dimensions - 1) {
                    c.clone()
                } else {
                    return None;
                }
            } else {
                // step 1.2
                match &kind {
                    ArrayReferenceKinds::Object(c) => c.clone(),
                    // Note: this assumes that
                    // all 1-dimenensional, primitive array classes
                    // are created during heap setup
                    _ => panic!(
                        "dimension is 1, \
but a primitve array kind (got '{:?}') \
would create a 2 dimensional array here!",
                        &kind
                    ),
                }
            };

            // step 2
            let array_class =
                Rc::new(Array::new(ObjectArrayKind::new(scalar_class)));
            // step 3
            self.classes.insert(class_name, array_class.clone());
            Some(array_class)
        }
    }
}

impl Default for Heap {
    fn default() -> Self {
        Heap::new()
    }
}
