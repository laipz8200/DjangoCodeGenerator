use crate::utils::parse_field;

/// Generate DRF Serializer code.
///
/// # Example
///
/// ```rust
/// use django_code_generator::serializer::generate_serializer_code;
///
/// let name = "User";
/// let fields = vec![String::from("name:string"), String::from("is_active:bool")];
/// let code = generate_serializer_code(name, &fields);
/// assert_eq!(code, String::from("
/// class UserSerializer(serializers.Serializer):
///     \"\"\"User serializer
///
///     auto generated code.
///     \"\"\"
///     name = serializers.CharField()
///     is_active = serializers.BooleanField()
///
/// "))
/// ```
pub fn generate_serializer_code(name: &str, fields: &Vec<String>) -> String {
    let mut content = String::new();
    // class name
    content.push_str("\nclass ");
    content.push_str(name);
    content.push_str("Serializer(serializers.Serializer):\n");
    content.push_str("    \"\"\"");
    content.push_str(name);
    content.push_str(" serializer\n\n");
    content.push_str("    auto generated code.\n");
    content.push_str("    \"\"\"\n");
    // fields
    for value in fields.iter() {
        if let Ok((fieldname, fieldtype)) = parse_field(value) {
            content.push_str("    ");
            content.push_str(fieldname);
            content.push_str(" = ");
            content.push_str(generate_field_code(fieldtype));
            content.push_str("\n");
        };
    }
    content.push_str("\n");
    content
}

fn generate_field_code(fieldtype: &str) -> &str {
    match fieldtype {
        "int" => "serializers.IntegerField()",
        "bool" => "serializers.BooleanField()",
        "string" => "serializers.CharField()",
        "json" => "serializers.JSONField()",
        fieldtype => panic!("not support field type: {}", fieldtype),
    }
}

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_field_code() {
        struct TestCase<'a> {
            fieldtype: &'a str,
            want: &'a str,
        }

        let cases = vec![
            TestCase {
                fieldtype: "string",
                want: "serializers.CharField",
            },
            TestCase {
                fieldtype: "int",
                want: "serializers.IntegerField",
            },
            TestCase {
                fieldtype: "json",
                want: "serializers.JSONField",
            },
            TestCase {
                fieldtype: "bool",
                want: "serializers.BooleanField",
            },
        ];

        for tt in cases {
            assert_eq!(tt.want, generate_field_code(tt.fieldtype))
        }
    }
}
