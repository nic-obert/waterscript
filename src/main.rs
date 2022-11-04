mod files;
mod token;
mod vm;
mod syntax_tree;
mod tokenizer;
mod types;
mod error;
mod byte_code;
mod jit;


use clap::Parser;
use std::path::PathBuf;


#[derive(Parser)]
#[clap(author, about, version)]
struct Cli {

    /// The input file to execute
    #[clap(value_parser)]
    pub input_file: PathBuf,

    /// Verbose mode
    #[clap(short, long, action)]
    pub verbose: bool,
}


fn main() {
    
    let args = Cli::parse();

    let script = files::load_file(&args.input_file);

    let mut tokens = tokenizer::tokenize(&script);

    let syntax_tree = syntax_tree::SyntaxTree::from_tokens(&mut tokens.extract_tokens(), &script);
    
    let jit = jit::Jit::from_syntax_tree(syntax_tree, &script);

}

