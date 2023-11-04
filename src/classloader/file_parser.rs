use nom::combinator::map_res;
use nom::error::make_error;
use strum::IntoEnumIterator;
use std::path::Path;
use std::ptr::null;
use anyhow::Context;
use nom::IResult;
use nom::multi::length_count;
use nom::bytes::complete::{
    tag,
    take
};
use nom::number::complete::{be_u16, be_u8, be_u32, be_f32, be_f64, be_i64, be_i32};
use crate::classloader::ClassAccessFlag;
use enumflags2::BitFlags;
use nom::error::ErrorKind;
use super::{ClassFile, ReferenceKind};
use super::CpInfo;

fn take2(mut content: &[u8]) ->  &[u8]{
    let result = take::<usize,&[u8], ()>(2usize)(content).unwrap();
    content = result.0;
    return result.1;
}

fn parse_constant_pool(current_content: &[u8])-> IResult<&[u8], CpInfo>{
    let tag_content = current_content;
    let (current_content,tag) = be_u8(current_content)?;
    match tag {
        7 =>{
            let (current_content, name_index) = be_u16(current_content)?; 
            Ok((current_content, CpInfo::ClassInfo { name_index: (name_index) }))
        },
        9=>{
            let (current_content, class_index) = be_u16(current_content)?; 
            let (current_content, name_and_type_index) = be_u16(current_content)?;
            Ok((current_content, CpInfo::FieldRefInfo { class_index: (class_index), name_and_type_index: (name_and_type_index) }))
        },
        10=> {
            let (current_content, class_index) = be_u16(current_content)?; 
            let (current_content, name_and_type_index) = be_u16(current_content)?;
            Ok((current_content, CpInfo::MethodRefInfo { class_index: (class_index), name_and_type_index: (name_and_type_index) }))
        },
        11=> {
            let (current_content, class_index) = be_u16(current_content)?; 
            let (current_content, name_and_type_index) = be_u16(current_content)?;
            Ok((current_content, CpInfo::InterfaceMethodRefInfo { class_index: (class_index), name_and_type_index: (name_and_type_index) }))
        },
        8=>{
            let (current_content, string_index) = be_u16(current_content)?; 
            Ok((current_content, CpInfo::StringInfo { string_index: (string_index) }))
        }
        3=>{
            let (current_content, int_value) = be_i32(current_content)?; //when the byteorder is not big_endian, this produces the wrong number
            Ok((current_content, CpInfo::IntegerInfo( int_value )))
        },
        4=>{
            let (current_content, float_value) = be_f32(current_content)?; //when the byteorder is not big_endian, this produces the wrong number
            Ok((current_content, CpInfo::FloatInfo( float_value )))
        },
        5=>{
            let (current_content, long_value) = be_i64(current_content)?; //when the byteorder is not big_endian, this produces the wrong number
            Ok((current_content, CpInfo::LongInfo( long_value )))
        },
        6=>{
            let (current_content, float_value) = be_f64(current_content)?; //when the byteorder is not big_endian, this produces the wrong number
            Ok((current_content, CpInfo::DoubleInfo( float_value )))
        },
        12=>{
            let (current_content, name_index) = be_u16(current_content)?;
            let (current_content, descriptor_index) = be_u16(current_content)?;
            Ok((current_content, CpInfo::NameAndTypeInfo { namae_index: (name_index), descriptor_index: (descriptor_index) }))
        },
        1=>{
            let (current_content, length) = be_u16(current_content)?;
            todo!()
        }
        15=>{
            let (current_content, reference_kind) = be_u8(current_content)?;
            let (current_content, reference_index) = be_u16(current_content)?;
            Ok((current_content, CpInfo::MethodHandleInfo { reference_kind: (ReferenceKind::try_from(reference_kind).unwrap()), reference_index: (reference_index) }))
        },
        16=>{
            let (current_content, descriptor_index) = be_u16(current_content)?;
            Ok((current_content, CpInfo::MethodTypeInfo { descriptor_index: (descriptor_index) }))
        },
        18=>{
            let (current_content, bootstrap_method_attr_index) = be_u16(current_content)?;
            let (current_content, name_and_type_index) = be_u16(current_content)?;
            Ok((current_content, CpInfo::InvokeDynamicInfo { bootstrap_method_attr_index: (bootstrap_method_attr_index), name_and_type_index: (name_and_type_index) }))
        },
        _ => Err(nom::Err::Failure(nom::error::Error::new(tag_content, ErrorKind::Tag)))
    }
}

fn parse_class_file(current_content: &[u8])->IResult<&[u8],ClassFile>{
    let current_content = tag(b"\xCA\xFE\xBA\xBE")(current_content)?.0;
    let (current_content, minor_ver) = be_u16(current_content)?;
    let (current_content, major_ver) = be_u16(current_content)?;
    let (current_content, constant_pool) = length_count(be_u16, parse_constant_pool)(current_content)?;
    let (current_content, access_flag_byte) = be_u16(current_content)?;
    let class_access_flag = BitFlags::<ClassAccessFlag>::from_bits(access_flag_byte);
    let (current_content, super_class) = be_u16(current_content)?; 
    
    todo!()
}

fn parse<P: AsRef<Path>>(path_to_file: P) -> anyhow::Result<ClassFile> {
    //read input and magic number
    let content = std::fs::read(path_to_file).context("File can not be read")?;
    match parse_class_file(&content) {
        Ok((_,class_file))=> Ok(class_file),
        Err(e)=> Err(e.to_owned().into()),
    }
}