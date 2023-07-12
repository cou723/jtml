mod lexer;
use std::fs;
use lexer;
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
        // let compiled = compile(file_text);
        // let compiled = Ok("test");
        // // write to file
        // let mut file = match fs::File::create(Path::new(&filename).with_extension("html")) {
        //     Ok(file) => file,
        //     Err(_) => {
        //         eprintln!("Error creating file {}", filename);
        //         continue;
        //     }
        // };
        // file.write(compiled?.as_bytes())?;
    }
    Ok(())
}

struct Node{
    tag: String,
    attributes: Vec<(String,String)>,
    children: Vec<Node>,
}

enum ParserError{
    UnexpectedToken(String),
}

fn parser(tokens:lexer::Token) -> Result<Node,ParserError>{
    document(tokens)
}

fn document(tokens:lexer::Token) -> Result<Node,ParserError>{
    let mut node=Node{
        tag: tokens.pop().unwrap().to_string(),
        attributes: attributes(tokens)?,
        children: Vec::new(),
    };
    while(tokens.len() != 0){
        let (node,rest) = element(tokens)?;
        tokens = rest;
    }
}

fn attributes(tokens:lexer::Token) -> Result<Vec<(String,String)>,ParserError>{
    let mut attributes:Vec<(String,String)> = Vec::new();
    while(tokens.len() != 0){
        let (attribute,rest) = attribute(tokens)?;
        attributes.push(attribute);
        tokens = rest;
    }
    Ok(attributes)
}

fn attribute(mut tokens:&Vec<lexer::Token>) -> Result<(String,String),ParserError>{
    let mut key:String;
    let mut value:String;
    if let lexer::Token::Id(_key) = tokens[0]{
        key=_key;
        tokens.pop();
    }else{
        return Err(ParserError::UnexpectedToken(tokens[0].to_string()));
    }
    if let lexer::Token::Equal = tokens[1]{
    }else {
        return Err(ParserError::UnexpectedToken(tokens[1].to_string()));
    }
    if let lexer::Token::Text(_value) = tokens[2]{
        value=_value;
        tokens.pop();
    }else{
        return Err(ParserError:UnexpectedToken(tokens[2].to_string()));
    }
    Ok((key,value))

}

fn compile(text: String) -> Result<String, converter::CompileError> {
    let tokens = lexer::lexer(text);

}
