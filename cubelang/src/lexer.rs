#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Keyword(Keyword),
    Number(f64),
    String(String),

    OpenParenthesis,
    CloseParenthesis,

    OpenBracket,
    CloseBracket,

    Plus,
    Minus,
    Asterisk,
    ForwardSlash,

    GreaterThan,
    LessThan,

    Comma,

    Period,
    Colon,

    Eof
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Keyword {
    End,
    Fn,
    Data,
    Var,
    Return,
    Is,
    Not,
    If,
    Elif,
    Else,
    None,
    True,
    False
}

pub struct Lexer {
    tokens: Vec<Token>
}

impl Lexer {
    pub fn parse(code: &str) -> Self {
        let chars = code.chars();

        let mut tokens = Vec::new();

        let mut enumerator = chars.enumerate().peekable();

        while let Some((pos, c)) = enumerator.next() {
            let token = match c {
                '(' => Token::OpenParenthesis,
                ')' => Token::CloseParenthesis,

                '[' => Token::OpenBracket,
                ']' => Token::CloseBracket,

                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Asterisk,
                '/' => Token::ForwardSlash,

                '>' => Token::GreaterThan,
                '<' => Token::LessThan,

                ',' => Token::Comma,

                '.' => Token::Period,
                ':' => Token::Colon,

                '\n' | '\r' | '\t' | ' ' => continue,

                // Handle strings.
                // Cube supports both " and ' strings.
                // TODO: String formatting, " strings should allow for %VAR% formats.
                '"' | '\'' => {
                    let mut tok_pos = 0;

                    while let Some((string_pos, string_c)) = enumerator.next() {
                        if string_c == c {
                            tok_pos = string_pos;

                            break;
                        }
                    }

                    Token::String(code[pos + 1 .. tok_pos].to_string())
                },

                // Handle numbers.
                // TODO: Negative numbers.
                '0'..='9' => {
                    let mut tok_pos = 0;

                    while let Some((num_pos, num_c)) = enumerator.next() {
                        tok_pos = num_pos;

                        if (num_c < '0' || num_c > '9') && num_c != '.' {
                            break;
                        }
                    }

                    println!("{}", &code[pos .. tok_pos]);

                    Token::Number(code[pos .. tok_pos].parse().unwrap())
                }

                chr => {
                    continue;
                }
            };

            tokens.push(token);
        }

        tokens.push(Token::Eof);

        Self {
            tokens
        }
    }

    pub fn tokens(&self) -> &Vec<Token> {
        &self.tokens
    }
}