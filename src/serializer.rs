use crate::utils::{convert_name, parse_field};
use std::collections::HashMap;

/// DRF Serializer Generator.
pub struct Generator<'a> {
    code_map: HashMap<&'a str, &'a str>,
}

impl<'a> Generator<'a> {
    /// New DRF Serializer Generator.
    pub fn new() -> Generator<'a> {
        let mut code_map: HashMap<&'a str, &'a str> = HashMap::new();
        code_map.insert("int", "serializers.IntegerField()");
        code_map.insert("bool", "serializers.BooleanField()");
        code_map.insert("string", "serializers.CharField()");
        code_map.insert("json", "serializers.JSONField()");
        Generator { code_map }
    }
    /// Generate DRF Serializer code.
    ///
    /// # Example
    ///
    /// ```rust
    /// use django_code_generator::serializer;
    ///
    /// let name = "user";
    /// let fields = vec![String::from("name:string"), String::from("is_active:bool")];
    /// let generator = serializer::Generator::new();
    /// let code = generator.generate_serializer_code(name, &fields);
    /// assert_eq!(code, Ok(String::from("
    /// class UserSerializer(serializers.Serializer):
    ///     \"\"\"User serializer
    ///
    ///     auto generated code.
    ///     \"\"\"
    ///     name = serializers.CharField()
    ///     is_active = serializers.BooleanField()
    /// ")));
    /// ```
    pub fn generate_serializer_code(
        &self,
        name: &str,
        fields: &Vec<String>,
    ) -> Result<String, String> {
        let name = convert_name(name)?;
        let mut content = format!(
            "
class {name}Serializer(serializers.Serializer):
    \"\"\"{name} serializer

    auto generated code.
    \"\"\"
"
        )
        .to_owned();
        for field in fields.iter() {
            if let Ok((fieldname, fieldtype)) = parse_field(field) {
                if let Some(code) = self.code_map.get(fieldtype) {
                    content += format!("    {fieldname} = {code}\n").as_str();
                }
            };
        }
        Ok(String::from(content))
    }
    /// Generate DRF ModelSerializer code.
    ///
    /// # Example
    ///
    /// ```rust
    /// use django_code_generator::serializer;
    /// let generator = serializer::Generator::new();
    ///
    /// let name = "normal_user";
    /// let fields = vec![String::from("name:string"), String::from("is_active:bool")];
    /// let generator = serializer::Generator::new();
    /// let code = generator.generate_model_serializer_code(name, &fields);
    /// assert_eq!(code, Ok(String::from("
    /// class NormalUserModelSerializer(serializers.ModelSerializer):
    ///     \"\"\"NormalUser model serializer
    ///
    ///     auto generated code.
    ///     \"\"\"
    ///     class Meta:
    ///         model = NormalUser
    ///         fields = ('name', 'is_active', 'id', 'created_at', 'updated_at')
    /// ")));
    /// ```
    pub fn generate_model_serializer_code(
        &self,
        name: &str,
        fields: &Vec<String>,
    ) -> Result<String, String> {
        let name = convert_name(name)?;
        let mut v = vec![];
        for field in fields {
            let (fieldname, _) = parse_field(field)?;
            v.push(format!("'{fieldname}'"));
        }
        v.push(String::from("'id'"));
        v.push(String::from("'created_at'"));
        v.push(String::from("'updated_at'"));
        let fields = v.join(", ");
        let content = format!(
            "
class {name}ModelSerializer(serializers.ModelSerializer):
    \"\"\"{name} model serializer

    auto generated code.
    \"\"\"
    class Meta:
        model = {name}
        fields = ({fields})
"
        );
        Ok(content)
    }
}
