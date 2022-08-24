use super::Field;

pub struct TextField {
    name: String,
    index: bool,
    unique: bool,
}

impl TextField {
    pub fn new(name: String) -> TextField {
        TextField {
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

impl Field for TextField {
    fn model_field_code(&self) -> String {
        let index = if self.index { "index=True, " } else { "" };
        let unique = if self.unique { "unique=True, " } else { "" };
        format!("{} = models.TextField({}{})", self.name, index, unique)
    }
    fn serializer_field_code(&self) -> String {
        format!("{} = serializers.CharField()", self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_field() {
        let field = TextField {
            name: String::from("name"),
            index: true,
            unique: false,
        };
        assert_eq!(
            field.model_field_code(),
            String::from("name = models.TextField(index=True, )")
        );
    }
}
