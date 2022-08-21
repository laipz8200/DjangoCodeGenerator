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
    ///
    /// ")));
    /// ```
    pub fn generate_serializer_code(
        &self,
        name: &str,
        fields: &Vec<String>,
    ) -> Result<String, String> {
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
                if let Some(code) = self.code_map.get(fieldtype) {
                    content.push_str("    ");
                    content.push_str(fieldname);
                    content.push_str(" = ");
                    content.push_str(code);
                    content.push_str("\n");
                }
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
    ///         fields = ('name', 'is_active', 'id', 'created_at', 'updated_at',)
    ///
    /// ")));
    /// ```
    pub fn generate_model_serializer_code(
        &self,
        name: &str,
        fields: &Vec<String>,
    ) -> Result<String, String> {
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
}
