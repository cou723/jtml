// このフォーマッターはコメントを消してしまう。続きを作成する場合はprettierを利用したformatterを開発する。

use std::fs;
use std::path::Path;

use jtml::lexer::lexer;
use jtml::parser::{
    parser, Child, Document,
    {element::Attribute, element::Attributes, element::Children, Element},
};
use structopt::StructOpt;
#[derive(StructOpt)]
struct Cli {
    #[structopt(required = true, min_values = 1)]
    filenames: Vec<String>,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Cli::from_args();
    let filenames = args.filenames;
    for filename in filenames {
        // read text
        let path = Path::new(&filename);
        if path.is_dir() {
            eprintln!("{} is a directory", filename);
            continue;
        }
        let file_text = match fs::read_to_string(&filename) {
            Ok(text) => text,
            Err(_) => {
                eprintln!("Error reading from {}", filename);
                continue;
            }
        };
        // format
        /* let formatted =  */
        format(file_text);
        // write to file
        // let mut file = match fs::File::create(Path::new(&filename).with_extension("html")) {
        //     Ok(file) => file,
        //     Err(_) => {
        //         eprintln!("Error creating file {}", filename);
        //         continue;
        //     }
        // };
        // file.write(formatted.as_bytes())?;
    }
    Ok(())
}

fn format(text: String) -> String {
    let tokens = lexer(text);
    let ast = parser(&mut tokens.unwrap());
    let formatted = ast.unwrap().print(0);
    println!("{}", formatted);
    formatted
}

// --- Formatter trait ---

trait Formatter {
    fn print(&self, indent_deps: usize) -> String;
}

impl Formatter for Document {
    fn print(&self, indent_deps: usize) -> String {
        let mut printed = String::new();

        for element in &self.elements {
            printed += &element.print(indent_deps);
            printed.push('\n');
        }

        printed
    }
}

impl Formatter for Element {
    fn print(&self, indent_deps: usize) -> String {
        let mut printed = String::new();

        printed += &self.element_name.clone();

        printed += "(";
        printed += &self.attributes.print(indent_deps);
        printed += ")";

        printed += "{";
        if self.children.len() > 0 {
            printed += "\n";
            printed += &self.children.print(indent_deps + 1);
            printed += " ".repeat(indent_deps * 4).as_str();
        }
        printed += "}";

        printed
    }
}

impl Formatter for Attributes {
    fn print(&self, indent_deps: usize) -> String {
        let mut printed = String::new();
        for attribute in self {
            printed += &attribute.print(indent_deps);
            printed.push(' ');
        }

        // 最後一文字余計に入れたスペースを削除
        if self.len() > 0 {
            printed.pop();
        }

        printed
    }
}

impl Formatter for Attribute {
    fn print(&self, _indent_deps: usize) -> String {
        format!("{}={}", self.0, self.1)
    }
}

impl Formatter for Children {
    fn print(&self, indent_deps: usize) -> String {
        let mut printed = String::new();

        for child in self {
            printed += &" ".repeat(indent_deps * 4);
            printed += &child.print(indent_deps);
            printed.push('\n')
        }

        printed
    }
}

impl Formatter for Child {
    fn print(&self, indent_deps: usize) -> String {
        match self {
            Child::Element(element) => element.print(indent_deps),
            Child::Text(text) => text.clone(),
        }
    }
}

// test
#[cfg(test)]
mod test {
    use super::{Attributes, Child, Children, Element, Formatter};

    #[test]
    fn child() {
        let child = Child::Text("test".to_string());
        assert_eq!(child.print(0), "test");

        let child = Child::Element(Element {
            element_name: "p".to_string(),
            attributes: Attributes::new(),
            children: Children::new(),
        });
        assert_eq!(child.print(0), "p(){}");

        let child = Child::Element(Element {
            element_name: "p".to_string(),
            attributes: Attributes::new(),
            children: Children::from(vec![Child::Text("test".to_string())]),
        });
        assert_eq!(child.print(0), "p(){\n    test\n}");

        let child = Child::Element(Element {
            element_name: "p".to_string(),
            attributes: Attributes::new(),
            children: Children::from(vec![
                Child::Text("test".to_string()),
                Child::Text("test".to_string()),
            ]),
        });
        assert_eq!(child.print(0), "p(){\n    test\n    test\n}");

        let child = Child::Element(Element {
            element_name: "p".to_string(),
            attributes: Attributes::new(),
            children: Children::from(vec![
                Child::Text("test".to_string()),
                Child::Element(Element {
                    element_name: "p".to_string(),
                    attributes: Attributes::new(),
                    children: Children::new(),
                }),
            ]),
        });
        assert_eq!(child.print(0), "p(){\n    test\n    p(){}\n}");

        let child = Child::Element(Element {
            element_name: "p".to_string(),
            attributes: Attributes::new(),
            children: Children::from(vec![
                Child::Text("test".to_string()),
                Child::Element(Element {
                    element_name: "p".to_string(),
                    attributes: Attributes::new(),
                    children: Children::from(vec![
                        Child::Text("test".to_string()),
                        Child::Text("test".to_string()),
                    ]),
                }),
            ]),
        });
        assert_eq!(
            child.print(0),
            "p(){\n    test\n    p(){\n        test\n        test\n}\n}"
        );
    }

    #[test]
    fn attribute() {
        let attribute = ("test".to_string(), "test".to_string());
        assert_eq!(attribute.print(0), "test=test");

        assert_eq!(Attributes::new().print(0), "");
    }

    #[test]
    fn attributes() {
        let attributes = vec![
            ("test".to_string(), "test".to_string()),
            ("test".to_string(), "test".to_string()),
            ("test".to_string(), "test".to_string()),
        ];
        assert_eq!(
            Attributes::from(attributes).print(0),
            "test=test test=test test=test"
        );
    }
}
