use super::{ClassFile, CpInfo, ReferenceKind};

#[derive(Clone, Debug)]
pub enum RuntimeCPEntry {
    Class {
        name: String,
    },
    MethodRefInfo {
        class: String,
        name: String,
        descriptor: String,
    },
    FieldRefInfo {
        class: String,
        name: String,
        descriptor: String,
    },
    InterfaceRefInfo {
        class: String,
        name: String,
        descriptor: String,
    },
    StringInfo(String),
    IntegerInfo(i32),
    FloatInfo(f32),
    LongInfo(i64),
    DoubleInfo(f64),
    NameAndTypeInfo {
        name: String,
        descriptor: String,
    },
    MethodHandleInfo {
        reference_kind: ReferenceKind,
        reference_index: u16,
    },
    MethodTypeInfo {
        descriptor: String,
    },
    InvokeDynamicInfo {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    Resolved,
}

pub fn remove_cp_offset(index: usize) -> usize {
    index - 1
}

impl RuntimeCPEntry {
    pub fn as_class(&self) -> Option<&str> {
        if let RuntimeCPEntry::Class { name } = self {
            Some(name.as_str())
        } else {
            None
        }
    }

    pub fn as_field_ref(&self) -> Option<(&str, &str, &str)> {
        if let RuntimeCPEntry::FieldRefInfo {
            name,
            class,
            descriptor,
        } = self
        {
            Some((name.as_str(), class.as_str(), descriptor.as_str()))
        } else {
            None
        }
    }

