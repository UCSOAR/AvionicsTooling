use std::fmt::Display;

pub struct Options;

impl Display for Options {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let options: &[&str] = &[
            "\t1 - Generate Svelte component from SVG",
            "\t2 - Set color mapping",
            "\t3 - Set SVG file path",
            "\t4 - Set style file path",
            "\t5 - Save and exit",
        ];

        write!(f, "Options:\n{}", options.join("\n"))
    }
}

impl Options {
    #[inline(always)]
    pub fn parse(input: &str) -> Option<u8> {
        input.parse::<u8>().ok()
    }
}
