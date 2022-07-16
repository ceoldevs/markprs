use std::fs;

pub fn read(filepath: String) -> Vec<u8> {
    let contents = fs::read(filepath)
        .expect("Markdown file doesnt exist");

    contents
}
