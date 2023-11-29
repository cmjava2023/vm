pub mod print_stream;
pub mod string;
pub mod system;

pub use crate::class::builtin_classes::{
    print_stream::{PrintStream, PrintStreamInstance},
    string::{StringClass, StringInstance},
    system::SystemClass,
};

pub struct ObjectClass {}
