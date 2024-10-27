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
}
