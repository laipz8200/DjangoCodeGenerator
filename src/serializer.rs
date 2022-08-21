use crate::utils::{convert_name, parse_field};

/// Generate DRF Serializer code.
///
/// # Example
///
/// ```rust
/// use django_code_generator::serializer::generate_serializer_code;
///
/// let name = "user";
/// let fields = vec![String::from("name:string"), String::from("is_active:bool")];
/// let code = generate_serializer_code(name, &fields);
/// assert_eq!(code, Ok(String::from("
/// class UserSerializer(serializers.Serializer):
///     \"\"\"User serializer
///
///     auto generated code.
///     \"\"\"
///     name = serializers.CharField()
///     is_active = serializers.BooleanField()
///
/// ")))
/// ```
pub fn generate_serializer_code(name: &str, fields: &Vec<String>) -> Result<String, String> {
    let mut content = String::new();
    // class name
    content.push_str("\nclass ");
    content.push_str(&convert_name(name)?);
    content.push_str("Serializer(serializers.Serializer):\n");
    content.push_str("    \"\"\"");
    content.push_str(&convert_name(name)?);
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
    Ok(content)
}

/// Generate DRF ModelSerializer code.
///
/// # Example
///
/// ```rust
/// use django_code_generator::serializer::generate_model_serializer_code;
///
/// let name = "normal-user";
/// let fields = vec![String::from("name:string"), String::from("is_active:bool")];
/// let code = generate_model_serializer_code(name, &fields);
/// assert_eq!(code, Ok(String::from("
/// class NormalUserModelSerializer(serializers.ModelSerializer):
///     \"\"\"NormalUser model serializer
///
///     auto generated code.
///     \"\"\"
///     class Meta:
///         model = NormalUser
///         fields = ('name', 'is_active', 'id', 'created_at', 'updated_at',)
///
/// ")))
/// ```
pub fn generate_model_serializer_code(name: &str, fields: &Vec<String>) -> Result<String, String> {
    let mut content = String::new();
    // class name
    content.push_str("\nclass ");
    content.push_str(&convert_name(name)?);
    content.push_str("ModelSerializer(serializers.ModelSerializer):\n");
    content.push_str("    \"\"\"");
    content.push_str(&convert_name(name)?);
    content.push_str(" model serializer\n\n");
    content.push_str("    auto generated code.\n");
    content.push_str("    \"\"\"\n");
    content.push_str("    class Meta:\n");
    content.push_str("        model = ");
    content.push_str(&convert_name(name)?);
    content.push_str("\n");
    content.push_str("        fields = (");
    // fields
    for value in fields.iter() {
        if let Ok((fieldname, _)) = parse_field(value) {
            content.push_str("'");
            content.push_str(fieldname);
            content.push_str("', ");
        };
    }
    content.push_str("'id', 'created_at', 'updated_at',)\n");
    content.push_str("\n");
    Ok(content)
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

#[cfg(test)]
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
                want: "serializers.CharField()",
            },
            TestCase {
                fieldtype: "int",
                want: "serializers.IntegerField()",
            },
            TestCase {
                fieldtype: "json",
                want: "serializers.JSONField()",
            },
            TestCase {
                fieldtype: "bool",
                want: "serializers.BooleanField()",
            },
        ];

        for tt in cases {
            assert_eq!(tt.want, generate_field_code(tt.fieldtype))
        }
    }
}
