use crate::language::ast::InsertAST;
use crate::language::ast::AST::Insert;
use crate::language::lexer::Literal;
use std::collections::LinkedList;

use super::Token;
use super::TypeToken;
use super::CreateTableAST;
use super::TableProperty;
use super::Type;
use super::AST;

pub fn parse(ts: Vec<Token>) -> Result<AST, String> {
    return match &ts[0] {
        Token::Create => parse_create(ts),
        Token::Insert => parse_insert(ts),
        t             => Err(String::from("Unexpected keyword") + &token_to_string1(t))
    }
}

fn parse_create(ts: Vec<Token>) -> Result<AST, String> {
    let ast = match &ts[1] {
        Token::Table => parse_create_table(ts),
        t            => Err(String::from("Unexpected keyword") + &token_to_string1(t))
    }?;

    return Ok(AST::CreateTable(ast));
}

fn parse_create_table(_ts: Vec<Token>) -> Result<CreateTableAST, String> {

    let mut ts: LinkedList<Token> = LinkedList::new();

    for t in _ts {
        ts.push_back(t);
    }

    match ts.pop_front() {
        Some(Token::Create) => Ok(()),
        _ => Err(String::from("Expected CREATE keyword"))
    }?;

    match ts.pop_front() {
        Some(Token::Table) => Ok(()),
        _ => Err(String::from("Expected TABLE keyword"))
    }?;

    println!("{:?}", ts);

    let name = match ts.pop_front() {
        Some(Token::Id(name)) => Ok(name),
        Some(t) => Err(String::from("Unexpected token, expected Id: ") + &token_to_string(t)),
        None => Err(String::from("Unexpected end of query"))
    }?;

    return match ts.pop_front() {
            Some (Token::Lpr) =>  {
                let mut property_defs : Vec<TableProperty> = Vec::new();
                let mut buffer : Vec<Token> = Vec::new();
                
                for t in ts {
                    match t {
                        Token::Id(_) if buffer.len() == 0 => buffer.push(t),
                        Token::Type(_) if buffer.len() == 1 => buffer.push(t),
                        Token::Non if buffer.len() == 2 => buffer.push(t),
                        Token::Null if buffer.len() == 3 => buffer.push(t),
                        Token::Comma if buffer.len() > 1 => {
                            property_defs.push(tokens_to_tableproperty(buffer)?);
                            buffer = Vec::new();
                        },
                        Token::Rpr if buffer.len() > 1 => {
                            property_defs.push(tokens_to_tableproperty(buffer)?);
                            buffer = Vec::new();
                        }
                        _ => return Err(String::from("Unexpected token: ") + &token_to_string(t))
                    }
                }

                if buffer.len() > 1 {
                    property_defs.push(tokens_to_tableproperty(buffer)?);
                }

                return Ok(CreateTableAST {
                    name: name,
                    props: property_defs,
                    pk: None,
                    if_exists: false
                });
            },
            Some (t) => Err(String::from("Unexpected token, expected (: ") + &token_to_string(t)),
            None => Err(String::from("Unexpected end of query"))
        }
}


fn type_to_string1(t: &TypeToken) -> String {
    return match t {
        TypeToken::Int => String::from("INT"),
        TypeToken::String => String::from("STRING"),
        TypeToken::Bool => String::from("BOOL"),
    }
}

fn token_to_string(t: Token) -> String {
    return token_to_string1(&t)
}

fn token_to_string1(t: &Token) -> String {
    return match t {
        Token::Create => String::from("CREATE"),
        Token::Table => String::from("TABLE"),
        Token::Lpr => String::from("LPR"),
        Token::Rpr => String::from("RPR"),
        Token::Id(s) => String::from("Id(") + &String::from(s) + &")",
        Token::Comma => String::from(","),
        Token::Non => String::from("NON"),
        Token::Null => String::from("NULL"),
        Token::Type(tt) => String::from("TYPE(") + &type_to_string1(tt) + &")" ,
        Token::Insert => String::from("INSERT"),
        Token::Values => String::from("VALUES"),
        Token::Literal(_) => String::from("LITERAL"),
        Token::Into => String::from("INTO"),
    }
}


fn tokens_to_tableproperty (tokens: Vec<Token>) -> Result<TableProperty, String> {
    // let tokens = &_tokens
    let name = match &tokens[0] { 
        Token::Id(n) => Ok(String::from(n)), 
        t            => Err(String::from("Unexpected token, expected Id: ") + &token_to_string1(t)) 
    }?;

    let _type = match &tokens[1] { 
        Token::Type(TypeToken::Bool)    => Ok(Type::Bool),
        Token::Type(TypeToken::Int)     => Ok(Type::Int),
        Token::Type(TypeToken::String)  => Ok(Type::String),
        t                               => Err(String::from("Unexpected token, expected Type: ") + &token_to_string1(t)),

    }?;

    let nullable = if tokens.len() == 4 {
        if tokens[2] == Token::Non && tokens[3] == Token::Null {
           true
        } else { false }
    } else { false };
    
    return Ok(TableProperty { name: name, _type: _type, nullable: nullable })
}

fn parse_insert(_ts: Vec<Token>) -> Result<AST, String> {
    let mut ts: LinkedList<Token> = LinkedList::new();

    for t in _ts {
        ts.push_back(t);
    }

    match ts.pop_front() {
        Some(Token::Insert) => Ok(()),
        _                   => Err(String::from("Expected Insert keyword"))
    }?;

    match ts.pop_front() {
        Some(Token::Into) => Ok(()),
        _                 => Err(String::from("Expected Into keyword"))
    }?;

    let table = match ts.pop_front() {
        Some(Token::Id(t)) => Ok(t),
        _                  => Err(String::from("Expected Id keyword"))
    }?;

    match ts.pop_front() {
        Some(Token::Values) => Ok(()),
        _                 => Err(String::from("Expected Values keyword"))
    }?;

    match ts.pop_front() {
        Some(Token::Lpr) => Ok(()),
        _                 => Err(String::from("Expected Lpr keyword"))
    }?;

    let mut values: Vec<Literal> = Vec::new();

    for t in ts {
        match t {
            Token::Literal(l) => values.push(l),
            // todo: check if proper seperated
            Token::Comma => {}
            // todo: check if proper closed
            Token::Rpr => {},
            _ => return Err(String::from("Expected Literal"))
        }
    }

    return Ok(AST::Insert(InsertAST { table: table, values: values }));

}