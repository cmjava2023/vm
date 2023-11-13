use super::cp_decoder::RuntimeCPEntry;
use crate::{
    class::{BytecodeClass, Field, Method},
    classloader::{cp_decoder::decode_constant_pool, ClassFile},
};

fn create_bytecode_methods(
    _class_file: &ClassFile,
    _runtime_cp: &[RuntimeCPEntry],
) -> Vec<Method> {
    todo!();
}

fn create_bytecode_fields(
    _class_file: &ClassFile,
    _runtime_cp: &[RuntimeCPEntry],
) -> Vec<Field> {
    todo!();
}

fn create_bytecode_instance_fields(
    _class_file: &ClassFile,
    _runtime_cp: &[RuntimeCPEntry],
) -> Vec<String> {
    todo!();
}

pub fn create_bytecode_class(class_file: &ClassFile) -> BytecodeClass {
    let runtime_cp = decode_constant_pool(class_file);

    let methods = create_bytecode_methods(class_file, &runtime_cp);
    let static_fields = create_bytecode_fields(class_file, &runtime_cp);
    let instance_fields =
        create_bytecode_instance_fields(class_file, &runtime_cp);
    let class: &RuntimeCPEntry = &runtime_cp
        [<u16 as std::convert::Into<usize>>::into(class_file.this_class)];
    let (package, name) = class
        .as_class()
        .unwrap()
        .rsplit_once('/')
        .expect("Class has a package");
    let super_class = None;
    let interfaces = Vec::new();
    BytecodeClass {
        methods,
        static_fields,
        instance_fields,
        package: package.to_string(),
        name: name.to_string(),
        super_class,
        interfaces,
    }
}
