use bnf;
use bnf::ParseTree;
use std::fmt;

#[derive(Debug)]
pub enum CompileError {
    ParseError,
    IOError,
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CompileError::ParseError => write!(f, "ParseError"),
            CompileError::IOError => write!(f, "IOError"),
        }
    }
}

impl std::error::Error for CompileError {}

pub fn convert_elements(elements_tree: &ParseTree) -> Result<String, CompileError> {
    let mut result = String::new();
    for element in elements_tree.rhs_iter() {
        match element {
            bnf::ParseTreeNode::Nonterminal(element) => {
                result.push_str(&convert_element(element)?);
            }
            _ => return Err(CompileError::ParseError),
        }
    }
    Ok(result)
}

fn convert_element(element_tree: &ParseTree) -> Result<String, CompileError> {
    if element_tree.rhs_iter().count() == 1 {
        return convert_string(element_tree);
    } else {
        return convert_block_element(element_tree);
    }
}

fn convert_block_element(block_element_tree: &ParseTree) -> Result<String, CompileError> {
    let mut result = String::new();
    let mut iter = block_element_tree.rhs_iter();
    let element_name = match iter.next() {
        Some(element_name) => match element_name {
            bnf::ParseTreeNode::Terminal(terminal) => terminal,
            _ => return Err(CompileError::ParseError),
        },
        _ => return Err(CompileError::ParseError),
    };
    let attributes = match iter.next() {
        Some(attributes) => match attributes {
            bnf::ParseTreeNode::Nonterminal(non_terminal) => {
                match convert_attributes(non_terminal) {
                    Ok(n) => n,
                    Err(e) => return Err(e),
                }
            }
            _ => return Err(CompileError::ParseError),
        },
        _ => return Err(CompileError::ParseError),
    };
    let value = match iter.next() {
        Some(value) => match value {
            bnf::ParseTreeNode::Nonterminal(value) => match convert_element(value) {
                Ok(n) => n,
                Err(e) => return Err(e),
            },
            _ => return Err(CompileError::ParseError),
        },
        _ => return Err(CompileError::ParseError),
    };
    result.push_str("<");
    result.push_str(element_name);
    result.push_str(" ");
    result.push_str(attributes.as_str());
    result.push_str(">");
    result.push_str(value.as_str());
    result.push_str("</");
    result.push_str(element_name);
    result.push_str(">");
    Ok(result)
}

fn convert_attributes(attributes_tree: &ParseTree) -> Result<String, CompileError> {
    let mut result = String::new();
    let mut iter = attributes_tree.rhs_iter();
    let left_parenthesis = match iter.next() {
        Some(lp) => match lp {
            bnf::ParseTreeNode::Terminal(lp) => lp,
            _ => return Err(CompileError::ParseError),
        },
        _ => return Err(CompileError::ParseError),
    };

    let mut attribute_str = String::new();
    while let Some(attribute) = iter.next() {
        match attribute {
            bnf::ParseTreeNode::Nonterminal(non_terminal) => {
                match convert_attribute(non_terminal) {
                    Ok(n) => attribute_str += n.as_str(),
                    Err(e) => return Err(e),
                }
            }
            bnf::ParseTreeNode::Terminal(terminal) => {
                if *terminal == ")" {
                    continue;
                } else {
                    return Err(CompileError::ParseError);
                }
            }
        }
    }

    let right_parenthesis = match iter.next() {
        Some(rp) => match rp {
            bnf::ParseTreeNode::Terminal(rp) => rp,
            _ => return Err(CompileError::ParseError),
        },
        _ => return Err(CompileError::ParseError),
    };
    return Ok(format!("( {} )", attribute_str));
}

fn convert_attribute(attribute_tree: &ParseTree) -> Result<String, CompileError> {
    let mut result = String::new();
    let mut iter = attribute_tree.rhs_iter();
    let key = match iter.next() {
        Some(k) => match k {
            bnf::ParseTreeNode::Terminal(s) => s,
            _ => return Err(CompileError::ParseError),
        },
        _ => return Err(CompileError::ParseError),
    };
    let equal = match iter.next() {
        Some(e) => match e {
            bnf::ParseTreeNode::Terminal(s) => s,
            _ => return Err(CompileError::ParseError),
        },
        _ => return Err(CompileError::ParseError),
    };
    let value = match iter.next() {
        Some(e) => match e {
            bnf::ParseTreeNode::Terminal(s) => s,
            _ => return Err(CompileError::ParseError),
        },
        _ => return Err(CompileError::ParseError),
    };
    return Ok(format!("{} = {}", key, value));
}

fn convert_string(string_tree: &ParseTree) -> Result<String, CompileError> {
    let mut result = String::new();
    let mut iter = string_tree.rhs_iter();
    match iter.next() {
        Some(d_quote) => match d_quote {
            bnf::ParseTreeNode::Terminal(terminal) => {
                if *terminal == "\"" {
                    result.push_str(terminal);
                } else {
                    return Err(CompileError::ParseError);
                }
            }
            _ => return Err(CompileError::ParseError),
        },
        _ => return Err(CompileError::ParseError),
    };

    let string = match iter.next() {
        Some(string) => match string {
            bnf::ParseTreeNode::Terminal(terminal) => terminal,
            _ => return Err(CompileError::ParseError),
        },
        _ => return Err(CompileError::ParseError),
    };

    // WETである。要改善
    match iter.next() {
        Some(d_quote) => match d_quote {
            bnf::ParseTreeNode::Terminal(terminal) => {
                if *terminal == "\"" {
                    result.push_str(terminal);
                } else {
                    return Err(CompileError::ParseError);
                }
            }
            _ => return Err(CompileError::ParseError),
        },
        _ => return Err(CompileError::ParseError),
    };
    result.push_str(string);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_bnf;
    use bnf;

    #[test]
    fn test_convert_string() {
        let bnf_text = get_bnf::get_string_bnf().to_string() + "\n" + get_bnf::get_letter_bnf();
        println!("{}", bnf_text);
        let grammar: bnf::Grammar = bnf_text.parse().unwrap();
        let input = r#""test""#;
        let expected = r#""test""#;
        println!("{:?}", grammar.parse_input(input).next());
        assert_eq!(
            convert_string(&(grammar.parse_input(input).next().unwrap())).unwrap(),
            expected
        );
    }
}
