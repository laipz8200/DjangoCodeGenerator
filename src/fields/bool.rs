use super::Field;

pub struct BooleanField {
    name: String,
    default: Option<bool>,
}

impl BooleanField {
    pub fn new(name: String) -> BooleanField {
        BooleanField {
            name,
            default: None,
        }
    }
    pub fn set_default(&mut self, default: bool) {
        self.default = Some(default);
    }
}

impl Field for BooleanField {
    fn model_field_code(&self) -> String {
        let mut params = String::new();
        if let Some(default) = self.default {
            if default {
                params.push_str("default=True, ");
            } else {
                params.push_str("default=False, ");
            }
        }
        format!("{} = models.BooleanField({})", self.name, params)
    }
    fn serializer_field_code(&self) -> String {
        format!("{} = serializers.BooleanField()", self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_field() {
        let mut field = BooleanField::new(String::from("name"));
        assert_eq!(
            field.model_field_code(),
            String::from("name = models.BooleanField()")
        );
        field.set_default(true);
        assert_eq!(
            field.model_field_code(),
            String::from("name = models.BooleanField(default=True, )")
        );
        field.set_default(false);
        assert_eq!(
            field.model_field_code(),
            String::from("name = models.BooleanField(default=False, )")
        );
    }
}
