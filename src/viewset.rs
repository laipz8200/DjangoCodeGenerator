/// DRF ViewSet Generator.
pub struct Generator<'a> {
    methods: Vec<&'a str>,
}
use crate::utils::convert_name;

impl<'a> Generator<'a> {
    /// New DRF ViewSet Generator.
    pub fn new() -> Generator<'a> {
        Generator {
            methods: vec!["list", "retrieve", "update", "destroy"],
        }
    }
    /// Generate DRF Model ViewSet Code.
    ///
    /// # Example
    ///
    /// ```rust
    /// use django_code_generator::viewset;
    ///
    /// let name = "user";
    /// let generator = viewset::Generator::new();
    /// let code = generator.generate_model_viewset_code(name);
    /// assert_eq!(code, Ok(String::from("
    /// class UserModelViewSet(viewsets.ModelViewSet):
    ///     \"\"\"User model viewset
    ///
    ///     auto generated code.
    ///     \"\"\"
    ///     queryset = User.objects.all()
    ///     serializer_class = UserModelSerializer
    ///
    ///     def list(self, request, *args, **kwargs):
    ///         \"\"\"User list
    ///         \"\"\"
    ///         return super().list(request, *args, **kwargs)
    ///
    ///     def retrieve(self, request, *args, **kwargs):
    ///         \"\"\"User retrieve
    ///         \"\"\"
    ///         return super().retrieve(request, *args, **kwargs)
    ///
    ///     def update(self, request, *args, **kwargs):
    ///         \"\"\"User update
    ///         \"\"\"
    ///         return super().update(request, *args, **kwargs)
    ///
    ///     def destroy(self, request, *args, **kwargs):
    ///         \"\"\"User destroy
    ///         \"\"\"
    ///         return super().destroy(request, *args, **kwargs)
    /// ")));
    /// ```
    pub fn generate_model_viewset_code(&self, name: &str) -> Result<String, String> {
        let name = convert_name(name)?;
        let mut content = format!(
            "
class {name}ModelViewSet(viewsets.ModelViewSet):
    \"\"\"{name} model viewset

    auto generated code.
    \"\"\"
    queryset = {name}.objects.all()
    serializer_class = {name}ModelSerializer
"
        );
        for method in self.methods.iter() {
            content += format!(
                "
    def {method}(self, request, *args, **kwargs):
        \"\"\"{name} {method}
        \"\"\"
        return super().{method}(request, *args, **kwargs)
"
            )
            .as_str();
        }
        Ok(content)
    }
}
