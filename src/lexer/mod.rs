pub mod lexer;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
pub struct Lexer {
    pub input: Vec<char>,
    pub pos: usize,
    pub read_pos: usize,
    pub ch: char,
}


impl Lexer {
    #[allow(dead_code)] // TODO: remove the dead code labels 
    // later after deciding which function will be retained
    pub fn new(input: Vec<char>) -> Self {
        Self { 
            input,
            pos: 0, 
            read_pos: 0, 
            ch: '0'
        }
    }

    #[allow(dead_code)] // TODO: remove the dead code labels 
    // later after deciding which function will be retained
    pub fn new_u8(input: Vec<u8>) -> Self {
        Self { 
            input: input.iter()
            .map(|b| *b as char)
            .collect::<Vec<char>>(),
            pos: 0, 
            read_pos: 0, 
            ch: '0'
        }
    }

    // #[allow(dead_code)] // TODO: remove the dead code labels 
    // pub fn output_token() { }

    pub fn is_text(ch: &char) -> bool {
        lazy_static! {
            static ref RE:Regex = Regex::new(r"[0-9a-zA-Z]").unwrap();
        }
        let mut b = [0; 4];
        RE.is_match(ch.encode_utf8(&mut b))
    }

}
