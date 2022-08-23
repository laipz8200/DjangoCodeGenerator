use super::Field;

pub struct IntegerField {
    name: String,
    index: bool,
    unique: bool,
}

impl IntegerField {
    pub fn new(name: String) -> IntegerField {
        IntegerField {
            name,
            index: false,
            unique: false,
        }
    }
    pub fn set_index(&mut self) {
        self.index = true;
    }
    pub fn set_unique(&mut self) {
        self.unique = true;
    }
}

impl Field for IntegerField {
    fn model_field_code(&self) -> String {
        let index = if self.index { "index=True, " } else { "" };
        let unique = if self.unique { "unique=True, " } else { "" };
        format!("{} = models.IntegerField({}{})", self.name, index, unique)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_field() {
        let field = IntegerField {
            name: String::from("age"),
            index: false,
            unique: true,
        };
        assert_eq!(
            field.model_field_code(),
            String::from("age = models.IntegerField(unique=True, )")
        );
    }
}
