use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CssColor(String);

impl CssColor {
    pub fn new(color: &str) -> Result<Self> {
        let color = color.to_lowercase();

        if color.starts_with('#') {
            let hex_code = color.trim_start_matches('#');
            let is_valid_hex = hex_code.chars().all(|c| c.is_ascii_hexdigit());

            if is_valid_hex {
                return Ok(Self(color));
            }

            Err(anyhow!("Invalid CSS hex color code."))
        } else {
            let is_valid_color_name = color.chars().all(|c| c.is_alphabetic());

            if is_valid_color_name {
                return Ok(Self(color));
            }

            Err(anyhow!("Invalid CSS color name."))
        }
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

mod test {
    use super::CssColor;

    #[test]
    fn test_css_color_new() {
        let valid_hex_color = "#ff00ff";
        let invalid_hex_color = "#ff00fg";
        let valid_color_name = "red";
        let invalid_color_name = "red1";

        assert!(CssColor::new(valid_hex_color).is_ok());
        assert!(CssColor::new(invalid_hex_color).is_err());
        assert!(CssColor::new(valid_color_name).is_ok());
        assert!(CssColor::new(invalid_color_name).is_err());
    }

    #[test]
    fn test_css_color_as_str() {
        let color = "red";
        let css_color = CssColor::new(color).unwrap();

        assert_eq!(css_color.as_str(), color);
    }
}
