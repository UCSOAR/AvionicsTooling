use crate::svg::{css_color::CssColor, var_color::VarColor};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};

const CONFIG_FILE_NAME: &str = "config.json";

#[derive(Serialize, Deserialize)]
pub struct Config {
    style_file_path: String,
    color_mappings: HashMap<CssColor, VarColor>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            style_file_path: "style.css".to_string(),
            color_mappings: HashMap::new(),
        }
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color_mappings = if self.color_mappings.len() == 0 {
            "None".to_string()
        } else {
            self.color_mappings
                .iter()
                .map(|(css_color, var_color)| {
                    format!("{} -> {}", css_color.as_str(), var_color.as_str())
                })
                .collect::<Vec<String>>()
                .join(", ")
        };

        write!(
            f,
            "Current configuration:\nStyle file path: \"{}\"\nColor mappings: {}",
            self.style_file_path, color_mappings
        )
    }
}

impl Config {
    pub fn from_file() -> Result<Self> {
        let config_file = std::fs::read_to_string(CONFIG_FILE_NAME)?;
        let config = serde_json::from_str::<Self>(config_file.as_str())?;

        Ok(config)
    }

    pub fn to_file(&self) -> Result<()> {
        let config_file = serde_json::to_string_pretty(self)?;
        std::fs::write(CONFIG_FILE_NAME, config_file)?;

        Ok(())
    }

    pub fn read_style_file(&self) -> Result<String> {
        let style_file = std::fs::read_to_string(self.style_file_path.as_str())?;

        Ok(style_file)
    }
}
