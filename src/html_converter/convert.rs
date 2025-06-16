use crate::formatter::FormatConfig;

pub trait Convert {
    fn to_html(&self, ignore_comment: bool) -> String;
    fn to_jtml(&self, ignore_comment: bool, indent_depth: usize, config: &FormatConfig) -> String;
}
