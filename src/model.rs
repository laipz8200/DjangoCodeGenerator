use crate::fields;
use crate::utils::upper_camel_case;

pub struct DjangoModel {
    name: String,
    fields: Vec<Box<dyn fields::Field>>,
}

impl DjangoModel {
    pub fn new(name: String, fields: Vec<String>) -> DjangoModel {
        let mut typed_fields: Vec<Box<dyn fields::Field>> = vec![];
        for field in fields {
            let v: Vec<&str> = field.split(':').collect();
            let field = fields::match_field(v);
            typed_fields.push(field);
        }
        DjangoModel {
            name,
            fields: typed_fields,
        }
    }
    pub fn code(&self) -> String {
        let name = upper_camel_case(&self.name);
        let mut code = format!("class {}(models.Model):\n", name).to_owned();
        code.push_str("    id = models.AutoField(primary_key=True, )\n");
        code.push_str("    created_at = models.DateTimeField(editable=False, auto_add=True, )\n");
        code.push_str(
            "    updated_at = models.DateTimeField(editable=False, auto_add_now=True, )\n",
        );
        for field in self.fields.iter() {
            code.push_str("    ");
            code.push_str(&field.model_field_code());
            code.push_str("\n")
        }
        code
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_django_model() {
        let fields = vec![
            String::from("name:string:index"),
            String::from("age:int"),
            String::from("leader:references:user"),
            String::from("card_id:string:unique"),
            String::from("can_swim:bool:false"),
        ];
        let model = DjangoModel::new(String::from("person"), fields);
        let want = String::from(
            "class Person(models.Model):
    id = models.AutoField(primary_key=True, )
    created_at = models.DateTimeField(editable=False, auto_add=True, )
    updated_at = models.DateTimeField(editable=False, auto_add_now=True, )
    name = models.CharField(max_length=200, index=True, )
    age = models.IntegerField()
    leader = models.ForeignKey(\"User\", on_delete=models.CASCADE, )
    card_id = models.CharField(max_length=200, unique=True, )
    can_swim = models.BooleanField(default=False, )
",
        );
        assert_eq!(want, model.code());
    }
}
