pub mod lexer;
pub mod parser;
pub mod tree;
pub mod codegen;

pub fn compile(ui: String) -> String {
    let tokens = lexer::tokenize(ui);
    let tree = parser::parse(tokens);
    codegen::generate(tree)
}
