use crate::language::TableProperty;
use crate::language::Type;

use std::fs;
use std::fs::File;

use crate::language::ast::CreateTableAST;

pub fn create_table(ast: CreateTableAST) -> Result<(), String> {

    match fs::create_dir_all(data_dir() + &ast.name) {
        Err(e) => Err(format!("Error while creating data dir: {}", e)),
        _ => Ok(())
    }?;

    match File::create(data_dir() + &ast.name + &"/table") {
        Err(e) => Err(format!("Error while creating table file: {}", e)),
        _ => Ok(())
    }?;

    match File::create(data_dir() + &ast.name + &"/table") {
        Err(e) => Err(format!("Error while creating table file: {}", e)),
        _ => Ok(())
    }?;


    match fs::write(data_dir() + &ast.name + &"/metadata", get_metadata(ast.props)) {
        Err(e) => Err(format!("Error while creating and populating metadata file: {}", e)),
        _ => Ok(())
    }?;

    return Ok(());
}

fn get_metadata(tb: Vec<TableProperty>) -> String {
    let mut res = String::new();

    for p in tb {
        let t = match p._type {
            Type::Bool => "bool",
            Type::Int => "int",
            Type::String => "string"
        };

        let n = if p.nullable { "null" } else { "nonnull" };

        res = res + &p.name + " " + t + " " + n + "\n";
    }

    return res;
}

fn data_dir() -> String { return String::from("C:/Users/Wim/Desktop/rust-rdms/database/"); }