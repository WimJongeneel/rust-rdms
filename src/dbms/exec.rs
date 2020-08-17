use crate::dbms::create_table::create_table;
use crate::language::AST;

pub fn run(ast: AST) -> Result<(), String> {
    return match ast {
        AST::CreateTable(_ast) => create_table(_ast)
    }
}