use cubelang::lexer::{Keyword, Lexer, Token};

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

#[test]
fn math_test() {
    const CODE: &str = "(12 * 3) - 4 + 2.46 / (3 - -34.2437 * -.5)";

    let expected = vec![
        Token::OpenParenthesis,
        Token::Number(12.0),
        Token::Asterisk,
        Token::Number(3.0),
        Token::CloseParenthesis,
        Token::Minus,
        Token::Number(4.0),
        Token::Plus,
        Token::Number(2.46),
        Token::ForwardSlash,
        Token::OpenParenthesis,
        Token::Number(3.0),
        Token::Minus,
        Token::Number(-34.2437),
        Token::Asterisk,
        Token::Number(-0.5),
        Token::CloseParenthesis,
        Token::Eof
    ];

    let tokens = Lexer::parse(CODE).tokens().to_vec();
    println!("{tokens:#?}");

    assert_eq!(expected, tokens);
}

#[test]
fn keyword_test() {
    const CODE: &str = "if else none None data return 'sdfsdf' sdfsdf";

    let expected = vec![
        Token::Keyword(Keyword::If),
        Token::Keyword(Keyword::Else),
        Token::Keyword(Keyword::None),
        Token::Identifier("None".to_string()),
        Token::Keyword(Keyword::Data),
        Token::Keyword(Keyword::Return),
        Token::String("sdfsdf".to_string()),
        Token::Identifier("sdfsdf".to_string()),
        Token::Eof
    ];

    let tokens = Lexer::parse(CODE).tokens().to_vec();
    println!("{tokens:#?}");

    assert_eq!(expected, tokens);
}