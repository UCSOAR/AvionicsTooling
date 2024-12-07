use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct SvgAttributes(HashMap<String, String>);

impl SvgAttributes {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn set_attribute(&mut self, attr_name: &str, attr_value: &str) {
        self.0.insert(attr_name.to_string(), attr_value.to_string());
    }

    pub fn get_attribute(&self, attr_name: &str) -> Option<&String> {
        self.0.get(attr_name.to_string().as_str())
    }

    pub fn serialize(&self) -> String {
        let serialized_attributes = self
            .0
            .iter()
            .map(|(attr_name, attr_value)| format!(" {}='{}'", attr_name, attr_value))
            .collect::<String>();

        format!("<svg{}>", serialized_attributes)
    }
}

mod test {
    use super::SvgAttributes;

    #[test]
    fn test_serialize_no_attributes() {
        let attributes = SvgAttributes::new();
        let expected = r#"<svg>"#;

        assert_eq!(expected, attributes.serialize().as_str())
    }

    #[test]
    fn test_serialize_with_attributes() {
        let mut attributes = SvgAttributes::new();
        attributes.set_attribute("class", "SOAR");
        attributes.set_attribute("name", "fan");

        let serialized = attributes.serialize();

        let expected_1 = r#"<svg class='SOAR' name='fan'>"#;
        let expected_2 = r#"<svg name='fan' class='SOAR'>"#;

        assert!(serialized.as_str() == expected_1 || serialized.as_str() == expected_2);
    }
}
