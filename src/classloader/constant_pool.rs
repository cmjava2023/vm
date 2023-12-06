use crate::classloader::ReferenceKind;

#[derive(Debug)]
pub enum CpInfo {
    ClassInfo {
        name_index: u16,
    },
    FieldRefInfo {
        class_index: u16,
        name_and_type_index: u16,
    },
    MethodRefInfo {
        class_index: u16,
        name_and_type_index: u16,
    },
    InterfaceMethodRefInfo {
        class_index: u16,
        name_and_type_index: u16,
    },
    StringInfo {
        string_index: u16,
    },
    IntegerInfo(i32),
    FloatInfo(f32),
    LongInfo(i64),
    DoubleInfo(f64),
    // long/double values count as two slots
    Reserved,
    NameAndTypeInfo {
        name_index: u16,
        descriptor_index: u16,
    },
    UTF8INFO(String),
    MethodHandleInfo {
        reference_kind: ReferenceKind,
        reference_index: u16,
    },
    MethodTypeInfo {
        descriptor_index: u16,
    },
    InvokeDynamicInfo {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
}

impl CpInfo {
    pub fn as_class_info(&self) -> Option<u16> {
        if let CpInfo::ClassInfo { name_index } = *self {
            Some(name_index)
        } else {
            None
        }
    }

    pub fn as_field_ref_info(&self) -> Option<(u16, u16)> {
        if let CpInfo::FieldRefInfo {
            class_index,
            name_and_type_index,
        } = *self
        {
            Some((class_index, name_and_type_index))
        } else {
            None
        }
    }

    pub fn as_method_ref_info(&self) -> Option<(u16, u16)> {
        if let CpInfo::MethodRefInfo {
            class_index,
            name_and_type_index,
        } = *self
        {
            Some((class_index, name_and_type_index))
        } else {
            None
        }
    }

    pub fn as_interface_ref_info(&self) -> Option<(u16, u16)> {
        if let CpInfo::InterfaceMethodRefInfo {
            class_index,
            name_and_type_index,
        } = *self
        {
            Some((class_index, name_and_type_index))
        } else {
            None
        }
    }

    pub fn as_string_info(&self) -> Option<u16> {
        if let CpInfo::StringInfo { string_index } = *self {
            Some(string_index)
        } else {
            None
        }
    }

    pub fn as_integer_info(&self) -> Option<i32> {
        if let CpInfo::IntegerInfo(value) = *self {
            Some(value)
        } else {
            None
        }
    }

    pub fn as_float_info(&self) -> Option<f32> {
        if let CpInfo::FloatInfo(value) = *self {
            Some(value)
        } else {
            None
        }
    }

    pub fn as_long_info(&self) -> Option<i64> {
        if let CpInfo::LongInfo(value) = *self {
            Some(value)
        } else {
            None
        }
    }

    pub fn as_double_info(&self) -> Option<f64> {
        if let CpInfo::DoubleInfo(value) = *self {
            Some(value)
        } else {
            None
        }
    }

    pub fn as_name_and_type_info(&self) -> Option<(u16, u16)> {
        if let CpInfo::NameAndTypeInfo {
            name_index,
            descriptor_index,
        } = *self
        {
            Some((name_index, descriptor_index))
        } else {
            None
        }
    }

    pub fn as_utf8_info(&self) -> Option<&str> {
        if let CpInfo::UTF8INFO(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn as_mehod_handle_info(&self) -> Option<(ReferenceKind, u16)> {
        if let CpInfo::MethodHandleInfo {
            reference_kind,
            reference_index,
        } = *self
        {
            Some((reference_kind, reference_index))
        } else {
            None
        }
    }

    pub fn as_method_type_info(&self) -> Option<u16> {
        if let CpInfo::MethodTypeInfo { descriptor_index } = *self {
            Some(descriptor_index)
        } else {
            None
        }
    }

    pub fn as_invoke_dynamic_info(&self) -> Option<(u16, u16)> {
        if let CpInfo::InvokeDynamicInfo {
            bootstrap_method_attr_index,
            name_and_type_index,
        } = *self
        {
            Some((bootstrap_method_attr_index, name_and_type_index))
        } else {
            None
        }
    }
}
