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

    let tokens = Lexer::parse(CODE).unwrap().tokens().to_vec();
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

    let tokens = Lexer::parse(CODE).unwrap().tokens().to_vec();
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

    let tokens = Lexer::parse(CODE).unwrap().tokens().to_vec();
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

    let tokens = Lexer::parse(CODE).unwrap().tokens().to_vec();
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

    let tokens = Lexer::parse(CODE).unwrap().tokens().to_vec();
    println!("{tokens:#?}");

    assert_eq!(expected, tokens);
}

#[test]
fn invalid_test() {
    const CODE: &str = "hello there &";

    let result = Lexer::parse(CODE);
    assert!(result.is_err());
}

#[test]
fn full_test() {
    const CODE: &str = "
fn main()
    x is 5 + 4.4 - -.67
    y is 10
    z is add(x, y)

    # Reset time to 0.
    Time:setTime(0)

    if z is 15
        log('Yeah!')
    elif z is not 10.5
        log(\"Huh?\")
    else
        prompt('???')
    end

    return false
end";

    let expected = vec![
        // fn main()
        Token::Keyword(Keyword::Fn),
        Token::Identifier("main".to_string()),
        Token::OpenParenthesis,
        Token::CloseParenthesis,

        // x is 5 + 4.4 - -.67
        Token::Identifier("x".to_string()),
        Token::Keyword(Keyword::Is),
        Token::Number(5.0),
        Token::Plus,
        Token::Number(4.4),
        Token::Minus,
        Token::Number(-0.67),

        // y is 10
        Token::Identifier("y".to_string()),
        Token::Keyword(Keyword::Is),
        Token::Number(10.0),

        // z is add(x, y)
        Token::Identifier("z".to_string()),
        Token::Keyword(Keyword::Is),
        Token::Identifier("add".to_string()),
        Token::OpenParenthesis,
        Token::Identifier("x".to_string()),
        Token::Comma,
        Token::Identifier("y".to_string()),
        Token::CloseParenthesis,

        // Time:setTime(0)
        Token::Identifier("Time".to_string()),
        Token::Colon,
        Token::Identifier("setTime".to_string()),
        Token::OpenParenthesis,
        Token::Number(0.0),
        Token::CloseParenthesis,

        // if z is 15
        Token::Keyword(Keyword::If),
        Token::Identifier("z".to_string()),
        Token::Keyword(Keyword::Is),
        Token::Number(15.0),

        // log('Yeah!')
        Token::Identifier("log".to_string()),
        Token::OpenParenthesis,
        Token::String("Yeah!".to_string()),
        Token::CloseParenthesis,

        // elif z is not 10.5
        Token::Keyword(Keyword::Elif),
        Token::Identifier("z".to_string()),
        Token::Keyword(Keyword::Is),
        Token::Keyword(Keyword::Not),
        Token::Number(10.5),

        // log("Huh?")
        Token::Identifier("log".to_string()),
        Token::OpenParenthesis,
        Token::String("Huh?".to_string()),
        Token::CloseParenthesis,

        // else
        Token::Keyword(Keyword::Else),

        // prompt('???')
        Token::Identifier("prompt".to_string()),
        Token::OpenParenthesis,
        Token::String("???".to_string()),
        Token::CloseParenthesis,

        // end
        Token::Keyword(Keyword::End),

        // return false
        Token::Keyword(Keyword::Return),
        Token::Keyword(Keyword::False),

        // end
        Token::Keyword(Keyword::End),

        Token::Eof
    ];

    let tokens = Lexer::parse(CODE).unwrap().tokens().to_vec();
    println!("{tokens:#?}");

    assert_eq!(expected, tokens);
}