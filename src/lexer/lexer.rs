#[derive(Debug)]
#[allow(dead_code)]
#[derive(PartialEq)]
pub enum TokenType {
    H6,
    H5,
    H4,
    H3,
    H2,
    H1,
    PRECODE,
    CODE,
    EXCLAIM,
    OPENPARAN,
    CLOSEPARAN,
    OPENSQUARE,
    CLOSESQUARE,
    DOUBLEUNDERSCORE,
    UNDERSCORE,
    DOUBLEASTERISK,
    ASTERISK,
    /* PARAGRAPH, */
    TEXT,
    UL,
    OL,
    EOL,
    EOF,
}

#[derive(Debug)]
pub struct Token {
    pub t_type: TokenType,
    pub value: String,
}

/* 
pub fn assign_tkn_type(value: &Vec<char>) -> Result<TokenType, String> {
    let lexeme: String = value.into_iter().collect::<String>();

    match &lexeme[..] {
        "######" => Ok(TokenType::H6),
        "#####" => Ok(TokenType::H5),
        "####" => Ok(TokenType::H4),
        "###" => Ok(TokenType::H3),
        "##" => Ok(TokenType::H2),
        "#" => Ok(TokenType::H1),
        "```" => Ok(TokenType::PRECODE),
        "`" => Ok(TokenType::CODE),
        "!" => Ok(TokenType::EXCLAIM),
        "(" => Ok(TokenType::OPENPARAN),
        ")" => Ok(TokenType::CLOSEPARAN),
        "[" => Ok(TokenType::OPENSQUARE),
        "]" => Ok(TokenType::CLOSESQUARE),
        "__" => Ok(TokenType::DOUBLEUNDERSCORE),
        "_" => Ok(TokenType::UNDERSCORE),
        "**" => Ok(TokenType::DOUBLEASTERISK),
        "*" => Ok(TokenType::ASTERISK),
        _ => Err(String::from("Not a keyword")),
    }
}

*/
// string iterations: https://gist.github.com/jimmychu0807/9a89355e642afad0d2aeda52e6ad2424
