use cmjava::{
    classloader::{
        attribute_parser::parse_attributes,
        class_creator::create_bytecode_class, file_parser::parse,
    },
    heap::Heap,
};

fn main() {
    let mut heap = Heap::default();
    let raw_class = parse("tests/data/hello_world/Main.class").unwrap();
    let class = parse_attributes(raw_class);
    let bytecode_class = create_bytecode_class(&class, &mut heap);
    println!("{:#?}", bytecode_class)
}
