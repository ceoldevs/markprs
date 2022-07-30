use crate::lexer;

#[allow(dead_code)]
pub fn parse_md(tokens: &mut lexer::Lexer) {
    let mut token: lexer::lexer::Token;
    loop {
        token = tokens.next();

        if token.t_type == lexer::lexer::TokenType::EOF {
           break; 
        }

        match token.t_type {
            lexer::lexer::TokenType::H1 => {
                token = tokens.next();
                print!("<h1>{}</h1>", token.value)
            },
            lexer::lexer::TokenType::H2 => {
                token = tokens.next();
                print!("<h2>{}</h2>", token.value)
            },
            lexer::lexer::TokenType::H3 => {
                token = tokens.next();
                print!("<h3>{}</h3>", token.value)
            },
            lexer::lexer::TokenType::H4 => {
                token = tokens.next();
                print!("<h3>{}</h3>", token.value)
            },
            lexer::lexer::TokenType::H5 => {
                token = tokens.next();
                print!("<h5>{}</h5>", token.value)
            },
            lexer::lexer::TokenType::H6 => {
                token = tokens.next();
                print!("<h6>{}</h6>", token.value)
            },
            lexer::lexer::TokenType::EOL => {
                println!("")
            },
            _ => {
                println!("<p>{}</p>", token.value)
            },
        }
    }
}

