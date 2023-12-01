use std::iter::{Enumerate, Peekable};
use std::str::Chars;

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

impl Keyword {
    pub fn get(text: &str) -> Option<Self> {
        Some(match text {
            "end" => Self::End,
            "fn" => Self::Fn,
            "data" => Self::Data,
            "var" => Self::Var,
            "return" => Self::Return,
            "is" => Self::Is,
            "not" => Self::Not,
            "if" => Self::If,
            "elif" => Self::Elif,
            "else" => Self::Else,
            "none" => Self::None,
            "true" => Self::True,
            "false" => Self::False,
            _ => return None
        })
    }
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
                '-' => {
                    // Handle negative numbers - this also handles cases such as -.5, which makes me
                    // die inside but it's valid according to the spec so we need to handle it!
                    if let Some(peek) = enumerator.peek() {
                        if peek.1.is_ascii_digit() || peek.1 == '.' {
                            Self::parse_number(&mut enumerator, code, pos)
                        } else {
                            Token::Minus
                        }
                    } else {
                        Token::Minus
                    }
                },
                '*' => Token::Asterisk,
                '/' => Token::ForwardSlash,

                '>' => Token::GreaterThan,
                '<' => Token::LessThan,

                ',' => Token::Comma,

                '.' => {
                    // Sadly, cubelang can handle numbers that start with a decimal point, with no
                    // preceding number (for example, .5 or .123 as opposed to 0.5 or 0.123)
                    // This code checks to see if the next character is a digit, and if so,
                    // interprets it as a number.
                    if let Some(peek) = enumerator.peek() {
                        if peek.1.is_ascii_digit() {
                            Self::parse_number(&mut enumerator, code, pos)
                        } else {
                            Token::Period
                        }
                    } else {
                        Token::Period
                    }
                }

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
                '0'..='9' => {
                    Self::parse_number(&mut enumerator, code, pos)
                }

                chr => {
                    if !chr.is_alphanumeric() {
                        // TODO: Return a result, not panic.
                        panic!("Unexpected token '{chr}'.");
                    }

                    let mut tok_pos = 0;

                    loop {
                        let (pos, c) = match enumerator.peek() {
                            None => {
                                tok_pos += 1;
                                break;
                            },
                            Some(p) => *p
                        };

                        tok_pos = pos;

                        if !c.is_alphanumeric() {
                            break;
                        }

                        enumerator.next();
                    };

                    let identifier = &code[pos .. tok_pos];

                    match Keyword::get(identifier) {
                        None => Token::Identifier(identifier.to_string()),
                        Some(kw) => Token::Keyword(kw)
                    }
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

    fn parse_number(enumerator: &mut Peekable<Enumerate<Chars>>, code: &str, pos: usize) -> Token {
        let mut tok_pos = 0;

        loop {
            // Instead of calling next(), we call peek().
            // This is due to the way that our loops work.
            // The main loop ALWAYS calls next(), meaning that it will skip an entire character.
            // If there's a space, all good. If there isn't... then there are problems.
            // In this case, we ONLY call next() if we KNOW that the next digit is a number.
            let (pos, c) = match enumerator.peek() {
                // Handle EOF scenario, we must bump the token position by 1, as the range used
                // to get the number is exclusive on the upper bound.
                None => {
                    tok_pos += 1;
                    break;
                },
                Some(p) => *p
            };

            tok_pos = pos;

            if !c.is_ascii_digit() && c != '.' {
                break;
            }

            enumerator.next();
        };

        Token::Number(code[pos .. tok_pos].parse().unwrap())
    }
}