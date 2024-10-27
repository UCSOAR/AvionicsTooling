use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct XmlAttributes(HashMap<String, String>);

impl XmlAttributes {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn set_attribute(&mut self, attr_name: String, attr_value: String) {
        self.0.insert(attr_name, attr_value);
    }

    pub fn serialize(&self) -> String {
        let mut curr = "<svg ".to_string();

        let serialized_attributes = self
            .0
            .iter()
            .map(|(attr_name, attr_value)| format!("{}='{}' ", attr_name, attr_value))
            .collect::<String>();

        curr.push_str(serialized_attributes.as_str());
        curr.pop();
        curr.push('>');

        curr
    }
}

mod test {
    use super::XmlAttributes;

    #[test]
    fn test_serialize_no_attributes() {
        let attributes = XmlAttributes::new();
        let expected = r#"<svg>"#;

        assert_eq!(expected, attributes.serialize().as_str())
    }

    #[test]
    fn test_serialize_with_attributes() {
        let mut attributes = XmlAttributes::new();
        attributes.set_attribute("class".to_string(), "SOAR".to_string());
        attributes.set_attribute("name".to_string(), "fan".to_string());

        let serialized = attributes.serialize();

        let expected_1 = r#"<svg class='SOAR' name='fan'>"#;
        let expected_2 = r#"<svg name='fan' class='SOAR'>"#;

        assert!(serialized.as_str() == expected_1 || serialized.as_str() == expected_2);
    }
}
