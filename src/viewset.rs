/// DRF ViewSet Generator.
pub struct Generator;
use crate::utils::convert_name;

impl Generator {
    /// New DRF ViewSet Generator.
    pub fn new() -> Generator {
        Generator
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
    ///         \"\"\"list user
    ///         \"\"\"
    ///         return super().list(request, *args, **kwargs)
    ///
    ///     def retrieve(self, request, *args, **kwargs):
    ///         \"\"\"retrieve user
    ///         \"\"\"
    ///         return super().retrieve(request, *args, **kwargs)
    ///
    ///     def update(self, request, *args, **kwargs):
    ///         \"\"\"update user
    ///         \"\"\"
    ///         return super().update(request, *args, **kwargs)
    ///
    ///     def destroy(self, request, *args, **kwargs):
    ///         \"\"\"destroy user
    ///         \"\"\"
    ///         return super().destroy(request, *args, **kwargs)
    ///
    /// ")));
    /// ```
    pub fn generate_model_viewset_code(&self, name: &str) -> Result<String, String> {
        let mut content = String::new();
        // class name
        content.push_str(
            format!(
                "\nclass {}ModelViewSet(viewsets.ModelViewSet):\n",
                convert_name(name)?
            )
            .as_str(),
        );
        content.push_str(format!("    \"\"\"{} model viewset\n\n", convert_name(name)?).as_str());
        content.push_str("    auto generated code.\n");
        content.push_str("    \"\"\"\n");
        // class variables
        content
            .push_str(format!("    queryset = {}.objects.all()\n", convert_name(name)?).as_str());
        content.push_str(
            format!(
                "    serializer_class = {}ModelSerializer\n\n",
                convert_name(name)?
            )
            .as_str(),
        );
        // methods
        for method in vec!["list", "retrieve", "update", "destroy"] {
            content.push_str(
                format!("    def {}(self, request, *args, **kwargs):\n", method).as_str(),
            );
            content.push_str(format!("        \"\"\"{} {}\n", method, name).as_str());
            content.push_str("        \"\"\"\n");
            content.push_str(
                format!(
                    "        return super().{}(request, *args, **kwargs)\n\n",
                    method
                )
                .as_str(),
            );
        }
        Ok(content)
    }
}
