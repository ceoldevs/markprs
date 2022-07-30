use std::env;

mod reader;
mod lexer;
mod parser;

/*
* 1. read the file ✔️ 
* 2. break the text into lexems ➖
* 3. build the parse tree ❌
* 4. build AST ❌
* 5. convert to HTML ❌
*/

fn main() {

    let args: Vec<String> = env::args().collect();
    // println!("{}", args.len());

    if args.len() != 2 {
        panic!("filename argument missing");
    }

    let filename = &args[1];
    let contents = reader::read(filename.to_string());
    let mut lexemes = lexer::Lexer::new_u8(contents);
    /* loop {
        let token = lexer::Lexer::next(&mut lexemes);
        if token.t_type == lexer::lexer::TokenType::EOF {
            println!("{:?}", token);
            break;
        } else {
            println!("{:?}", token);
        }
    } */

    parser::parse_md(&mut lexemes);

    /* let token_1 = lexer::Lexer::next(&mut lexemes);
    let token_2 = lexer::Lexer::next(&mut lexemes);
    let token_3 = lexer::Lexer::next(&mut lexemes);
    let token_4 = lexer::Lexer::next(&mut lexemes);
    let token_5 = lexer::Lexer::next(&mut lexemes);

    println!("{:?}", token_1);
    println!("{:?}", token_2);
    println!("{:?}", token_3);
    println!("{:?}", token_4);
    println!("{:?}", token_5); */


    /* for ch in lexemes.input.iter() {
        if *ch == '\n'{
            println!("'\\n' - {}", lexer::Lexer::is_text(&ch));
        } else {
            println!("'{}' - {}", ch, lexer::Lexer::is_text(&ch));
        }
    } */
    // println!("{:?}", lexemes);
}
