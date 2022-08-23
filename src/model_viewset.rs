use crate::utils::upper_camel_case;

pub struct DRFModelViewSet {
    name: String,
}

impl DRFModelViewSet {
    pub fn new(name: String) -> DRFModelViewSet {
        DRFModelViewSet { name }
    }
    pub fn code(&self) -> String {
        let name = upper_camel_case(&self.name);
        let queryset = format!("{}.objects.all()", name);
        let serializer_class = format!("{}ModelSerializer", name);
        format!(
            "class {}ModelViewSet(viewsets.ModelViewSet):
    queryset = {}
    serializer_class = {}
",
            name, queryset, serializer_class
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drf_model_viewset() {
        let viewset = DRFModelViewSet::new(String::from("user"));
        let want = String::from(
            "class UserModelViewSet(viewsets.ModelViewSet):
    queryset = User.objects.all()
    serializer_class = UserModelSerializer
",
        );
        assert_eq!(want, viewset.code());
    }
}
