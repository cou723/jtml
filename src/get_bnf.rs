pub fn get_bnf() -> String {
    BNF_LIST
        .iter()
        .fold(String::new(), |acc, f| acc + f() + "\n")
}

static BNF_LIST: [fn() -> &'static str; 9] = [
    get_letter_bnf,
    get_string_bnf,
    get_attribute_bnf,
    get_attributes_bnf,
    get_element_name_bnf,
    get_value_bnf,
    get_element_bnf,
    get_elements_bnf,
    get_document_bnf,
];

pub fn get_letter_bnf() -> &'static str {
    "<letter> ::= 'a' | 'b' | 'c' | 'd' | 'e' | 'f' |
'g' | 'h' | 'i' | 'j' | 'k' | 'l' |
'm' | 'n' | 'o' | 'p' | 'q' | 'r' |
's' | 't' | 'u' | 'v' | 'w' | 'x' |
'y' | 'z' |
'A' | 'B' | 'C' | 'D' | 'E' | 'F' |
'G' | 'H' | 'I' | 'J' | 'K' | 'L' |
'M' | 'N' | 'O' | 'P' | 'Q' | 'R' |
'S' | 'T' | 'U' | 'V' | 'W'|  'X'|
'Y' | 'Z'"
}
pub fn get_string_bnf() -> &'static str {
    "<string> ::= <letter> | <letter> <string>"
}
pub fn get_attribute_bnf() -> &'static str {
    "<attribute> ::= <string> '=' <string>"
}
pub fn get_attributes_bnf() -> &'static str {
    "<attributes> ::= '(' <attribute> ')' | '(' <attributes> ' ' <attribute> ')'"
}
pub fn get_element_name_bnf() -> &'static str {
    "<element-name> ::= 'html'|'div'|'head'|'body'|'title'|'p'|'h1'|'h2'|'h3'|'h4'|'h5'|'h6'|'ul'|'ol'|'li'|'a'|'img'|'span'|'br'|'hr'|'script'|'link'|'meta'|'style'|'table'|'tr'|'td'|'th'|'thead'|'tbody'|'tfoot'|'form'|'input'|'button'|'select'|'option'|'textarea'|'label'|'fieldset'|'legend'|'iframe'|'canvas'|'audio'|'video'|'source'|'nav'|'header'|'footer'|'section'|'article'|'aside'|'details'|'summary'|'figure'|'figcaption'|'mark'|'time'|'progress'|'meter'|'ruby'|'rt'|'rp'|'bdi'|'wbr'|'bdo'|'q'|'blockquote'|'cite'|'abbr'|'address'|'em'|'strong'|'small'|'s'|'cite'|'code'|'samp'|'kbd'|'var'|'sub'|'sup'|'i'|'b'|'u'|'tt'|'strike'|'big'|'pre'|'center'|'font'|'basefont'|'dir'|'menu'|'applet'|'object'|'param'|'embed'|'map'|'area'|'frame'|'frameset'|'noframes'|'iframe'|'del'|'ins'|'caption'|'col'|'colgroup'|'optgroup'|'thead'|'tbody'|'tfoot'|'tr'|'td'|'th'|'button'|'datalist'|'keygen'|'output'|'progress'|'meter'|'details'|'summary'|'command'|'menuitem'|'dialog'|'legend'|'fieldset'|'label'|'optgroup'|'option'|'textarea'|'input'|'select'|'button'|'form'|'style'|'script'|'noscript'|'template'|'slot'|'canvas'|'svg'|'math'|'audio'|'video'|'source'|'track'|'embed'|'object'|'param'|'iframe'|'frame'|'frameset'|'img'|'map'|'area'|'picture'|'source'|'track'|'embed'|'object'|'param'|'iframe'"
}
pub fn get_element_bnf() -> &'static str {
    "<element> ::= <element-name> <attributes> <value> | <string>"
}
pub fn get_value_bnf() -> &'static str {
    "<value> ::= '{' <elements> '}'"
}
pub fn get_elements_bnf() -> &'static str {
    "<elements> ::= <element> | <elements> <element>"
}
pub fn get_document_bnf() -> &'static str {
    "<document> ::= <elements>"
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_bnf;
    use bnf;

    #[test]
    fn get_letter_bnf() {
        let bnf_text = BNF_LIST[0]();
        assert!(bnf_text.parse::<bnf::Grammar>().is_ok());
    }

    #[test]
    fn get_string_bnf() {
        let bnf_text = BNF_LIST[0..1]
            .iter()
            .fold(String::new(), |acc, f| acc + f() + "\n");
        assert!(bnf_text.parse::<bnf::Grammar>().is_ok());
    }

    #[test]
    fn get_attribute_bnf() {
        let bnf_text = BNF_LIST[0..2]
            .iter()
            .fold(String::new(), |acc, f| acc + f() + "\n");
        assert!(bnf_text.parse::<bnf::Grammar>().is_ok());
    }

    #[test]
    fn get_attributes_bnf() {
        let bnf_text = BNF_LIST[0..3]
            .iter()
            .fold(String::new(), |acc, f| acc + f() + "\n");
        assert!(bnf_text.parse::<bnf::Grammar>().is_ok());
    }

    #[test]
    fn get_element_name_bnf() {
        let bnf_text = BNF_LIST[0..4]
            .iter()
            .fold(String::new(), |acc, f| acc + f() + "\n");
        assert!(bnf_text.parse::<bnf::Grammar>().is_ok());
    }

    #[test]
    fn get_value_bnf() {
        let bnf_text = BNF_LIST[0..5]
            .iter()
            .fold(String::new(), |acc, f| acc + f() + "\n");
        assert!(bnf_text.parse::<bnf::Grammar>().is_ok());
    }

    #[test]
    fn get_element_bnf() {
        let bnf_text = BNF_LIST[0..6]
            .iter()
            .fold(String::new(), |acc, f| acc + f() + "\n");
        assert!(bnf_text.parse::<bnf::Grammar>().is_ok());
    }

    #[test]
    fn get_elements_bnf() {
        let bnf_text = BNF_LIST[0..7]
            .iter()
            .fold(String::new(), |acc, f| acc + f() + "\n");
        assert!(bnf_text.parse::<bnf::Grammar>().is_ok());
    }

    #[test]
    fn get_document_bnf() {
        let bnf_text = BNF_LIST[0..8]
            .iter()
            .fold(String::new(), |acc, f| acc + f() + "\n");
        assert!(bnf_text.parse::<bnf::Grammar>().is_ok());
    }
}
