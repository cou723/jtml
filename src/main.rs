mod lexer;
mod parser;
use std::collections::VecDeque;
use std::fs;
use std::io::Write;
use std::path::Path;
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
        // compile
        let compiled = match compile(file_text) {
            Ok(compiled) => compiled,
            Err(e) => {
                eprintln!("Error compiling '{}' ({})", filename, e);
                continue;
            }
        };
        // write to file
        let mut file = match fs::File::create(Path::new(&filename).with_extension("html")) {
            Ok(file) => file,
            Err(_) => {
                eprintln!("Error creating file {}", filename);
                continue;
            }
        };
        file.write(compiled.as_bytes())?;
    }
    Ok(())
}

enum CompileError {
    UnexpectedToken(String),
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CompileError::UnexpectedToken(s) => write!(f, "Unexpected token: {}", s),
        }
    }
}

fn compile(text: String) -> Result<String, CompileError> {
    let mut tokens = VecDeque::from(match lexer::lexer(text){
        Ok(tokens) => tokens,
        Err(_e) => {
            return Err(CompileError::UnexpectedToken("lexical error".to_string()));
        },
    });

    let ast = match parser::parser(&mut tokens) {
        Ok(ast) => ast,
        Err(e) => return Err(CompileError::UnexpectedToken(e.to_string())),
    };
    Ok(ast.to_html())
}
