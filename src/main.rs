use std::io;

mod language;

fn main() {
    println!("Welcome to the SQL RELP");
    
    loop {
        let mut query = String::new();

        io::stdin()
            .read_line(&mut query)
            .expect("");

        if query.starts_with(":exit") { break; }

        let ts = language::lexer::lex(query);

        println!("{:?}", ts);


        let ast = language::parser::parse(ts);
        
        println!("{:?}", ast);
    }
}