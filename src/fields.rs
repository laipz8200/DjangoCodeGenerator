mod bool;
mod char;
mod foreign;
mod int;

pub use self::bool::BooleanField;
pub use self::char::CharField;
pub use self::foreign::ForeignKey;
pub use self::int::IntegerField;

pub trait Field {
    fn model_field_code(&self) -> String;
    fn serializer_field_code(&self) -> String;
}

pub fn match_field(v: Vec<&str>) -> Box<dyn Field> {
    let name = String::from(v[0]);
    match v[1] {
        "int" => {
            let mut field = IntegerField::new(name);
            if v.len() == 3 {
                if v[2] == "index" {
                    field.set_index();
                } else {
                    field.set_unique();
                }
            }
            Box::new(field)
        }
        "string" => {
            let mut field = CharField::new(name);
            if v.len() == 3 {
                if v[2] == "index" {
                    field.set_index();
                } else {
                    field.set_unique();
                }
            }
            Box::new(field)
        }
        "references" => {
            let reference = String::from(v[2]);
            let field = ForeignKey::new(name, reference);
            Box::new(field)
        }
        "bool" => {
            let mut field = BooleanField::new(name);
            if v.len() == 3 {
                if v[2] == "true" {
                    field.set_default(true);
                } else if v[2] == "false" {
                    field.set_default(false);
                }
            }
            Box::new(field)
        }
        t => panic!("unsupported field type: {}", t),
    }
}
