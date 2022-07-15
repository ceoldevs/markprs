// use std::env;
use std::fs;

/*
* 1. read the file
* 2. break the text into lexems
* 3. build the parse tree
* 4. build AST
* 5. convert to HTML
*/

/*

not sure how the tokens for headings should be

eg: 

heading `code`
---

we can go with [heading, `, code, `, ---]


*/

fn reader(filepath: String) -> String {
    let contents = fs::read_to_string(filepath)
        .expect("Markdown file doesnt exist");

    contents
}

fn lexer(contents: String) {
    for c in contents.chars() {
        if (c == "#")
    }
}

fn main() {
    let contents = reader("README.md".to_string());
    lexer(contents);
    // print!("{}", contents[0])
}
