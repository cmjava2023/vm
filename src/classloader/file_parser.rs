use std::path::Path;

use anyhow::Context;
use enumflags2::BitFlags;
use nom::{
    bytes::complete::tag,
    combinator::{eof, map},
    error::ErrorKind,
    multi::{length_count, length_value, many_till},
    number::complete::{be_f32, be_f64, be_i32, be_i64, be_u16, be_u32, be_u8},
    IResult,
};

use super::{
    CpInfo, MethodAccessFlag, RawAttributeInfo, RawClassFile, RawFieldInfo,
    RawMethodInfo, ReferenceKind,
};
use crate::classloader::{ClassAccessFlag, FieldAccessFlag};

fn parse_utf8_code_point(current_content: &[u8]) -> IResult<&[u8], char> {
    let tag_content = current_content;
    let (current_content, first_byte) = be_u8(current_content)?;
    if first_byte == 0b11101101 {
        let (current_content, v) = be_u8(current_content)?;
        let (current_content, w) = be_u8(current_content)?;
        let (current_content, _) = tag(&[(0b11101101 as u8)])(current_content)?;
        let (current_content, y) = be_u8(current_content)?;
        let (current_content, z) = be_u8(current_content)?;
        let v: u32 = v.into();
        let w: u32 = w.into();
        let y: u32 = y.into();
        let z: u32 = z.into();
        let code_point = 0x10000
            + ((v & 0x0f) << 16)
            + ((w & 0x3f) << 10)
            + ((y & 0x0f) << 6)
            + (z & 0x3f);
        Ok((
            current_content,
            char::from_u32(code_point).expect("Byte is valid utf8"),
        ))
    } else if (first_byte & (0b111 << 5)) == (0b111 << 5) {
        let (current_content, y) = be_u8(current_content)?;
        let (current_content, z) = be_u8(current_content)?;
        let x: u32 = first_byte.into();
        let y: u32 = y.into();
        let z: u32 = z.into();
        let code_point = ((x & 0xf) << 12) + ((y & 0x3f) << 6) + (z & 0x3f);
        Ok((
            current_content,
            char::from_u32(code_point).expect("Byte is valid utf8"),
        ))
    } else if (first_byte & (0b11 << 6)) == (0b11 << 6) {
        let (current_content, y) = be_u8(current_content)?;
        let x: u32 = first_byte.into();
        let y: u32 = y.into();
        let code_point: u32 = ((x & 0x1f) << 6) + (y & 0x3f);
        Ok((
            current_content,
            char::from_u32(code_point).expect("Byte is valid utf8"),
        ))
    } else if (first_byte & (1 << 7)) == 0 {
        Ok((
            current_content,
            char::from_u32(first_byte.into()).expect("Byte is valid utf8"),
        ))
    } else {
        Err(nom::Err::Failure(nom::error::Error::new(
            tag_content,
            ErrorKind::Tag,
        )))
    }
}

fn parse_utf8_from_constant_pool(
    current_content: &[u8],
) -> IResult<&[u8], String> {
    let (current_content, (code_points, _)) =
        many_till(parse_utf8_code_point, eof)(current_content)?;
    Ok((current_content, code_points.iter().collect()))
}

fn parse_constant_pool(current_content: &[u8]) -> IResult<&[u8], CpInfo> {
    let tag_content = current_content;
    let (current_content, tag) = be_u8(current_content)?;
    match tag {
        7 => {
            let (current_content, name_index) = be_u16(current_content)?;
            Ok((
                current_content,
                CpInfo::ClassInfo {
                    name_index: (name_index),
                },
            ))
        },
        9 => {
            let (current_content, class_index) = be_u16(current_content)?;
            let (current_content, name_and_type_index) =
                be_u16(current_content)?;
            Ok((
                current_content,
                CpInfo::FieldRefInfo {
                    class_index: (class_index),
                    name_and_type_index: (name_and_type_index),
                },
            ))
        },
        10 => {
            let (current_content, class_index) = be_u16(current_content)?;
            let (current_content, name_and_type_index) =
                be_u16(current_content)?;
            Ok((
                current_content,
                CpInfo::MethodRefInfo {
                    class_index: (class_index),
                    name_and_type_index: (name_and_type_index),
                },
            ))
        },
        11 => {
            let (current_content, class_index) = be_u16(current_content)?;
            let (current_content, name_and_type_index) =
                be_u16(current_content)?;
            Ok((
                current_content,
                CpInfo::InterfaceMethodRefInfo {
                    class_index: (class_index),
                    name_and_type_index: (name_and_type_index),
                },
            ))
        },
        8 => {
            let (current_content, string_index) = be_u16(current_content)?;
            Ok((
                current_content,
                CpInfo::StringInfo {
                    string_index: (string_index),
                },
            ))
        },
        3 => {
            // when the byteorder is not big_endian,
            // this produces the wrong number
            let (current_content, int_value) = be_i32(current_content)?;
            Ok((current_content, CpInfo::IntegerInfo(int_value)))
        },
        4 => {
            // when the byteorder is not big_endian,
            // this produces the wrong number
            let (current_content, float_value) = be_f32(current_content)?;
            Ok((current_content, CpInfo::FloatInfo(float_value)))
        },
        5 => {
            // when the byteorder is not big_endian,
            // this produces the wrong number
            let (current_content, long_value) = be_i64(current_content)?;
            Ok((current_content, CpInfo::LongInfo(long_value)))
        },
        6 => {
            // when the byteorder is not big_endian,
            // this produces the wrong number
            let (current_content, float_value) = be_f64(current_content)?;
            Ok((current_content, CpInfo::DoubleInfo(float_value)))
        },
        12 => {
            let (current_content, name_index) = be_u16(current_content)?;
            let (current_content, descriptor_index) = be_u16(current_content)?;
            Ok((
                current_content,
                CpInfo::NameAndTypeInfo {
                    name_index: (name_index),
                    descriptor_index: (descriptor_index),
                },
            ))
        },
        1 => {
            let (current_content, string) = length_value(
                be_u16,
                parse_utf8_from_constant_pool,
            )(current_content)?;
            Ok((current_content, CpInfo::UTF8INFO(string)))
        },
        15 => {
            let (current_content, reference_kind) = be_u8(current_content)?;
            let (current_content, reference_index) = be_u16(current_content)?;
            Ok((
                current_content,
                CpInfo::MethodHandleInfo {
                    reference_kind: (ReferenceKind::try_from(reference_kind)
                        .unwrap()),
                    reference_index: (reference_index),
                },
            ))
        },
        16 => {
            let (current_content, descriptor_index) = be_u16(current_content)?;
            Ok((
                current_content,
                CpInfo::MethodTypeInfo {
                    descriptor_index: (descriptor_index),
                },
            ))
        },
        18 => {
            let (current_content, bootstrap_method_attr_index) =
                be_u16(current_content)?;
            let (current_content, name_and_type_index) =
                be_u16(current_content)?;
            Ok((
                current_content,
                CpInfo::InvokeDynamicInfo {
                    bootstrap_method_attr_index: (bootstrap_method_attr_index),
                    name_and_type_index: (name_and_type_index),
                },
            ))
        },
        _ => Err(nom::Err::Failure(nom::error::Error::new(
            tag_content,
            ErrorKind::Tag,
        ))),
    }
}

