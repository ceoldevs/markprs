# Dev Docs for MarkDown Lexer 

> The project is still in alpha and will undergo massive 
> changes moving forward

## Initializing the Lexer

The current idea of the lexer is to first take the input
of the code by initializing the instance of Lexer with the
`Lexer::new_u8` method offered by `lexer` module.

Since the `fs::read` method outputs type of `Vec<u8>`, 
`Lexer::new_u8` enables to pass the `fs::read` output
directly.

## `Lexer::next` method

This method is used to get the next token from the markdown
saved inside the `Lexer`.

> Under Development

The current code is able to 

## Bugs, Fixes and Todos

1. The current code is failing to indentify the ending new 
line character before the EOF.

2. The current logic for the lexer is complicated to 
identify the characters and meta characters.
