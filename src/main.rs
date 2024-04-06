mod html_generator;
mod html_generator_error;
pub mod jtml_lexer;
pub mod jtml_parser;

use html_generator::from_jtml;
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

        // convert
        let html = match from_jtml(file_text, true) {
            Ok(converted) => converted,
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
        file.write(html.as_bytes())?;
    }
    Ok(())
}
