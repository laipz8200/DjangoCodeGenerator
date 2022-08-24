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
            let field = fields::match_field(v);
            typed_fields.push(field);
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
