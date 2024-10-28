pub struct Transpiler;

impl Transpiler {
    pub fn to_svelte(raw_svg_text: &str, raw_styling_text: &str) -> String {
        format!("{raw_svg_text}\n<style>\n{raw_styling_text}\n</style>")
    }
}
