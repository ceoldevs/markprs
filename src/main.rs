mod lexer;
mod reader;

/*
* 1. read the file
* 2. break the text into lexems
* 3. build the parse tree
* 4. build AST
* 5. convert to HTML
*/

fn main() {
    let contents = reader::read("tests/headings.md".to_string());

    let lexemes = lexer::Lexer::new_u8(contents);
    for ch in lexemes.input.iter() {
        if *ch == '\n'{
            println!("'\\n' - {}", lexer::Lexer::is_text(&ch));
        } else {
            println!("'{}' - {}", ch, lexer::Lexer::is_text(&ch));
        }
    }
    // println!("{:?}", lexemes);
}
