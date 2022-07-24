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

The current code is able to provide lexemes for headings
but is Spaghetti code done for prototyping. Below are
algorithms for handling the lexing.

### prev new line

This algorithm is used by those special characters which 
provide semantics when they are at starting line.
eg: Headings, Lists (ul and li), blockquotes.

### check and drop or choose
Once we encounter special symbols like `(, *, **,   `

### check until change

This algorithm is used for finding the text in the markdown
by finding the end of the end of the text until a symbol is
found.

For headings, we use the **prev new line** algorithm and then 
use **check until change**

**Possible cases**

```md
hello there `bold`\n -> TEXT(hello there ) CODE(`) TEXT(BOLD) CODE(`) EOL(\n)
hello there `bold` some\n -> TEXT(hello there ) CODE(`) TEXT(bold) CODE(`) TEXT( some) EOL(\n)
hello # there\n -> TEXT(hello # there) EOL (\n)
## hello there -> H2(##) TEXT(hello there)
```

1. once found text (h in this case), we start looping until
we find a set of special characters.

special characters are -> `\n, *, ! (`


## Bugs, Fixes and Todos

1. The current code is failing to indentify the ending new 
line character before the EOF.

2. The current logic for the lexer is complicated to 
identify the characters and meta characters.
