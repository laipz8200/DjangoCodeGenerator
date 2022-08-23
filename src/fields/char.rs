use super::Field;

pub struct CharField {
    name: String,
    index: bool,
    unique: bool,
}

impl CharField {
    pub fn new(name: String) -> CharField {
        CharField {
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

impl Field for CharField {
    fn model_field_code(&self) -> String {
        let index = if self.index { "index=True, " } else { "" };
        let unique = if self.unique { "unique=True, " } else { "" };
        format!(
            "{} = models.CharField(max_length=200, {}{})",
            self.name, index, unique
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_field() {
        let field = CharField {
            name: String::from("name"),
            index: true,
            unique: false,
        };
        assert_eq!(
            field.model_field_code(),
            String::from("name = models.CharField(max_length=200, index=True, )")
        );
    }
}
