use cmjava::classloader::{
    attribute_parser::parse_attributes, file_parser::parse,
};
fn main() {
    let raw_class = parse("HelloWorld.class").unwrap();
    let class = parse_attributes(raw_class);
    println!("{:#?}", class)
}
