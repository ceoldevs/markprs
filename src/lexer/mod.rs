pub mod lexer;

// use std::process::exit;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
pub struct MetaData {
    pub prev_token: Option<lexer::TokenType>,
}

#[derive(Debug)]
pub struct Lexer {
    pub input: Vec<char>,
    pub pos: usize,
    pub read_pos: usize,
    pub meta_data: MetaData
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
            meta_data: MetaData { 
                prev_token: Option::None
            }
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
            meta_data: MetaData { 
                prev_token: Option::None
            }
        }
    }

    // #[allow(dead_code)] // TODO: remove the dead code labels 
    // pub fn output_token() { }

    fn is_text(ch: &char) -> bool {
        lazy_static! {
            static ref RE:Regex = Regex::new(r"[0-9a-zA-Z]").unwrap();
        }

        let mut b = [0; 4];
        RE.is_match(ch.encode_utf8(&mut b))
    }

    fn is_head(&self) -> bool {
        self.meta_data.prev_token == None || self.meta_data.prev_token == Some(lexer::TokenType::EOL)
    }

    // TODO: Working fine but not sure how to detect 
    // the end of symbols
    pub fn next(&mut self) -> lexer::Token {
        let token: lexer::Token = lexer::Token{
            t_type: lexer::TokenType::EOF,
            value: "EOF".to_string()
        };

        let len = self.input.len();
        if self.read_pos >= len {
            return token
        }

        // Headings parse
        /*
        * Need to check if the prev_token is TokenType::EOL or `None`
        * depending upon the count of hash, return the token type for headings
        */

        // self.read_pos
        //
        #[allow(unused_assignments)]
        let mut char = self.input[self.read_pos];

        if Lexer::is_text(&char) || (char == '#' && !Lexer::is_head(&self)){
            // print!("a");
            // TODO: handle edge cases of out of index
            // println!("a {}", char);
            // did this to say that any more # that come until we reach \n are not heading tags
            self.meta_data.prev_token = Some(lexer::TokenType::TEXT);
            self.read_pos += 1;
            char = self.input[self.read_pos];
            loop {
                // FIXME: Looping infinitely in this loop for some reason
                // println!("a {}", char);
                // TODO: need to identify header texts
                /* if !Lexer::is_head(&self, &char) {
                    if char == '\n' {
                        println!("{:?}", self.meta_data.prev_token);
                    }
                } */
                if Lexer::is_text(&char) || char == ' ' || (char == '#' && !Lexer::is_head(&self)){
                    self.read_pos += 1;
                    char = self.input[self.read_pos];
                } else {
                    // TODO: handle edge cases of out of index
                    let value = self.input[self.pos..self.read_pos] 
                                   .iter() 
                                   .collect::<String>();
                    self.pos = self.read_pos;
                    self.meta_data.prev_token = Some(lexer::TokenType::TEXT);
                    return lexer::Token{
                        t_type: lexer::TokenType::TEXT,
                        value
                    };
                }
            }
        } else {
            if char == '\n'{
                self.read_pos += 1;
                let value = self.input[self.pos..self.read_pos].iter().collect::<String>();
                self.pos = self.read_pos;
                // println!("{}, {}", len, self.read_pos);
                self.meta_data.prev_token = Some(lexer::TokenType::EOL);
                return lexer::Token{
                    t_type: lexer::TokenType::EOL,
                    value
                };
            } else if char == '#' && 
                (self.meta_data.prev_token == None || 
                self.meta_data.prev_token == Some(lexer::TokenType::EOL))
            {
                // print!("test");
                #[allow(unused_variables)]
                let mut count = 1;
                loop {
                    self.read_pos += 1;
                    char = self.input[self.read_pos];
                    if char == '#' {
                        count += 1;
                    } else {
                        break;
                    }
                }

                let t_type: lexer::TokenType;
                let value: String;

                // TODO: handle edge cases of out of index
                self.read_pos += 1;
                // println!("{}", self.read_pos);
                value = self.input[self.pos..self.read_pos].iter().collect::<String>();
                self.pos = self.read_pos;
                println!("-{}-", self.input[self.read_pos]
                );
                
                match count {
                    1 => t_type = lexer::TokenType::H1,
                    2 => t_type = lexer::TokenType::H2,
                    3 => t_type = lexer::TokenType::H3,
                    4 => t_type = lexer::TokenType::H4,
                    5 => t_type = lexer::TokenType::H5,
                    6 => t_type = lexer::TokenType::H6,
                    _ => t_type = lexer::TokenType::TEXT
                }

                self.meta_data.prev_token = Some(lexer::TokenType::H1);
                return lexer::Token{
                    t_type,
                    value,
                };

                /* if self.meta_data.prev_token == None {
                } else {
                } */
            }
        }

        token
    }

    /* fn nontext(&mut self, len: usize) -> lexer::Token {
        let t_type;
        let value;
        let mut ch: char;
        if len > self.read_pos {
            ch = self.input[self.read_pos];
            if ch == '#' {
                let mut count = 1;
                loop {
                    self.read_pos+=1;
                    ch = self.input[self.read_pos];
                    if ch != '#'{
                        break;
                    }
                    count+=1;
                }

                if count == 6 {
                    t_type = lexer::TokenType::H6;
                }
                else if count == 5 {
                    t_type = lexer::TokenType::H5;
                } else if count == 4 {
                    t_type = lexer::TokenType::H4;
                } else if count == 3 {
                    t_type = lexer::TokenType::H3;
                } else if count == 2 {
                    t_type = lexer::TokenType::H2;
                } else {
                    t_type = lexer::TokenType::H1;
                }
                value = self.input[self.pos..self.read_pos]
                        .to_vec()
                        .iter()
                        .collect::<String>();
                // println!("{}", ch);
            } else if ch == '*' {
                t_type = lexer::TokenType::UL;
                value = self.input[self.pos..self.read_pos+1]
                        .to_vec()
                        .iter()
                        .collect::<String>();
            } else if ch == '\n' {
                t_type = lexer::TokenType::EOL;
                value = self.input[self.pos..self.read_pos+1]
                        .to_vec()
                        .iter()
                        .collect::<String>();
            }else {
                t_type = lexer::TokenType::CODE;
                value = self.input[self.pos..self.read_pos+1]
                        .to_vec()
                        .iter()
                        .collect::<String>();
            }


        } else {
            t_type = lexer::TokenType::EOF; 
            value = "END OF FILE".to_string();
        }

        return lexer::Token { t_type, value }
    }
 */
}
