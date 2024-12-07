use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub struct VarColor(String);

impl VarColor {
    pub fn new(var_name: &str) -> Result<Self> {
        let is_valid_var_name = var_name.chars().all(|c| c.is_alphanumeric() || c == '-');

        if is_valid_var_name {
            Ok(Self(var_name.to_string()))
        } else {
            Err(anyhow!("Invalid CSS variable name."))
        }
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

mod test {
    use super::VarColor;

    #[test]
    fn test_var_color_new() {
        let valid_var_name = "--soar-fan";
        let invalid_var_name = "--soar fan";

        assert!(VarColor::new(valid_var_name).is_ok());
        assert!(VarColor::new(invalid_var_name).is_err());
    }

    #[test]
    fn test_var_color_as_str() {
        let var_name = "--soar-fan";
        let var_color = VarColor::new(var_name).unwrap();

        assert_eq!(var_color.as_str(), var_name);
    }
}
