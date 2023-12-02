use crate::lexer::Token;

pub fn parse(tokens: &Vec<Token>) {
    let mut enumerator = tokens.iter().enumerate().peekable();

    while let Some((pos, token)) = enumerator.next() {
        match token {
            Token::Identifier(sd) => {

            },

            _ => panic!("Syntax error.")
        }
    }
}