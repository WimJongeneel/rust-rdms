use crate::language::lexer::Literal;
use crate::language::ast::InsertAST;
use std::fs;

pub fn insert(ast: InsertAST) -> Result<(), String> {

    // todo: get table metadata and validate types

    match fs::write(data_dir() + &ast.table + &"/table", get_data(ast.values)) {
        Err(e) => Err(format!("Error while creating and populating metadata file: {}", e)),
        _ => Ok(())
    }?;

    Ok(())
}

fn get_data(d: Vec<Literal>) -> String {
    let mut r = String::new();

    for c in d {
        // todo: don't add ' ' to start of line
        r = r + " " + &match c {
            Literal::Bool(b) => String::from(if b { "true" } else { "false" }),
            // todo: escaping
            Literal::String(s) => String::from("'") + &s + &"'",
            Literal::Int(i) => i.to_string()
        };
    }

    r
}


// todo: make config helpers
fn data_dir() -> String { return String::from("C:/Users/Wim/Desktop/rust-rdms/database/"); }
