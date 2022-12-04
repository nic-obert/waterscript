use std::collections::HashMap;

use crate::code_node::CodeNode;
use crate::syntax_tree::SyntaxTree;


/// Represents a scope source code block
pub struct CodeBlock<'a> {
    nodes: Vec<CodeNode<'a>>,
    symbols: HashMap<String, usize>,
    context: Option<*mut CodeBlock<'a>>,
}


impl CodeBlock<'_> {

    pub fn from_syntax_tree<'a>(syntax_tree: &'a SyntaxTree, source: &str, context: Option<*mut CodeBlock<'a>>) -> CodeBlock<'a> {
        let mut this = CodeBlock {
            nodes: Vec::new(),
            symbols: HashMap::new(),
            context,
        };
        
        for syntax_node in &syntax_tree.statements {
            this.nodes.push(CodeNode::from_syntax_node(syntax_node, source, &this));
        }

        this
    }


    pub fn declare_local(&mut self, name: &str) -> usize {
        let local_id = self.symbols.len();
        self.symbols.insert(name.to_string(), local_id);
        local_id
    }


    pub fn get_local_id(&self, name: &str) -> Option<usize> {
        self.symbols.get(name).map(|id| *id)
    }

}

