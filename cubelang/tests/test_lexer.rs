use cubelang::lexer::{Lexer, Token};

#[test]
fn basic_test() {
    const CODE: &str = "() +-*/ []\n\t*";

    let expected = vec![
        Token::OpenParenthesis,
        Token::CloseParenthesis,
        Token::Plus,
        Token::Minus,
        Token::Asterisk,
        Token::ForwardSlash,
        Token::OpenBracket,
        Token::CloseBracket,
        Token::Asterisk,
        Token::Eof
    ];

    let tokens = Lexer::parse(CODE).tokens().to_vec();
    println!("{tokens:#?}");

    assert_eq!(expected, tokens);
}