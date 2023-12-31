use nom::{
    multi::length_count,
    number::complete::{be_u16, be_u32, be_u8},
    IResult,
};

use crate::{
    class::ClassIdentifier,
    classloader::{
        file_parser::parse_attribute_info as parse_raw_attribute_info,
        parse_class_identifier,
        raw::{RawAttributeInfo, RawClassFile},
        AttributeInfo, ClassFile, CodeAttribute, CpInfo, ExceptionTable,
        FieldInfo, MethodInfo,
    },
};

fn parse_attribute_info<'a, 'p, T, G, P>(
    raw_attribute: &'p RawAttributeInfo,
    raw_class_file: &'p RawClassFile,
    parse: G,
) -> T
where
    P: Fn(&'a [u8]) -> IResult<&'a [u8], T>,
    G: Fn(&'p RawAttributeInfo, &'p RawClassFile) -> P,
    'p: 'a,
{
    parse(raw_attribute, raw_class_file)(raw_attribute.info.as_ref())
        .unwrap()
        .1
}

fn parse_exception_table(
    current_content: &[u8],
) -> IResult<&[u8], ExceptionTable> {
    let (current_content, start_pc) = be_u16(current_content)?;
    let (current_content, end_pc) = be_u16(current_content)?;
    let (current_content, handler_pc) = be_u16(current_content)?;
    let (current_content, catch_type) = be_u16(current_content)?;
    Ok((
        current_content,
        ExceptionTable {
            start_pc,
            end_pc,
            handler_pc,
            catch_type,
        },
    ))
}

fn parse_code_attribute<'a>(
    _raw_attribute: &'a RawAttributeInfo,
    raw_class_file: &'a RawClassFile,
) -> impl Fn(&[u8]) -> IResult<&[u8], CodeAttribute> + 'a {
    move |current_content: &[u8]| {
        let (current_content, max_stack) = be_u16(current_content)?;
        let (current_content, max_locals) = be_u16(current_content)?;
        let (current_content, codes) =
            length_count(be_u32, be_u8)(current_content)?;
        let (current_content, exception_table) =
            length_count(be_u16, parse_exception_table)(current_content)?;
        let (current_content, raw_attributes) =
            length_count(be_u16, parse_raw_attribute_info)(current_content)?;
        let attributes = raw_attributes
            .into_iter()
            .map(|a| parse_attribute(&a, raw_class_file))
            .collect();
        Ok((
            current_content,
            CodeAttribute {
                max_stack: (max_stack),
                max_locals: (max_locals),
                code: (codes),
                exception_table: (exception_table),
                attributes: (attributes),
            },
        ))
    }
}

fn parse_exception_index_table_entry(
    raw_class_file: &RawClassFile,
) -> impl Fn(&[u8]) -> IResult<&[u8], ClassIdentifier> + '_ {
    move |current_content: &[u8]| {
        let (current_content, index) = be_u16(current_content)?;
        let cp_entry =
            raw_class_file.get_java_cp_entry(index as usize).unwrap();
        let name_index = cp_entry.as_class_info().unwrap();
        let cp_entry = raw_class_file
            .get_java_cp_entry(name_index as usize)
            .unwrap();
        let class_name = cp_entry.as_utf8_info().unwrap();
        let identifier = parse_class_identifier(class_name);
        Ok((current_content, identifier))
    }
}

fn parse_exceptions_attribute<'a>(
    _raw_attribute: &'a RawAttributeInfo,
    raw_class_file: &'a RawClassFile,
) -> impl Fn(&[u8]) -> IResult<&[u8], Vec<ClassIdentifier>> + 'a {
    move |current_content: &[u8]| {
        let (current_content, classes) = length_count(
            be_u16,
            parse_exception_index_table_entry(raw_class_file),
        )(current_content)?;
        Ok((current_content, classes))
    }
}

fn parse_attribute(
    raw_attribute: &RawAttributeInfo,
    raw_class_file: &RawClassFile,
) -> AttributeInfo {
    let name = raw_class_file
        .get_java_cp_entry(Into::<usize>::into(
            raw_attribute.attribute_name_index,
        ))
        .expect("Attribute name index is valid");
    let name = if let CpInfo::UTF8INFO(name) = name {
        name.as_ref()
    } else {
        panic!("Attribute name index does not point to UTF8Info")
    };
    match name {
        "Code" => AttributeInfo::Code(parse_attribute_info(
            raw_attribute,
            raw_class_file,
            parse_code_attribute,
        )),
        "SourceFile" => AttributeInfo::SourceFile,
        "LineNumberTable" => AttributeInfo::LineNumberTable,
        "LocalVariableTable" => AttributeInfo::LocalVariableTable,
        "Exceptions" => AttributeInfo::Exceptions(parse_attribute_info(
            raw_attribute,
            raw_class_file,
            parse_exceptions_attribute,
        )),
        "StackMapTable" => AttributeInfo::StackMapTable,
        _ => panic!("Unknown Attribute {}", name),
    }
}

pub fn parse_attributes(class_file: RawClassFile) -> ClassFile {
    let class_attributes: Vec<AttributeInfo> = class_file
        .attributes
        .iter()
        .map(|a| parse_attribute(a, &class_file))
        .collect();
    let methods: Vec<MethodInfo> = class_file
        .methods
        .iter()
        .map(|m| MethodInfo {
            access_flags: (m.access_flags),
            descriptor_index: (m.descriptor_index),
            name_index: (m.name_index),
            attributes: (m
                .attributes
                .iter()
                .map(|a| parse_attribute(a, &class_file))
                .collect()),
        })
        .collect();
    let fields: Vec<FieldInfo> = class_file
        .fields
        .iter()
        .map(|f| FieldInfo {
            access_flags: (f.access_flags),
            descriptor_index: (f.descriptor_index),
            name_index: (f.name_index),
            attributes: (f
                .attributes
                .iter()
                .map(|a| parse_attribute(a, &class_file))
                .collect()),
        })
        .collect();

    ClassFile {
        access_flags: (class_file.access_flags),
        constant_pool: (class_file.constant_pool),
        this_class: (class_file.this_class),
        super_class: (class_file.super_class),
        interfaces: (class_file.interfaces),
        fields: (fields),
        methods: (methods),
        attributes: (class_attributes),
    }
}