    pub fn as_method_ref(&self) -> Option<(&str, &str, &str)> {
        if let RuntimeCPEntry::MethodRefInfo {
            class,
            name,
            descriptor,
        } = self
        {
            Some((class.as_str(), name.as_str(), descriptor.as_str()))
        } else {
            None
        }
    }
}

fn decode_class_info(entry: &CpInfo, class_file: &ClassFile) -> String {
    let name_index =
        entry.as_class_info().expect("Cp_Info must be a class info");
    let name_entry = class_file
        .get_java_cp_entry(Into::<usize>::into(name_index))
        .unwrap();
    name_entry
        .as_utf8_info()
        .expect("CP_Info must be an UTF8_Info")
        .to_string()
}

fn decode_name_and_type_info(
    entry: &CpInfo,
    class_file: &ClassFile,
) -> (String, String) {
    let (name_index, descriptor_index) = entry
        .as_name_and_type_info()
        .expect("CP_Info must be an name_and_type_Info");
    let name = class_file
        .get_java_cp_entry(Into::<usize>::into(name_index))
        .unwrap()
        .as_utf8_info()
        .expect("CP_Info must be an UTF8_Info");
    let descriptor = class_file
        .get_java_cp_entry(Into::<usize>::into(descriptor_index))
        .unwrap()
        .as_utf8_info()
        .expect("CP_Info must be an UTF8_Info");
    (name.to_string(), descriptor.to_string())
}

fn decode_field_ref(
    entry: &CpInfo,
    class_file: &ClassFile,
) -> (String, String, String) {
    let (class_index, name_and_type_index) = entry
        .as_field_ref_info()
        .expect("Cp_info must be an field_ref_info");
    let class_name = decode_class_info(
        class_file
            .get_java_cp_entry(Into::<usize>::into(class_index))
            .unwrap(),
        class_file,
    );
    let (name, descriptor) = decode_name_and_type_info(
        class_file
            .get_java_cp_entry(Into::<usize>::into(name_and_type_index))
            .unwrap(),
        class_file,
    );
    (class_name, name, descriptor)
}

fn decode_method_ref(
    entry: &CpInfo,
    class_file: &ClassFile,
) -> (String, String, String) {
    let (class_index, name_and_type_index) = entry
        .as_method_ref_info()
        .expect("Cp_info must be an method_ref_info");
    let class_name = decode_class_info(
        class_file
            .get_java_cp_entry(Into::<usize>::into(class_index))
            .unwrap(),
        class_file,
    );
    let (name, descriptor) = decode_name_and_type_info(
        class_file
            .get_java_cp_entry(Into::<usize>::into(name_and_type_index))
            .unwrap(),
        class_file,
    );
    (class_name, name, descriptor)
}

fn decode_interface_ref(
    entry: &CpInfo,
    class_file: &ClassFile,
) -> (String, String, String) {
    let (class_index, name_and_type_index) = entry
        .as_interface_ref_info()
        .expect("Cp_info must be an interface_ref_info");
    let class_name = decode_class_info(
        class_file
            .get_java_cp_entry(Into::<usize>::into(class_index))
            .unwrap(),
        class_file,
    );
    let (name, descriptor) = decode_name_and_type_info(
        class_file
            .get_java_cp_entry(Into::<usize>::into(name_and_type_index))
            .unwrap(),
        class_file,
    );
    (class_name, name, descriptor)
}

fn decode_string_info(entry: &CpInfo, class_file: &ClassFile) -> String {
    let name_index = entry
        .as_string_info()
        .expect("Cp_Info must be a string info");
    let name_entry = class_file
        .get_java_cp_entry(Into::<usize>::into(name_index))
        .unwrap();
    name_entry
        .as_utf8_info()
        .expect("CP_Info must be an UTF8_Info")
        .to_string()
}

fn decode_method_type_info(entry: &CpInfo, class_file: &ClassFile) -> String {
    let index = entry
        .as_method_type_info()
        .expect("Cp_Info must be a methodType info");
    let name_entry = class_file
        .get_java_cp_entry(Into::<usize>::into(index))
        .unwrap();
    name_entry
        .as_utf8_info()
        .expect("CP_Info must be an UTF8_Info")
        .to_string()
}

fn decode_entry(entry: &CpInfo, class_file: &ClassFile) -> RuntimeCPEntry {
    match entry {
        CpInfo::ClassInfo { name_index: _ } => RuntimeCPEntry::Class {
            name: (decode_class_info(entry, class_file)),
        },
        CpInfo::FieldRefInfo {
            class_index: _,
            name_and_type_index: _,
        } => {
            let (class_name, name, descriptor) =
                decode_field_ref(entry, class_file);
            RuntimeCPEntry::FieldRefInfo {
                class: (class_name),
                name: (name),
                descriptor: (descriptor),
            }
        },
        CpInfo::MethodRefInfo {
            class_index: _,
            name_and_type_index: _,
        } => {
            let (class_name, name, descriptor) =
                decode_method_ref(entry, class_file);
            RuntimeCPEntry::MethodRefInfo {
                class: (class_name),
                name: (name),
                descriptor: (descriptor),
            }
        },
        CpInfo::InterfaceMethodRefInfo {
            class_index: _,
            name_and_type_index: _,
        } => {
            let (class_name, name, descriptor) =
                decode_interface_ref(entry, class_file);
            RuntimeCPEntry::InterfaceRefInfo {
                class: (class_name),
                name: (name),
                descriptor: (descriptor),
            }
        },
        CpInfo::StringInfo { string_index: _ } => {
            RuntimeCPEntry::StringInfo(decode_string_info(entry, class_file))
        },
        CpInfo::IntegerInfo(_) => RuntimeCPEntry::IntegerInfo(
            entry
                .as_integer_info()
                .expect("Cp_Info must be a integer info"),
        ),
        CpInfo::FloatInfo(_) => RuntimeCPEntry::FloatInfo(
            entry.as_float_info().expect("Cp_Info must be a float info"),
        ),
        CpInfo::LongInfo(_) => RuntimeCPEntry::LongInfo(
            entry.as_long_info().expect("Cp_Info must be a long info"),
        ),
        CpInfo::DoubleInfo(_) => RuntimeCPEntry::DoubleInfo(
            entry
                .as_double_info()
                .expect("Cp_Info must be a double info"),
        ),
        CpInfo::NameAndTypeInfo {
            name_index: _,
            descriptor_index: _,
        } => RuntimeCPEntry::Resolved,
        CpInfo::UTF8INFO(_) => RuntimeCPEntry::Resolved,
        CpInfo::MethodHandleInfo {
            reference_kind: _,
            reference_index: _,
        } => todo!(),
        CpInfo::MethodTypeInfo {
            descriptor_index: _,
        } => RuntimeCPEntry::MethodTypeInfo {
            descriptor: (decode_method_type_info(entry, class_file)),
        },
        CpInfo::InvokeDynamicInfo {
            bootstrap_method_attr_index: _,
            name_and_type_index: _,
        } => todo!(),
    }
}

pub fn decode_constant_pool(class_file: &ClassFile) -> Vec<RuntimeCPEntry> {
    class_file
        .constant_pool
        .iter()
        .map(|e| decode_entry(e, class_file))
        .collect()
}
