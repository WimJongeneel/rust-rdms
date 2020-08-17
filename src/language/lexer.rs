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
    Null,
    Insert,
    Into,
    Values,
    Literal(Literal)
}   

#[derive(Debug)]
#[derive(PartialEq)]
pub enum TypeToken {
    Int,
    String,
    Bool
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Literal {
    Int(i32),
    Bool(bool),
    String(String)
    // todo: null literal
}

pub fn lex (s: String) -> Vec<Token> {
    // todo: string parsing with ignoring of tokens in the string
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
            '\r' if current == "" => {}
            '\r'                  => {
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
        "CREATE"    => Token::Create,
        "TABLE"     => Token::Table,
        "INT"       => Token::Type(TypeToken::Int),
        "STRING"    => Token::Type(TypeToken::String),
        "BOOL"      => Token::Type(TypeToken::Bool),
        "NON"       => Token::Non,
        "NULL"      => Token::Null,
        "TRUE"      => Token::Literal(Literal::Bool(true)),
        "FALSE"     => Token::Literal(Literal::Bool(false)),
        "INSERT"    => Token::Insert,
        "INTO"      => Token::Into,
        "VALUES"    => Token::Values,
        _           => match s.parse::<i32>() {
            Ok(i) => Token::Literal(Literal::Int(i)),
            Err(_) => Token::Id(s)
        }
    };
}