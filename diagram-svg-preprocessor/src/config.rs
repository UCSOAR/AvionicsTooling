use crate::svg::{css_color::CssColor, var_color::VarColor};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};

const CONFIG_FILE_NAME: &str = "config.json";

#[derive(Serialize, Deserialize)]
pub struct Config {
    style_file_path: String,
    svg_file_path: String,
    output_file_path: String,
    color_mappings: HashMap<CssColor, VarColor>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            style_file_path: "style.css".to_string(),
            svg_file_path: "Background-Light.svg".to_string(),
            output_file_path: "Diagram.svelte".to_string(),
            color_mappings: HashMap::new(),
        }
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color_mappings = if self.color_mappings.len() == 0 {
            "\tNone".to_string()
        } else {
            self.color_mappings
                .iter()
                .map(|(css_color, var_color)| {
                    format!("\t{} -> {}\n", css_color.as_str(), var_color.as_str())
                })
                .collect::<String>()
        };

        write!(
            f,
            "Current configuration:\nStyle file path: \"{}\"\nSVG file path: \"{}\"\nOutput file path: \"{}\"\nColor mappings:\n{}\n",
            self.style_file_path, self.svg_file_path, self.output_file_path, color_mappings
        )
    }
}

impl Config {
    #[inline(always)]
    pub fn color_mappings(&self) -> &HashMap<CssColor, VarColor> {
        &self.color_mappings
    }

    #[inline(always)]
    pub fn set_color_mapping(&mut self, css_color: CssColor, var_color: VarColor) {
        self.color_mappings.insert(css_color, var_color);
    }

    #[inline(always)]
    pub fn set_style_file_path(&mut self, style_file_path: &str) {
        self.style_file_path = style_file_path.to_string();
    }

    #[inline(always)]
    pub fn set_svg_file_path(&mut self, svg_file_path: &str) {
        self.svg_file_path = svg_file_path.to_string();
    }

    #[inline(always)]
    pub fn set_output_file_path(&mut self, output_file_path: &str) {
        self.output_file_path = output_file_path.to_string();
    }

    pub fn from_file() -> Result<Self> {
        let config_file = std::fs::read_to_string(CONFIG_FILE_NAME)?;
        let config = serde_json::from_str::<Self>(config_file.as_str())?;

        Ok(config)
    }

    pub fn save_to_file(&self) -> Result<()> {
        let config_file = serde_json::to_string_pretty(self)?;
        std::fs::write(CONFIG_FILE_NAME, config_file)?;

        Ok(())
    }

    pub fn output_file(&self, raw_svelte_text: &str) -> Result<()> {
        std::fs::write(self.output_file_path.as_str(), raw_svelte_text)?;

        Ok(())
    }

    pub fn read_style_file(&self) -> Result<String> {
        let style_file = std::fs::read_to_string(self.style_file_path.as_str())?;

        Ok(style_file)
    }

    pub fn read_svg_file(&self) -> Result<String> {
        let svg_file = std::fs::read_to_string(self.svg_file_path.as_str())?;

        Ok(svg_file)
    }
}
