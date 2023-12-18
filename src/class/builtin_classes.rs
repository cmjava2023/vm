pub mod array;
pub mod file_input_stream;
pub mod print_stream;
pub mod string;
pub mod system;

pub use crate::class::builtin_classes::{
    file_input_stream::{FileInputStream, FileInputStreamInstance},
    print_stream::{PrintStream, PrintStreamInstance},
    string::{StringClass, StringInstance},
    system::SystemClass,
};

pub struct ObjectClass {}
