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

    fn new_ln(&mut self) -> lexer::Token {
        self.read_pos += 1;
        let value = self.input[self.pos..self.read_pos].iter().collect::<String>();
        self.pos = self.read_pos;
        // println!("{}, {}", len, self.read_pos);
        self.meta_data.prev_token = Some(lexer::TokenType::EOL);
        return lexer::Token{
            t_type: lexer::TokenType::EOL,
            value
        };
    }

    // Headings parse
    /*
    * Need to check if the prev_token is TokenType::EOL or `None`
    * depending upon the count of hash, return the token type for headings
    */
    fn find_head(&mut self) -> lexer::Token {
        let mut count = 1;
        let mut ch: char;
        // let mut true_head = false;
        loop {
            self.read_pos += 1;
            ch = self.input[self.read_pos];
            if ch == '#' {
                count += 1;
            } else {
                break;
            }
        }

        let t_type: lexer::TokenType;
        let value: String;

        // If we have space or new line just after hashes, 
        // then only it is considered as heading. When 
        // parsing, if we encounter EOL, we take the text 
        // as empty strings.""
        if ch != ' ' && ch != '\n' {
            self.read_pos = self.pos;
            if self.is_head() {
                self.meta_data.prev_token = Some(lexer::TokenType::TEXT);
            }
            return self.find_text();
        }
        
        match count {
            1 => t_type = lexer::TokenType::H1,
            2 => t_type = lexer::TokenType::H2,
            3 => t_type = lexer::TokenType::H3,
            4 => t_type = lexer::TokenType::H4,
            5 => t_type = lexer::TokenType::H5,
            6 => t_type = lexer::TokenType::H6,
            _ => {
                // if # count more than 6, change the prev token to Text
                // and rerun the logic by which the control flow enters 
                // the Text handling phase
                self.read_pos = self.pos;
                if self.is_head() {
                    self.meta_data.prev_token = Some(lexer::TokenType::TEXT);
                }
                // println!("{:?}", self.meta_data.prev_token);
                return self.find_text();
            }
        }

        // TODO: handle edge cases of out of index
        // println!("{}", self.read_pos);
        self.read_pos += 1;
        value = self.input[self.pos..self.read_pos].iter().collect::<String>();
        self.pos = self.read_pos;

        self.meta_data.prev_token = Some(lexer::TokenType::H1);
        return lexer::Token{
            t_type,
            value,
        };
    }

    fn find_text(&mut self) -> lexer::Token {
        // TODO: handle edge cases of out of index
        self.meta_data.prev_token = Some(lexer::TokenType::TEXT);
        self.read_pos += 1;
        let mut ch = self.input[self.read_pos];
        loop {
            if Lexer::is_text(&ch) || ch == ' ' || (ch == '#' && !Lexer::is_head(&self)){
                self.read_pos += 1;
                ch = self.input[self.read_pos];
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
    }

    pub fn next(&mut self) -> lexer::Token {
        let token: lexer::Token = lexer::Token{
            t_type: lexer::TokenType::EOF,
            value: "EOF".to_string()
        };

        let len = self.input.len();
        if self.read_pos >= len {
            return token
        }

        let char = self.input[self.read_pos];

        // println!("{}, {:?}", char, self.meta_data.prev_token);

        if char == '\n'{
            return self.new_ln();
        } 

        if char == '#' && self.is_head() {
            return self.find_head();
        }

        if Lexer::is_text(&char) || char == ' ' || char == '#' && !self.is_head() {
            return self.find_text();
        } 

        token
    }
}
