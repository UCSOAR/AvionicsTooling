use super::svg_attributes::SvgAttributes;

pub fn modify_svg_open_tag(
    raw_svg_text: &str,
    original_tag_text: &str,
    mut attributes: SvgAttributes,
) -> String {
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

    raw_svg_text.replacen(original_tag_text, new_attributes.as_str(), 1)
}

mod test {
    use crate::svg::parser::parse_attributes;

    use super::modify_svg_open_tag;

    #[test]
    pub fn test_modify_empty_tag() {
        let input = r#"<!-- Soar!!! --><svg><!-- comment -->"#;
        let (attributes, original_svg_string) = parse_attributes(input).unwrap();
        let result = modify_svg_open_tag(input, original_svg_string.as_str(), attributes);

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
        let (attributes, original_svg_string) = parse_attributes(input).unwrap();
        let result = modify_svg_open_tag(input, original_svg_string.as_str(), attributes);

        assert!(result.contains("class='diagram'"));
        assert!(result.contains("fill='none'"));
        assert!(result.contains("xmlns='http://www.w3.org/2000/svg'"));
        assert!(result.contains("viewBox='0 0 69 420'"));
    }
}
