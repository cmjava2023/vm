use enumflags2::bitflags;

#[bitflags]
#[derive(Clone, Copy, Debug)]
#[repr(u16)]
pub enum ClassAccessFlag {
    Public = 0x0001,
    Final = 0x0010,
    Super = 0x0020,
    Interface = 0x0200,
    Abstract = 0x0400,
    Synthetic = 0x1000,
    Annotation = 0x2000,
    Enum = 0x4000,
}

#[bitflags]
#[derive(Clone, Copy, Debug)]
#[repr(u16)]
pub enum FieldAccessFlag {
    Public = 0x0001,
    Private = 0x002,
    Protected = 0x004,
    Static = 0x008,
    Final = 0x0010,
    Volatile = 0x0040,
    Transient = 0x0080,
    Synthetic = 0x1000,
    Enum = 0x4000,
}

#[bitflags]
#[derive(Clone, Copy, Debug)]
#[repr(u16)]
pub enum MethodAccessFlag {
    Public = 0x0001,
    Private = 0x002,
    Protected = 0x004,
    Static = 0x008,
    Final = 0x0010,
    Synchronized = 0x0020,
    Bridge = 0x0040,
    Vargs = 0x0080,
    Native = 0x0100,
    Abstract = 0x0400,
    Strict = 0x0800,
    Synthetic = 0x1000,
}
