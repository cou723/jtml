mod converter;
mod get_bnf;
use bnf;
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
        let compiled = compile(file_text);

        // write to file
        let mut file = match fs::File::create(Path::new(&filename).with_extension("html")) {
            Ok(file) => file,
            Err(_) => {
                eprintln!("Error creating file {}", filename);
                continue;
            }
        };
        file.write(compiled?.as_bytes())?;
    }
    Ok(())
}


fn compile(text: String) -> Result<String, converter::CompileError> {
    let grammar: Box<bnf::Grammar> = Box::new(get_bnf::get_bnf().parse().expect("Failed to parse grammar"));
    let mut parse_trees = grammar.parse_input(text.as_str());
    let parse_tree = match parse_trees.next() {
        Some(parse_tree) => parse_tree,
        _ => return Err(converter::CompileError::ParseError),
    };
    converter::convert_elements(&parse_tree)
}
