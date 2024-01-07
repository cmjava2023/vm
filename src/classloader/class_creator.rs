pub mod signature_parser;

use std::{ops::Range, rc::Rc};

use super::parse_class_identifier;
use crate::{
    class::{
        access_flags::FieldAccessFlag, BytecodeClass, Code, Field,
        FieldDescriptor, FieldKind, FieldValue, Method, MethodCode,
    },
    classloader::{
        class_creator::signature_parser::parse_method_arguments,
        cp_decoder::{decode_constant_pool, remove_cp_offset, RuntimeCPEntry},
        opcode_parser::parse_opcodes,
        ClassFile, MethodAccessFlag, MethodInfo,
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
    let mut opcode_sizes;
    let mut exeption_table = Vec::new();
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
            (_, (byte_code, opcode_sizes)) = parse_opcodes(
                &code_attribute.code,
                class_file,
                runtime_cp,
                heap,
            )
            .unwrap();

            for exception in &code_attribute.exception_table {
                let mut bytes_count = 0;
                let mut start_pc_code = 0;
                let mut end_pc_code = 0;
                let mut handler_pc_code = 0;
                for (i, code_size) in opcode_sizes.iter_mut().enumerate() {
                    if bytes_count > exception.start_pc
                        && bytes_count > exception.end_pc
                        && bytes_count > exception.handler_pc
                    {
                        break;
                    }
                    if bytes_count == exception.start_pc {
                        start_pc_code = i;
                    }
                    if bytes_count == exception.end_pc {
                        end_pc_code = i;
                    }
                    if bytes_count == exception.handler_pc {
                        handler_pc_code = i;
                    }
                    bytes_count += u16::from(*code_size);
                }
                let identifier = if exception.catch_type == 0 {
                    None
                } else {
                    let name_index = class_file
                        .get_java_cp_entry(exception.catch_type as usize)
                        .unwrap()
                        .as_class_info()
                        .unwrap();
                    let name = class_file
                        .get_java_cp_entry(name_index as usize)
                        .unwrap()
                        .as_utf8_info()
                        .unwrap();
                    Some(parse_class_identifier(name))
                };
                exeption_table.push(crate::class::ExceptionTable {
                    active: Range {
                        start: start_pc_code,
                        end: end_pc_code,
                    },
                    handler_position: handler_pc_code,
                    catch_type: identifier,
                });
            }
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
            exception_table: exeption_table,
            byte_code,
        }),
        name: name.to_string(),
        parameters,
        return_type,
        is_static: method.access_flags.contains(MethodAccessFlag::Static),
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
    class_file: &ClassFile,
    _runtime_cp: &[RuntimeCPEntry],
) -> (Vec<Rc<Field>>, Vec<FieldDescriptor>) {
    let mut instance_fields = Vec::new();
    let mut static_fields = Vec::new();

    for field_info in &class_file.fields {
        if field_info.access_flags.contains(FieldAccessFlag::Static) {
            let name = class_file
                .get_java_cp_entry(field_info.name_index as usize)
                .unwrap()
                .as_utf8_info()
                .unwrap()
                .to_string();
            let desciptor = class_file
                .get_java_cp_entry(field_info.descriptor_index as usize)
                .unwrap()
                .as_utf8_info()
                .unwrap();
            // everything except one of B C D F I J S Z
            // means a non primitive type wich are handled
            // the same for default values
            let value = match desciptor {
                "B" => FieldValue::byte(),
                "C" => FieldValue::char(),
                "D" => FieldValue::double(),
                "F" => FieldValue::float(),
                "I" => FieldValue::int(),
                "J" => FieldValue::long(),
                "S" => FieldValue::short(),
                "Z" => FieldValue::boolean(),
                _ => FieldValue::reference(),
            };
            static_fields.push(Rc::new(Field { name, value }));
        } else {
            let name = class_file
                .get_java_cp_entry(field_info.name_index as usize)
                .unwrap()
                .as_utf8_info()
                .unwrap()
                .to_string();
            let desciptor = class_file
                .get_java_cp_entry(field_info.descriptor_index as usize)
                .unwrap()
                .as_utf8_info()
                .unwrap();
            // everything except one of B C D F I J S Z
            // means a non primitive type
            // wich are handled the same for default values
            let kind = match desciptor {
                "B" => FieldKind::Byte,
                "C" => FieldKind::Char,
                "D" => FieldKind::Double,
                "F" => FieldKind::Float,
                "I" => FieldKind::Int,
                "J" => FieldKind::Long,
                "S" => FieldKind::Short,
                "Z" => FieldKind::Boolean,
                _ => FieldKind::Reference,
            };
            instance_fields.push(FieldDescriptor { name, kind });
        }
    }

    (static_fields, instance_fields)
}

pub fn create_bytecode_class(
    class_file: &ClassFile,
    heap: &mut Heap,
) -> BytecodeClass {
    let runtime_cp = decode_constant_pool(class_file);

    let methods = create_bytecode_methods(class_file, &runtime_cp, heap);
    let (static_fields, instance_fields) =
        create_bytecode_fields(class_file, &runtime_cp);
    let class: &RuntimeCPEntry =
        &runtime_cp[remove_cp_offset(class_file.this_class as usize)];
    let class_identifier = parse_class_identifier(class.as_class().unwrap());

    let super_class = None;
    let interfaces = Vec::new();
    BytecodeClass {
        methods,
        static_fields,
        instance_fields,
        class_identifier,
        super_class,
        interfaces,
    }
}
