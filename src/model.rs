use crate::utils::{convert_name, parse_field};

/// Generate Django model code.
///
/// # Example
///
/// ```rust
/// use django_code_generator::model::generate_model_code;
///
/// let name = "user";
/// let fields = vec![String::from("name:string"), String::from("is_active:bool")];
/// let code = generate_model_code(name, &fields);
/// assert_eq!(code, Ok(String::from("
/// class User(models.Model):
///     \"\"\"User model
///
///     auto generated code.
///     \"\"\"
///     id = models.AutoField(primary_key=True)
///     created_at = models.DateTimeField(editable=False, auto_add=True)
///     updated_at = models.DateTimeField(editable=False, auto_add_now=True)
///     name = models.CharField(max_length=200)
///     is_active = models.BooleanField()
///
/// ")))
/// ```
pub fn generate_model_code(name: &str, fields: &Vec<String>) -> Result<String, String> {
    let mut content = String::new();
    // class name
    content.push_str("\nclass ");
    content.push_str(&convert_name(name));
    content.push_str("(models.Model):\n");
    content.push_str("    \"\"\"");
    content.push_str(&convert_name(name));
    content.push_str(" model\n\n");
    content.push_str("    auto generated code.\n");
    content.push_str("    \"\"\"\n");
    // fields
    content.push_str("    id = models.AutoField(primary_key=True)\n");
    content.push_str("    created_at = models.DateTimeField(editable=False, auto_add=True)\n");
    content.push_str("    updated_at = models.DateTimeField(editable=False, auto_add_now=True)\n");
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

fn generate_field_code(fieldtype: &str) -> &str {
    match fieldtype {
        "int" => "models.IntegerField()",
        "bool" => "models.BooleanField()",
        "string" => "models.CharField(max_length=200)",
        "json" => "models.JSONField()",
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
                want: "models.CharField(max_length=200)",
            },
            TestCase {
                fieldtype: "int",
                want: "models.IntegerField()",
            },
            TestCase {
                fieldtype: "json",
                want: "models.JSONField()",
            },
            TestCase {
                fieldtype: "bool",
                want: "models.BooleanField()",
            },
        ];

        for tt in cases {
            assert_eq!(tt.want, generate_field_code(tt.fieldtype))
        }
    }
}
