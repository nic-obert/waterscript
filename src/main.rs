mod files;
mod token;
mod vm;
mod syntax_tree;
mod tokenizer;
mod types;
mod error;
mod byte_code;


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

    let contents = files::load_file(&args.input_file);

    let mut tokens = tokenizer::tokenize(contents);

    let syntax_tree = syntax_tree::SyntaxTree::from_token_list(&mut tokens);
    
}

