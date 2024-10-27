use super::svg_attributes::SvgAttributes;
use crate::config::Config;
use regex::Regex;

pub struct Modifier<'a> {
    config: &'a Config,
    accumulator: String,
}

impl<'a> Modifier<'a> {
    pub fn new(config: &'a Config, initial_accumulator: &str) -> Self {
        Self {
            config,
            accumulator: initial_accumulator.to_string(),
        }
    }

    pub fn fix_svg_open_tag(
        &mut self,
        original_tag_text: &str,
        mut attributes: SvgAttributes,
    ) -> &mut Self {
        attributes.set_attribute("class", "diagram");
        attributes.set_attribute("fill", "none");
        attributes.set_attribute("xmlns", "http://www.w3.org/2000/svg");

        match (
            attributes.get_attribute("width"),
            attributes.get_attribute("height"),
        ) {
            (Some(width), Some(height)) => {
                let view_box = format!("0 0 {} {}", width, height);
                attributes.set_attribute("viewBox", view_box.as_str());
            }
            _ => {}
        }

        let new_attributes = attributes.serialize();

        self.accumulator =
            self.accumulator
                .as_str()
                .replacen(original_tag_text, new_attributes.as_str(), 1);

        self
    }

    pub fn map_colors(&mut self) -> &mut Self {
        let replacements = self
            .config
            .color_mappings()
            .iter()
            .map(|(css_color, var_color)| {
                (
                    Regex::new(css_color.as_str())
                        .expect("Failed to create regex for color mapping."),
                    var_color,
                )
            })
            .collect::<Box<[_]>>();

        self.accumulator = replacements
            .iter()
            .fold(self.accumulator.to_string(), |acc, (regex, var_color)| {
                regex
                    .replace_all(
                        acc.as_str(),
                        format!("var({})", var_color.as_str()).as_str(),
                    )
                    .to_string()
            })
            .to_string();

        self
    }

    pub fn accumulate(&'a self) -> &'a str {
        self.accumulator.as_str()
    }
}

mod test {
    use super::Modifier;
    use crate::{
        config::Config,
        svg::{css_color::CssColor, parser::Parser, var_color::VarColor},
    };

    #[test]
    pub fn test_modify_empty_tag() {
        let input = r#"<!-- Soar!!! --><svg><!-- comment -->"#;
        let (attributes, original_svg_string) = Parser::parse_attributes(input).unwrap();

        let config = Config::default();
        let mut modifier = Modifier::new(&config, input);
        let result = modifier
            .fix_svg_open_tag(original_svg_string.as_str(), attributes)
            .accumulate();

        assert!(result.contains("Soar!!"));
        assert!(result.contains("comment"));
        assert!(result.contains("class='diagram'"));
        assert!(result.contains("fill='none'"));
        assert!(result.contains("xmlns='http://www.w3.org/2000/svg'"));
        assert!(!result.contains("viewBox"));
    }

    #[test]
    pub fn test_modify_tag_with_dimensions() {
        let input = r#"<!-- whyyy --><svg width="69" height="420"><!-- :( -->"#;
        let (attributes, original_svg_string) = Parser::parse_attributes(input).unwrap();

        let config = Config::default();
        let mut modifier = Modifier::new(&config, input);
        let result = modifier
            .fix_svg_open_tag(original_svg_string.as_str(), attributes)
            .accumulate();

        assert!(result.contains("class='diagram'"));
        assert!(result.contains("fill='none'"));
        assert!(result.contains("xmlns='http://www.w3.org/2000/svg'"));
        assert!(result.contains("viewBox='0 0 69 420'"));
    }

    #[test]
    pub fn test_map_colors() {
        let input = "<svg fill='#ff00ff' stroke='red' fill='#ff00ff' stroke='blue'></svg>";

        let mut config = Config::default();

        config.set_color_mapping(
            CssColor::new("#ff00ff").unwrap(),
            VarColor::new("--test-color").unwrap(),
        );

        config.set_color_mapping(
            CssColor::new("red").unwrap(),
            VarColor::new("--test-red").unwrap(),
        );

        let mut modifier = Modifier::new(&config, input);
        let result = modifier.map_colors().accumulate();
        let expected = "<svg fill='var(--test-color)' stroke='var(--test-red)' fill='var(--test-color)' stroke='blue'></svg>";

        assert_eq!(result, expected);
    }
}
