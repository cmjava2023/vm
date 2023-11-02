use std::ptr::null;

use nom::bytes::complete::{
    tag,
    take
};
use super::ClassFile;

fn take2(mut content: &[u8]) ->  &[u8]{
    let result = take::<usize,&[u8], ()>(2usize)(content).unwrap();
    content = result.0;
    return result.1;
}

fn parse(path_to_file: String) -> ClassFile {
    //read input and magic number
    let content = std::fs::read(path_to_file).expect("File can not be read");
    let mut current_content: &[u8] = &content;
    current_content = tag("0xCAFEBABE")(current_content).expect("Not a valid class file").0;
    let minor_ver = take2(current_content);
    let major_ver = take2(current_content);
}