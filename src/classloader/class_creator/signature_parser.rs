use crate::class::{ArgumentKind, SimpleArgumentKind};

enum ArgumentStates {
    BeforeArguments,
    Arguments,
    ReturnType,
    ClassName,
    Array,
}

fn match_argument_char(
    char: char,
) -> (Option<SimpleArgumentKind>, ArgumentStates) {
    match char {
        'B' => (Some(SimpleArgumentKind::Byte), ArgumentStates::Arguments),
        'D' => (Some(SimpleArgumentKind::Double), ArgumentStates::Arguments),
        'F' => (Some(SimpleArgumentKind::Float), ArgumentStates::Arguments),
        'I' => (Some(SimpleArgumentKind::Int), ArgumentStates::Arguments),
        'C' => (Some(SimpleArgumentKind::Char), ArgumentStates::Arguments),
        'J' => (Some(SimpleArgumentKind::Long), ArgumentStates::Arguments),
        'S' => (Some(SimpleArgumentKind::Short), ArgumentStates::Arguments),
        'Z' => (Some(SimpleArgumentKind::Boolean), ArgumentStates::Arguments),
        'L' => (None, ArgumentStates::ClassName),
        '[' => (None, ArgumentStates::Array),
        ')' => (None, ArgumentStates::ReturnType),
        _ => panic!(
            "Unexpected Symbol for method parameter kind, found {}",
            char
        ),
    }
}

pub fn parse_method_arguments(
    descriptor: &str,
) -> (Vec<ArgumentKind>, Option<ArgumentKind>) {
    let mut parameters = Vec::default();
    let mut state = ArgumentStates::BeforeArguments;
    let mut array_dim_counter = 0;
    let mut current_class_name: String = "".to_string();
    let mut return_type = None;
    let mut return_state = false;
    for char in descriptor.chars() {
        match state {
            ArgumentStates::BeforeArguments => {
                if char != '(' {
                    panic!(
                        "Method Arguments need to start with '(', found '{}' ",
                        char
                    )
                }
                state = ArgumentStates::Arguments;
            },
            ArgumentStates::Arguments => {
                let result = match_argument_char(char);
                state = result.1;
                if let Some(argument) = result.0 {
                    parameters.push(ArgumentKind::Simple(argument))
                }
            },
            ArgumentStates::ReturnType => {
                match char {
                    'D' => {
                        return_type = Some(ArgumentKind::Simple(
                            SimpleArgumentKind::Double,
                        ))
                    },
                    'F' => {
                        return_type = Some(ArgumentKind::Simple(
                            SimpleArgumentKind::Float,
                        ))
                    },
                    'B' => {
                        return_type =
                            Some(ArgumentKind::Simple(SimpleArgumentKind::Byte))
                    },
                    'I' => {
                        return_type =
                            Some(ArgumentKind::Simple(SimpleArgumentKind::Int))
                    },
                    'C' => {
                        return_type =
                            Some(ArgumentKind::Simple(SimpleArgumentKind::Char))
                    },
                    'J' => {
                        return_type =
                            Some(ArgumentKind::Simple(SimpleArgumentKind::Long))
                    },
                    'S' => {
                        return_type = Some(ArgumentKind::Simple(
                            SimpleArgumentKind::Short,
                        ))
                    },
                    'Z' => {
                        return_type = Some(ArgumentKind::Simple(
                            SimpleArgumentKind::Boolean,
                        ))
                    },
                    'V' => return_type = None,
                    'L' => {
                        state = ArgumentStates::ClassName;
                        return_state = true;
                    },
                    '[' => {
                        state = ArgumentStates::Array;
                        return_state = true;
                    },
                    _ => panic!(
                        "Unexpected Symbol for method return kind, found {}",
                        char
                    ),
                };
            },
            ArgumentStates::ClassName => {
                if char == ';' {
                    if array_dim_counter > 0 {
                        if return_state {
                            return_type = Some(ArgumentKind::Array {
                                dimensions: (array_dim_counter),
                                kind: (SimpleArgumentKind::Class(
                                    current_class_name.to_string(),
                                )),
                            });
                        } else {
                            parameters.push(ArgumentKind::Array {
                                dimensions: (array_dim_counter),
                                kind: (SimpleArgumentKind::Class(
                                    current_class_name.to_string(),
                                )),
                            });
                        }

                        array_dim_counter = 0;
                    } else if return_state {
                        return_type = Some(ArgumentKind::Simple(
                            SimpleArgumentKind::Class(
                                current_class_name.to_string(),
                            ),
                        ));
                    } else {
                        parameters.push(ArgumentKind::Simple(
                            SimpleArgumentKind::Class(
                                current_class_name.to_string(),
                            ),
                        ));
                    }
                    current_class_name = "".to_string();
                } else {
                    current_class_name.push(char);
                }
            }, // if Dim > 0 return array
            ArgumentStates::Array => {
                array_dim_counter += 1;
                let result = match_argument_char(char);
                state = result.1;
                if let Some(argument) = result.0 {
                    if return_state {
                        return_type = Some(ArgumentKind::Array {
                            dimensions: (array_dim_counter),
                            kind: (argument),
                        });
                    } else {
                        parameters.push(ArgumentKind::Array {
                            dimensions: (array_dim_counter),
                            kind: (argument),
                        });
                    }
                    array_dim_counter = 0;
                }
            },
        }
    }
    (parameters, return_type)
}
