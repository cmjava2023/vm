pub mod array;
pub mod file_input_stream;
pub mod input_stream;
pub mod object;
pub mod print_stream;
pub mod string;
pub mod system;

pub use crate::class::builtin_classes::{
    file_input_stream::{FileInputStream, FileInputStreamInstance},
    input_stream::{InputStream, InputStreamInstance},
    object::{ObjectClass, ObjectInstance},
    print_stream::{PrintStream, PrintStreamInstance},
    string::{StringClass, StringInstance},
    system::SystemClass,
};
