use std::path::Path;
use std::ptr::null;
use anyhow::Context;
use nom::IResult;
use nom::multi::length_count;
use nom::bytes::complete::{
    tag,
    take
};
use nom::number::complete::be_u16;
use super::ClassFile;
use super::CpInfo;

fn take2(mut content: &[u8]) ->  &[u8]{
    let result = take::<usize,&[u8], ()>(2usize)(content).unwrap();
    content = result.0;
    return result.1;
}

fn parse_constant_pool(current_content: &[u8])-> IResult<&[u8], CpInfo>{

    todo!()
}

fn parse_class_file(current_content: &[u8])->IResult<&[u8],ClassFile>{
    let current_content = tag(b"\xCA\xFE\xBA\xBE")(current_content)?.0;
    let (current_content, minor_ver) = be_u16(current_content)?;
    let (current_content, major_ver) = be_u16(current_content)?;
    let (current_content, constant_pool) = length_count(be_u16, parse_constant_pool)(current_content)?;
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