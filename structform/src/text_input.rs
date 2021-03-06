#[macro_export]
macro_rules! impl_text_input_with_stringops {
    ($text_input: ident, $type_name: literal, $type: ty) => {
        impl_text_input_with_stringops!(
            $text_input,
            |_e| structform::ParseError::InvalidFormat {
                required_type: $type_name.to_string()
            },
            $type
        );
    };
    ($text_input: ident, $type: ty) => {
        impl_text_input_with_stringops!(
            $text_input,
            |e| structform::ParseError::FromStrError(e.to_string()),
            $type
        );
    };
    ($text_input: ident, $handle_error: expr, $type: ty) => {
        impl structform::ParseAndFormat<$type> for $text_input<$type> {
            fn parse(value: &str) -> Result<$type, structform::ParseError> {
                let trimmed = value.trim();
                if trimmed.is_empty() {
                    Err(structform::ParseError::Required)
                } else {
                    trimmed.parse::<$type>().map_err($handle_error)
                }
            }

            fn format(value: &$type) -> String {
                value.to_string()
            }
        }

        impl structform::ParseAndFormat<Option<$type>> for $text_input<Option<$type>> {
            fn parse(value: &str) -> Result<Option<$type>, structform::ParseError> {
                let trimmed = value.trim();
                if trimmed.is_empty() {
                    Ok(None)
                } else {
                    trimmed
                        .parse::<$type>()
                        .map(Option::Some)
                        .map_err(|e| structform::ParseError::FromStrError(e.to_string()))
                }
            }

            fn format(value: &Option<$type>) -> String {
                match value {
                    None => "".to_string(),
                    Some(inner) => inner.to_string(),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_vec_text_input_with_stringops {
    ($text_input: ident, $type_name: literal, $type: ty) => {
        impl_vec_text_input_with_stringops!(
            $text_input,
            |_e| structform::ParseError::InvalidFormat {
                required_type: $type_name.to_string()
            },
            $type
        );
    };
    ($text_input: ident, $type: ty) => {
        impl_vec_text_input_with_stringops!(
            $text_input,
            |e| structform::ParseError::FromStrError(e.to_string()),
            $type
        );
    };
    ($text_input: ident, $handle_error: expr, $type: ty) => {
        impl structform::ParseAndFormat<Vec<$type>> for $text_input<Vec<$type>> {
            fn parse(value: &str) -> Result<Vec<$type>, structform::ParseError> {
                value
                    .trim()
                    .split(',')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .map(|trimmed| trimmed.parse::<$type>().map_err($handle_error))
                    .collect()
            }

            fn format(value: &Vec<$type>) -> String {
                value
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            }
        }
    };
}
