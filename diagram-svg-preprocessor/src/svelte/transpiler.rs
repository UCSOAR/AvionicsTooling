pub struct Transpiler;

impl Transpiler {
    pub fn to_svelte(raw_svg_text: &str, styling_lang: &str, raw_styling_text: &str) -> String {
        format!("{raw_svg_text}\n<style lang='{styling_lang}'>\n\t{raw_styling_text}\n</style>")
    }
}
