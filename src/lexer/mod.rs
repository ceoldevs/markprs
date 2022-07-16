pub mod lexer;

// use std::process::exit;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
pub struct Lexer {
    pub input: Vec<char>,
    pub pos: usize,
    pub read_pos: usize,
}

impl Lexer {
    #[allow(dead_code)] // TODO: remove the dead code labels 
    // later after deciding which function will be retained
    pub fn new(input: Vec<char>) -> Self {
        if input.len() == 0 {
            panic!("[Error] Input Empty");
        }
        Self { 
            input,
            pos: 0, 
            read_pos: 0, 
        }
    }

    #[allow(dead_code)] // TODO: remove the dead code labels 
    // later after deciding which function will be retained
    pub fn new_u8(input: Vec<u8>) -> Self {
        if input.len() == 0 {
panic!("[Error] Input Empty");
        }
        Self { 
            input: input.iter()
            .map(|b| *b as char)
            .collect::<Vec<char>>(),
            pos: 0, 
            read_pos: 0, 
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

    fn nontext(&mut self, len: usize) -> lexer::Token {
        let t_type;
        let value;
        if len > self.read_pos {
            t_type = lexer::TokenType::CODE;
            value = self.input[self.pos..self.read_pos+1]
                    .to_vec()
                    .iter()
                    .collect::<String>();
        } else {
            t_type = lexer::TokenType::EOF; 
            value = "END OF FILE".to_string();
        }

        return lexer::Token { t_type, value }
    }

    // TODO: Working fine but not sure how to detect 
    // the end of symbols
    pub fn next(&mut self) -> lexer::Token {
        let len = self.input.len();

        let token: lexer::Token;

        if len > self.read_pos {
            if Lexer::is_text(&self.input[self.read_pos]) {
                loop {
                    self.read_pos += 1;
                    if len > self.read_pos {
                        if !(Lexer::is_text(&self.input[self.read_pos]) || self.input[self.read_pos] == ' ') {
                            break token = lexer::Token {
                                t_type: lexer::TokenType::TEXT,
                                value: self.input[self.pos..self.read_pos]
                                        .to_vec()
                                        .iter()
                                        .collect::<String>(),
                            };
                        }
                    } else {
                        break token = lexer::Token {
                            t_type: lexer::TokenType::EOF,
                            value: "END OF LINE".to_string(),
                        };
                    }
                }
                self.pos = self.read_pos;
            } else {
                token = self.nontext(len);
                self.read_pos += 1;
                self.pos = self.read_pos;
            }
        } else {
            token = lexer::Token {
                t_type: lexer::TokenType::EOF,
                value: "END OF LINE".to_string(),
            };
        }

        token

    }

/* pub fn next(&mut self) -> lexer::Token {
        if self.input.len() == 0 {
            print!("No file");
            exit(1); // TODO: change to better error statement
        } 

        if self.read_pos == 0 {
            self.ch = self.input[self.read_pos];
            // self.read_pos += 1;
        }

        let count = self.input.len();
        loop {
            self.read_pos += 1;
            if Lexer::is_text(&self.ch) {
                let token = lexer::Token {
                    t_type:  lexer::TokenType::TEXT, 
                    value: self.input[self.pos..self.read_pos].to_vec().iter().collect::<String>()
                };
                self.pos = self.read_pos;
                if count > self.read_pos {
                    self.ch = self.input[self.read_pos];
                } else {
                    self.ch = '\0';
                    return lexer::Token {
                        t_type:  lexer::TokenType::EOF, 
                        value: "No more comments found" .to_string()
                    };
                }
                return token;
            } else {
                let token = lexer::Token {
                    t_type:  lexer::TokenType::H1, 
                    value: self.input[self.pos..self.read_pos].to_vec().iter().collect::<String>()
                };
                self.pos = self.read_pos;
                if count > self.read_pos {

                    self.ch = self.input[self.read_pos];
                } else {
                    self.ch = '\0';
                    return lexer::Token {
                        t_type:  lexer::TokenType::EOF, 
                        value: "No more comments found"
                            .to_string()
                    };
                }
                return token;
                // self.ch = self.input[self.read_pos];
            }
            /* if Lexer::is_text(&self.ch) {
                self.read_pos += 1;
                self.ch = self.input[self.pos];
            } else {
                // println!("{}, {}, {}", self.pos, self.read_pos, self.input.len());
                self.read_pos += 1;
                let token = lexer::Token {
                    t_type:  lexer::TokenType::H6, 
                    value: self.input[self.pos..self.read_pos].to_vec()
                };
                self.ch = self.input[self.pos];
                self.pos = self.read_pos;
                return token;
            } */
        }
    } */

}
