use super::Field;
use crate::utils::upper_camel_case;

pub struct ForeignKey {
    name: String,
    reference: String,
}

impl ForeignKey {
    pub fn new(name: String, reference: String) -> ForeignKey {
        ForeignKey { name, reference }
    }
}

impl Field for ForeignKey {
    fn model_field_code(&self) -> String {
        let other_model_name = upper_camel_case(&self.reference);
        format!(
            "{} = models.ForeignKey(\"{}\", on_delete=models.CASCADE, )",
            self.name, other_model_name
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foreign_key() {
        let field = ForeignKey {
            name: String::from("owner"),
            reference: String::from("user"),
        };
        assert_eq!(
            field.model_field_code(),
            String::from("owner = models.ForeignKey(\"User\", on_delete=models.CASCADE, )")
        );
    }
}
