use crate::utils::{convert_name, parse_field};
use std::collections::HashMap;

/// Django Model Generator.
pub struct Generator<'a> {
    code_map: HashMap<&'a str, &'a str>,
}

impl<'a> Generator<'a> {
    /// New Django model Parser.
    pub fn new() -> Generator<'a> {
        let mut code_map: HashMap<&str, &str> = HashMap::new();
        code_map.insert("int", "models.IntegerField()");
        code_map.insert("bool", "models.BooleanField()");
        code_map.insert("string", "models.CharField(max_length=200)");
        code_map.insert("json", "models.JSONField()");
        Generator { code_map }
    }
    /// Generate Django model code.
    ///
    /// # Example
    ///
    /// ```rust
    /// use django_code_generator::model;
    ///
    /// let name = "user";
    /// let fields = vec![String::from("name:string"), String::from("is_active:bool")];
    /// let generator = model::Generator::new();
    /// let code = generator.generate_model_code(name, &fields);
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
    /// ")));
    /// ```
    pub fn generate_model_code(&self, name: &str, fields: &Vec<String>) -> Result<String, String> {
        let name = convert_name(name)?;
        let mut content = format!(
            "
class {name}(models.Model):
    \"\"\"{name} model

    auto generated code.
    \"\"\"
    id = models.AutoField(primary_key=True)
    created_at = models.DateTimeField(editable=False, auto_add=True)
    updated_at = models.DateTimeField(editable=False, auto_add_now=True)
"
        )
        .to_owned();
        for field in fields.iter() {
            if let Ok((fieldname, fieldtype)) = parse_field(field) {
                if let Some(code) = self.code_map.get(&fieldtype) {
                    content += format!("    {fieldname} = {code}\n").as_str();
                }
            };
        }
        Ok(String::from(content))
    }
}
