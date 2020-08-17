use std::io;

mod language;
mod dbms;


fn main() -> Result<(), String> {
    println!("Welcome to the SQL RELP");
    
    loop {
        let mut query = String::new();

        io::stdin()
            .read_line(&mut query)
            .expect("");

        if query.starts_with(":exit") { break Err(String::from("User canceled RELP session")); }

        let ts = language::lex(query);

        println!("{:?}", ts);


        let ast = language::parser::parse(ts)?;
        
        println!("{:?}", ast);

        dbms::run(ast)?;
    }
}