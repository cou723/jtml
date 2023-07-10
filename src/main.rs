use bnf::Grammar;
use bnf::ParseTreeNode;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use structopt::StructOpt;
#[derive(StructOpt)]
struct Cli {
    #[structopt(required = true, min_values = 1)]
    filenames: Vec<String>,
}

fn parse(text:&'static str) -> Result<bnf::ParseTree<'static>, bnf::Error> {
    let bnf:&'static str=
    "<program> ::= <statements>
    <statements> ::= <statement> | <statements> <statement>
    <value> ::= '{' <statements> '}'
    <statement> ::= <element-name> <attributes> <value> | '\"' <string> '\"'
    <element-name> ::= 'html'|'div'|'head'|'body'|'title'|'p'|'h1'|'h2'|'h3'|'h4'|'h5'|'h6'|'ul'|'ol'|'li'|'a'|'img'|'span'|'br'|'hr'|'script'|'link'|'meta'|'style'|'table'|'tr'|'td'|'th'|'thead'|'tbody'|'tfoot'|'form'|'input'|'button'|'select'|'option'|'textarea'|'label'|'fieldset'|'legend'|'iframe'|'canvas'|'audio'|'video'|'source'|'nav'|'header'|'footer'|'section'|'article'|'aside'|'details'|'summary'|'figure'|'figcaption'|'mark'|'time'|'progress'|'meter'|'ruby'|'rt'|'rp'|'bdi'|'wbr'|'bdo'|'q'|'blockquote'|'cite'|'abbr'|'address'|'em'|'strong'|'small'|'s'|'cite'|'code'|'samp'|'kbd'|'var'|'sub'|'sup'|'i'|'b'|'u'|'tt'|'strike'|'big'|'pre'|'center'|'font'|'basefont'|'dir'|'menu'|'applet'|'object'|'param'|'embed'|'map'|'area'|'frame'|'frameset'|'noframes'|'iframe'|'del'|'ins'|'caption'|'col'|'colgroup'|'optgroup'|'thead'|'tbody'|'tfoot'|'tr'|'td'|'th'|'button'|'datalist'|'keygen'|'output'|'progress'|'meter'|'details'|'summary'|'command'|'menuitem'|'dialog'|'legend'|'fieldset'|'label'|'optgroup'|'option'|'textarea'|'input'|'select'|'button'|'form'|'style'|'script'|'noscript'|'template'|'slot'|'canvas'|'svg'|'math'|'audio'|'video'|'source'|'track'|'embed'|'object'|'param'|'iframe'|'frame'|'frameset'|'img'|'map'|'area'|'picture'|'source'|'track'|'embed'|'object'|'param'|'iframe'
    <attributes> ::= '()' | '(' <attribute> ')' | '(' <attributes> ' ' <attribute> ')'
    <attribute> ::= <string> '=' <string> | <string>
    <string> ::= <letter> | <letter> <string>
    <letter> ::= 'a' | 'b' | 'c' | 'd' | 'e' | 'f' |
                 'g' | 'h' | 'i' | 'j' | 'k' | 'l' |
                 'm' | 'n' | 'o' | 'p' | 'q' | 'r' |
                 's' | 't' | 'u' | 'v' | 'w' | 'x' |
                 'y' | 'z' |
                 'A' | 'B' | 'C' | 'D' | 'E' | 'F' |
                 'G' | 'H' | 'I' | 'J' | 'K' | 'L' |
                 'M' | 'N' | 'O' | 'P' | 'Q' | 'R' |
                 'S' | 'T' | 'U' | 'V' | 'W'|  'X'|
                 'Y' | 'Z'
    ";
    let grammar: Grammar = bnf.parse().expect("Failed to parse grammar");
    let mut parse_trees = grammar.parse_input(text);
    match parse_trees.next() {
        Some(parse_tree) => Ok(parse_tree),
        _ => Err(bnf::Error::ParseError("Failed to parse input".to_string())),
    }
}
//     let sentence = grammar.generate();
// match sentence {
//     Ok(s) => println!("random sentence: {}", s),
//     Err(e) => println!("something went wrong: {}!", e)
// }

fn compile(text: String) -> String {
    let mut vec: Vec<&ParseTreeNode>;

    "hoge".to_string()
}

fn main() -> Result<(), io::Error> {
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
        file.write(compiled.as_bytes()).unwrap();
    }
    Ok(())
}
