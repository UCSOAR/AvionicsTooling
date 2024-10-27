use anyhow::{anyhow, Result};
use regex::Regex;

use super::svg_attributes::SvgAttributes;

/**
 * Pattern for matching the opening tag of an SVG element.
 * Group 1: The opening tag of the SVG element.
 * Group 2: The attributes of the SVG element.
 * Group 3: The closing bracket of the SVG element.
 */
const SVG_OPEN_TAG_PATTERN: &str = r#"(<svg\s?+)([^>]*)(>)"#;

/**
 * Pattern for matching SVG tag attributes
 * Group 1: The attribute name
 * Group 2: The attribute value
 */
const SVG_ATTRIBUTE_PATTERN: &str = r#"(\w+)=["\']([^"\']+)["\']"#;

pub fn parse_attributes(raw_svg_text: &str) -> Result<(SvgAttributes, String)> {
    let svg_open_tag_regex =
        Regex::new(SVG_OPEN_TAG_PATTERN).expect("Failed to parse SVG open tag pattern.");
    let svg_attr_regex =
        Regex::new(SVG_ATTRIBUTE_PATTERN).expect("Failed to parse XML attribute pattern.");

    let svg_open_tag_caps = svg_open_tag_regex
        .captures(raw_svg_text)
        .ok_or(anyhow!("No capture groups found in provided SVG text."))?;

    let svg_tag_text = svg_open_tag_caps.get(0).ok_or(anyhow!(
        "Somehow failed to acquire overall capture group for SVG opening tag."
    ))?;

    let unparsed_attributes = svg_open_tag_caps.get(2).map_or("", |a| a.as_str());
    let mut attributes = SvgAttributes::new();

    for (_, [attr_name, attr_value]) in svg_attr_regex
        .captures_iter(&unparsed_attributes)
        .map(|c| c.extract())
    {
        attributes.set_attribute(attr_name, attr_value);
    }

    Ok((attributes, svg_tag_text.as_str().to_string()))
}

mod test {
    use crate::svg::{parser::parse_attributes, svg_attributes::SvgAttributes};

    #[test]
    fn test_svg_no_attributes_is_ok() {
        let input = r#"<svg>"#;
        let expected = SvgAttributes::new();
        let (attributes, _) = parse_attributes(input).unwrap();

        assert_eq!(attributes, expected);
    }

    #[test]
    fn test_parse_open_tag_overall_text_equals_svg_section() {
        let overall = r#"<!-- COMMENT --><svg class='tst' name="soarfan"><!-- COMMENT -->"#;
        let expected = r#"<svg class='tst' name="soarfan">"#;
        let (_, tag_text) = parse_attributes(overall).unwrap();

        assert_eq!(tag_text.as_str(), expected);
    }

    #[test]
    fn test_attributes_are_parsed_correctly() {
        let input = r#"<svg class='hello' name="SOAR">"#;
        let (attributes, _) = parse_attributes(input).unwrap();

        let mut expected = SvgAttributes::new();
        expected.set_attribute("class", "hello");
        expected.set_attribute("name", "SOAR");

        assert_eq!(attributes, expected);
    }

    #[test]
    fn test_non_svg_is_err() {
        let input = r#"<div class="test"></div>"#;
        let result = parse_attributes(input);

        assert!(result.is_err());
    }

    #[test]
    fn test_only_svg_closing_is_err() {
        let input = r#"</svg>"#;
        let result = parse_attributes(input);

        assert!(result.is_err());
    }
}
