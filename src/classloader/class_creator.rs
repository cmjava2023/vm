use std::rc::Rc;

use super::cp_decoder::RuntimeCPEntry;
use crate::{
    class::{BytecodeClass, Code, Field, Method, MethodCode},
    classloader::{
        cp_decoder::{decode_constant_pool, remove_cp_offset},
        opcode_parser::parse_opcodes,
        ClassFile, MethodInfo,
    },
    heap::Heap,
};

fn create_bytecode_method(
    method: &MethodInfo,
    class_file: &ClassFile,
    runtime_cp: &[RuntimeCPEntry],
    heap: &mut Heap,
) -> Rc<Method> {
    let mut byte_code = Vec::new();
    let mut stack_depth = 0;
    let mut local_variable_count = 0;
    let name = class_file
        .get_java_cp_entry(method.name_index as usize)
        .expect("Valid CP Reference in MethodInfo")
        .as_utf8_info()
        .unwrap();
    for element in method.attributes.iter() {
        let attribute = element.as_code_attribute();
        if let Some(code_attribute) = attribute {
            stack_depth = code_attribute.max_stack;
            local_variable_count = code_attribute.max_locals;
            (_, byte_code) = parse_opcodes(
                &code_attribute.code,
                class_file,
                runtime_cp,
                heap,
            )
            .unwrap();
        }
    }
    Rc::new(Method {
        code: MethodCode::Bytecode(Code {
            stack_depth: stack_depth.into(),
            local_variable_count: local_variable_count.into(),
            byte_code,
        }),
        name: name.to_string(),
    })
}

fn create_bytecode_methods(
    class_file: &ClassFile,
    runtime_cp: &[RuntimeCPEntry],
    heap: &mut Heap,
) -> Vec<Rc<Method>> {
    class_file
        .methods
        .iter()
        .map(|e| create_bytecode_method(e, class_file, runtime_cp, heap))
        .collect()
}

fn create_bytecode_fields(
    _class_file: &ClassFile,
    _runtime_cp: &[RuntimeCPEntry],
) -> Vec<Rc<Field>> {
    Vec::new()
}

fn create_bytecode_instance_fields(
    _class_file: &ClassFile,
    _runtime_cp: &[RuntimeCPEntry],
) -> Vec<String> {
    Vec::new()
}

pub fn create_bytecode_class(
    class_file: &ClassFile,
    heap: &mut Heap,
) -> BytecodeClass {
    let runtime_cp = decode_constant_pool(class_file);

    let methods = create_bytecode_methods(class_file, &runtime_cp, heap);
    let static_fields = create_bytecode_fields(class_file, &runtime_cp);
    let instance_fields =
        create_bytecode_instance_fields(class_file, &runtime_cp);
    let class: &RuntimeCPEntry =
        &runtime_cp[remove_cp_offset(class_file.this_class as usize)];
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
