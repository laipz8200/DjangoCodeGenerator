use crate::utils::upper_camel_case;

pub struct DRFModelSerializer {
    name: String,
}

impl DRFModelSerializer {
    pub fn new(name: String) -> DRFModelSerializer {
        DRFModelSerializer { name }
    }
    pub fn code(&self) -> String {
        let name = upper_camel_case(&self.name);
        format!(
            "class {}ModelSerializer(serializers.ModelSerializer):
    class Meta:
        model = {}
        fields = __all__
",
            name, name
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drf_model_serializer() {
        let viewset = DRFModelSerializer::new(String::from("user"));
        let want = String::from(
            "class UserModelSerializer(serializers.ModelSerializer):
    class Meta:
        model = User
        fields = __all__
",
        );
        assert_eq!(want, viewset.code());
    }
}
