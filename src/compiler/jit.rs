use super::code_block::CodeBlock;
use super::syntax_tree::SyntaxTree;


pub struct Jit {
    pub root: CodeBlock,
}


impl Jit {

    pub fn from_syntax_tree(mut syntax_tree: SyntaxTree, source: &str) -> Jit {
        Jit {
            root: CodeBlock::from_syntax_tree(&mut syntax_tree, source, None),
        }
    }

}

