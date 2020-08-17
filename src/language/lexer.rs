#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {
    Create,
    Table,
    Lpr,
    Rpr,
    Id(String),
    Type(TypeToken),
    Comma,
    Non,
    Null
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum TypeToken {
    Int,
    String,
    Bool
}

pub fn lex (s: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut current: String = String::new();

    for c in s.chars() {
        match c  {
            '(' if current == ""  => tokens.push(Token::Lpr),
            '('                   => {
                tokens.push(string_to_token(current));
                current = String::new();
                tokens.push(Token::Lpr);
            },
            ')' if current == ""  => tokens.push(Token::Rpr),
            ')'                   => {
                tokens.push(string_to_token(current));
                current = String::new();
                tokens.push(Token::Rpr);
            },
            ',' if current == ""  => tokens.push(Token::Comma),
            ','                   => {
                tokens.push(string_to_token(current));
                current = String::new();
                tokens.push(Token::Comma);
            },
            ' ' if current == ""  => {}
            ' '                   => {
                tokens.push(string_to_token(current));
                current = String::new();
            },
            '\t' if current == ""  => {}
            '\t'                   => {
                tokens.push(string_to_token(current));
                current = String::new();
            },
            '\n' if current == "" => {}
            '\n'                  => {
                tokens.push(string_to_token(current));
                current = String::new();
            },
            ';' if current == ""  => {}
            ';'                   => {
                tokens.push(string_to_token(current));
                current = String::new();
            },
            _                     => current.push(c)
        }
    }

    return tokens;
}

fn string_to_token(s: String) -> Token {
    return match &s[..] {
        "CREATE" => Token::Create,
        "TABLE" => Token::Table,
        "INT" => Token::Type(TypeToken::Int),
        "STRING" => Token::Type(TypeToken::String),
        "BOOL" => Token::Type(TypeToken::Bool),
        "NON" => Token::Non,
        "NULL" => Token::Null,
        _ => Token::Id(s)
    };
}