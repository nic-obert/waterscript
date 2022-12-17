use super::token::Token;
use super::syntax_node::{SyntaxNode, self};


/// Represents a list of statements.
#[derive(Clone, Default, Debug)]
pub struct SyntaxTree {
    pub statements: Vec<SyntaxNode>,
}


impl SyntaxTree {

    pub fn from_tokens(tokens: &[Token], source: &str) -> SyntaxTree {
        let mut raw_statements = syntax_node::tokens_to_syntax_node_statements(tokens, source);

        let statements = raw_statements.iter_mut().map(
            |statement| syntax_node::parse_statement(statement, source)
        ).collect();

        SyntaxTree { statements }
    }

}

