use super::code_block::CodeBlock;
use super::syntax_tree::SyntaxTree;


pub struct Jit<'a> {
    pub source: &'a str,
    pub root: CodeBlock<'a>,
}


impl Jit<'_> {

    pub fn from_syntax_tree<'a>(syntax_tree: &'a SyntaxTree, source: &'a str) -> Jit<'a> {
        Jit {
            source,
            root: CodeBlock::from_syntax_tree(syntax_tree, source, None),
        }
    }

}

