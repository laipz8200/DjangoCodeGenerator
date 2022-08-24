use crate::fields;
use crate::utils::upper_camel_case;

pub struct DRFSerializer {
    name: String,
    fields: Vec<Box<dyn fields::Field>>,
}

impl DRFSerializer {
    pub fn new(name: String, fields: Vec<String>) -> DRFSerializer {
        let mut typed_fields: Vec<Box<dyn fields::Field>> = vec![];
        for field in fields {
            let v: Vec<&str> = field.split(':').collect();
            let name = String::from(v[0]);
            match v[1] {
                "int" => {
                    let mut field = fields::IntegerField::new(name);
                    if v.len() == 3 {
                        if v[2] == "index" {
                            field.set_index();
                        } else {
                            field.set_unique();
                        }
                    }
                    typed_fields.push(Box::new(field));
                }
                "string" => {
                    let mut field = fields::CharField::new(name);
                    if v.len() == 3 {
                        if v[2] == "index" {
                            field.set_index();
                        } else {
                            field.set_unique();
                        }
                    }
                    typed_fields.push(Box::new(field));
                }
                "references" => {
                    let reference = String::from(v[2]);
                    let field = fields::ForeignKey::new(name, reference);
                    typed_fields.push(Box::new(field));
                }
                "bool" => {
                    let mut field = fields::BooleanField::new(name);
                    if v.len() == 3 {
                        if v[2] == "true" {
                            field.set_default(true);
                        } else if v[2] == "false" {
                            field.set_default(false);
                        }
                    }
                    typed_fields.push(Box::new(field));
                }
                t => panic!("unsupported field type: {}", t),
            }
        }
        DRFSerializer {
            name,
            fields: typed_fields,
        }
    }
    pub fn code(&self) -> String {
        let name = upper_camel_case(&self.name);
        let mut code = format!("class {}Serializer(serializers.Serializer):\n", name).to_owned();
        for field in self.fields.iter() {
            code.push_str("    ");
            code.push_str(&field.serializer_field_code());
            code.push_str("\n")
        }
        code
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drf_serializer() {
        let fields = vec![
            String::from("name:string:index"),
            String::from("age:int"),
            String::from("leader:references:user"),
            String::from("card_id:string:unique"),
            String::from("can_swim:bool:false"),
        ];
        let serializer = DRFSerializer::new(String::from("person"), fields);
        let want = String::from(
            "class PersonSerializer(serializers.Serializer):
    name = serializers.CharField()
    age = serializers.IntegerField()
    leader = serializers.IntegerField()
    card_id = serializers.CharField()
    can_swim = serializers.BooleanField()
",
        );
        assert_eq!(want, serializer.code());
    }
}
