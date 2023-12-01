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

#[test]
fn string_test() {
    const CODE: &str = "\"Hello world!\"'Hello world' '234234'";

    let expected = vec![
        Token::String("Hello world!".to_string()),
        Token::String("Hello world".to_string()),
        Token::String("234234".to_string()),
        Token::Eof
    ];

    let tokens = Lexer::parse(CODE).tokens().to_vec();
    println!("{tokens:#?}");

    assert_eq!(expected, tokens);
}

#[test]
fn num_test() {
    const CODE: &str = "1234.5 0.6 1234 -56890 .35 -123.4567";

    let expected = vec![
        Token::Number(1234.5),
        Token::Number(0.6),
        Token::Number(1234.0),
        Token::Number(-56890.0),
        Token::Number(0.35),
        Token::Number(-123.4567),
        Token::Eof
    ];

    let tokens = Lexer::parse(CODE).tokens().to_vec();
    println!("{tokens:#?}");

    assert_eq!(expected, tokens);
}