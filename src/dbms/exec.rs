use crate::dbms::create_table::create_table;
use crate::dbms::insert::insert;
use crate::language::AST;

pub fn run(ast: AST) -> Result<(), String> {
    return match ast {
        AST::CreateTable(_ast) => create_table(_ast),
        AST::Insert(_ast) => insert(_ast)
    }
}