pub fn parse_attribute_info(
    current_content: &[u8],
) -> IResult<&[u8], RawAttributeInfo> {
    let (current_content, name_index) = be_u16(current_content)?;
    let (current_content, attributes) =
        length_count(be_u32, be_u8)(current_content)?;
    Ok((
        current_content,
        RawAttributeInfo {
            attribute_name_index: (name_index),
            info: (attributes),
        },
    ))
}

fn parse_field_info(current_content: &[u8]) -> IResult<&[u8], RawFieldInfo> {
    let (current_content, access_flag_byte) = be_u16(current_content)?;
    let field_access_flags =
        BitFlags::<FieldAccessFlag>::from_bits(access_flag_byte).unwrap();
    let (current_content, name_index) = be_u16(current_content)?;
    let (current_content, descriptor_index) = be_u16(current_content)?;
    let (current_content, attributes) =
        length_count(be_u16, parse_attribute_info)(current_content)?;
    Ok((
        current_content,
        RawFieldInfo {
            access_flags: (field_access_flags),
            name_index: (name_index),
            descriptor_index: (descriptor_index),
            attributes: (attributes),
        },
    ))
}

fn parse_method_info(current_content: &[u8]) -> IResult<&[u8], RawMethodInfo> {
    let (current_content, access_flag_byte) = be_u16(current_content)?;
    let method_access_flags =
        BitFlags::<MethodAccessFlag>::from_bits(access_flag_byte).unwrap();
    let (current_content, name_index) = be_u16(current_content)?;
    let (current_content, descriptor_index) = be_u16(current_content)?;
    let (current_content, attributes) =
        length_count(be_u16, parse_attribute_info)(current_content)?;
    Ok((
        current_content,
        RawMethodInfo {
            access_flags: (method_access_flags),
            name_index: (name_index),
            descriptor_index: (descriptor_index),
            attributes: (attributes),
        },
    ))
}

fn parse_class_file(current_content: &[u8]) -> IResult<&[u8], RawClassFile> {
    let current_content = tag(b"\xCA\xFE\xBA\xBE")(current_content)?.0;
    let (current_content, minor_version) = be_u16(current_content)?;
    let (current_content, major_version) = be_u16(current_content)?;
    let (current_content, constant_pool) = length_count(
        map(be_u16, |cnt| cnt - 1),
        parse_constant_pool,
    )(current_content)?;
    let (current_content, access_flag_byte) = be_u16(current_content)?;
    let access_flags =
        BitFlags::<ClassAccessFlag>::from_bits(access_flag_byte).unwrap();
    let (current_content, this_class) = be_u16(current_content)?;
    let (current_content, super_class) = be_u16(current_content)?;
    let (current_content, interfaces) =
        length_count(be_u16, be_u16)(current_content)?;
    let (current_content, fields) =
        length_count(be_u16, parse_field_info)(current_content)?;
    let (current_content, methods) =
        length_count(be_u16, parse_method_info)(current_content)?;
    let (current_content, attributes) =
        length_count(be_u16, parse_attribute_info)(current_content)?;
    Ok((
        current_content,
        RawClassFile {
            minor_version,
            major_version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        },
    ))
}

pub fn parse<P: AsRef<Path>>(path_to_file: P) -> anyhow::Result<RawClassFile> {
    // read input and magic number
    let content =
        std::fs::read(path_to_file).context("File can not be read")?;
    match parse_class_file(&content) {
        Ok((_, class_file)) => Ok(class_file),
        Err(e) => Err(e.to_owned().into()),
    }
}
