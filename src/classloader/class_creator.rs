use std::rc::Rc;

use super::cp_decoder::RuntimeCPEntry;
use crate::{
    class::{
        ArgumentKind, BytecodeClass, Code, Field, Method, MethodCode,
        SimpleArgumentKind,
    },
    classloader::{
        cp_decoder::{decode_constant_pool, remove_cp_offset},
        opcode_parser::parse_opcodes,
        ClassFile, MethodInfo,
    },
    heap::Heap,
};

enum ArgumentStates {
    BeforeArguments,
    Arguments,
    ReturnType,
    ClassName,
    Array,
}

fn match_argument_char(
    char: char,
) -> (Option<SimpleArgumentKind>, ArgumentStates) {
    match char {
        'B' => (Some(SimpleArgumentKind::Byte), ArgumentStates::Arguments),
        'D' => (Some(SimpleArgumentKind::Double), ArgumentStates::Arguments),
        'F' => (Some(SimpleArgumentKind::Float), ArgumentStates::Arguments),
        'I' => (Some(SimpleArgumentKind::Int), ArgumentStates::Arguments),
        'C' => (Some(SimpleArgumentKind::Char), ArgumentStates::Arguments),
        'J' => (Some(SimpleArgumentKind::Long), ArgumentStates::Arguments),
        'S' => (Some(SimpleArgumentKind::Short), ArgumentStates::Arguments),
        'Z' => (Some(SimpleArgumentKind::Boolean), ArgumentStates::Arguments),
        'L' => (None, ArgumentStates::ClassName),
        '[' => (None, ArgumentStates::Array),
        ')' => (None, ArgumentStates::ReturnType),
        _ => panic!(
            "Unexpected Symbol for method parameter kind, found {}",
            char
        ),
    }
}

fn parse_method_arguments(
    descriptor: &str,
) -> (Vec<ArgumentKind>, Option<ArgumentKind>) {
    let mut parameters = Vec::default();
    let mut state = ArgumentStates::BeforeArguments;
    let mut array_dim_counter = 0;
    let mut current_class_name: String = "".to_string();
    let mut return_type = None;
    let mut return_state = false;
    for char in descriptor.chars() {
        match state {
            ArgumentStates::BeforeArguments => {
                if char != '(' {
                    panic!(
                        "Method Arguments need to start with '(', found '{}' ",
                        char
                    )
                }
                state = ArgumentStates::Arguments;
            },
            ArgumentStates::Arguments => {
                let result = match_argument_char(char);
                state = result.1;
                if let Some(argument) = result.0 {
                    parameters.push(ArgumentKind::Simple(argument))
                }
            },
            ArgumentStates::ReturnType => {
                match char {
                    'D' => {
                        return_type = Some(ArgumentKind::Simple(
                            SimpleArgumentKind::Double,
                        ))
                    },
                    'F' => {
                        return_type = Some(ArgumentKind::Simple(
                            SimpleArgumentKind::Float,
                        ))
                    },
                    'B' => {
                        return_type =
                            Some(ArgumentKind::Simple(SimpleArgumentKind::Byte))
                    },
                    'I' => {
                        return_type =
                            Some(ArgumentKind::Simple(SimpleArgumentKind::Int))
                    },
                    'C' => {
                        return_type =
                            Some(ArgumentKind::Simple(SimpleArgumentKind::Char))
                    },
                    'J' => {
                        return_type =
                            Some(ArgumentKind::Simple(SimpleArgumentKind::Long))
                    },
                    'S' => {
                        return_type = Some(ArgumentKind::Simple(
                            SimpleArgumentKind::Short,
                        ))
                    },
                    'Z' => {
                        return_type = Some(ArgumentKind::Simple(
                            SimpleArgumentKind::Boolean,
                        ))
                    },
                    'V' => return_type = None,
                    'L' => {
                        state = ArgumentStates::ClassName;
                        return_state = true;
                    },
                    '[' => {
                        state = ArgumentStates::Array;
                        return_state = true;
                    },
                    _ => panic!(
                        "Unexpected Symbol for method return kind, found {}",
                        char
                    ),
                };
            },
            ArgumentStates::ClassName => {
                if char == ';' {
                    if array_dim_counter > 0 {
                        if return_state {
                            return_type = Some(ArgumentKind::Array {
                                dimensions: (array_dim_counter),
                                kind: (SimpleArgumentKind::Class(
                                    current_class_name.to_string(),
                                )),
                            });
                        } else {
                            parameters.push(ArgumentKind::Array {
                                dimensions: (array_dim_counter),
                                kind: (SimpleArgumentKind::Class(
                                    current_class_name.to_string(),
                                )),
                            });
                        }

                        array_dim_counter = 0;
                    } else if return_state {
                        return_type = Some(ArgumentKind::Simple(
                            SimpleArgumentKind::Class(
                                current_class_name.to_string(),
                            ),
                        ));
                    } else {
                        parameters.push(ArgumentKind::Simple(
                            SimpleArgumentKind::Class(
                                current_class_name.to_string(),
                            ),
                        ));
                    }
                    current_class_name = "".to_string();
                } else {
                    current_class_name.push(char);
                }
            }, // if Dim > 0 return array
            ArgumentStates::Array => {
                array_dim_counter += 1;
                let result = match_argument_char(char);
                state = result.1;
                if let Some(argument) = result.0 {
                    if return_state {
                        return_type = Some(ArgumentKind::Array {
                            dimensions: (array_dim_counter),
                            kind: (argument),
                        });
                    } else {
                        parameters.push(ArgumentKind::Array {
                            dimensions: (array_dim_counter),
                            kind: (argument),
                        });
                    }
                    array_dim_counter = 0;
                }
            },
        }
    }
    (parameters, return_type)
}

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
    let desc_string = class_file
        .get_java_cp_entry(method.descriptor_index as usize)
        .expect("Valid CP Reference in MethodInfo")
        .as_utf8_info()
        .unwrap();
    let (parameters, return_type) = parse_method_arguments(desc_string);
    Rc::new(Method {
        code: MethodCode::Bytecode(Code {
            stack_depth: stack_depth.into(),
            local_variable_count: local_variable_count.into(),
            byte_code,
        }),
        name: name.to_string(),
        // TODO parse method signature and insert parameter count
        parameter_count: parameters.len(),
        parameters,
        return_type,
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
    let (package, name) = match class.as_class().unwrap().rsplit_once('/') {
        Some(package_and_name) => package_and_name,
        None => ("", class.as_class().unwrap()),
    };

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
