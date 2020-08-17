#[derive(Debug, PartialEq)]
pub struct CreateTableAST {
    pub name: String,
    pub props: Vec<TableProperty>,
    pub pk: Option<String>,
    pub if_exists: bool
}

#[derive(Debug,PartialEq)]
pub struct TableProperty {
    pub name: String,
    pub _type: Type,
    pub nullable: bool,
}

#[derive(Debug, PartialEq)]
pub enum Type {
    Int,
    String,
    Bool
}

#[derive(Debug, PartialEq)]
pub enum AST {
    CreateTable(CreateTableAST)
}