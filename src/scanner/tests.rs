use super::*;

#[test]
fn check_all_keywords_tokens_scan_correctly() {
    let mut s =
        Scanner::new("and class else if nil or print super var false for fun this true while");

    assert_eq!(s.scan_token().kind, TokenKind::And);
    assert_eq!(s.scan_token().kind, TokenKind::Class);
    assert_eq!(s.scan_token().kind, TokenKind::Else);
    assert_eq!(s.scan_token().kind, TokenKind::If);
    assert_eq!(s.scan_token().kind, TokenKind::Nil);
    assert_eq!(s.scan_token().kind, TokenKind::Or);
    assert_eq!(s.scan_token().kind, TokenKind::Print);
    assert_eq!(s.scan_token().kind, TokenKind::Super);
    assert_eq!(s.scan_token().kind, TokenKind::Var);
    assert_eq!(s.scan_token().kind, TokenKind::False);
    assert_eq!(s.scan_token().kind, TokenKind::For);
    assert_eq!(s.scan_token().kind, TokenKind::Fun);
    assert_eq!(s.scan_token().kind, TokenKind::This);
    assert_eq!(s.scan_token().kind, TokenKind::True);
    assert_eq!(s.scan_token().kind, TokenKind::While);
}

#[test]
fn check_numbers_tokens_scan_correctly() {
    let mut s = Scanner::new("0 1 234 0.10 1.14 .56");

    assert_eq!(s.scan_token().source, "0");
    assert_eq!(s.scan_token().source, "1");
    assert_eq!(s.scan_token().source, "234");
    assert_eq!(s.scan_token().source, "0.10");
    assert_eq!(s.scan_token().source, "1.14");
    assert_eq!(s.scan_token().source, ".56");
}

#[test]
fn check_string_tokens_scan_correctly() {
    let mut s = Scanner::new(r#" "Hello world" "This is an example of a string" "#);
    assert_eq!(s.scan_token().source, r#""Hello world""#);
    assert_eq!(s.scan_token().source, r#""This is an example of a string""#);
}

#[test]
fn check_identifiers_tokens_scan_correctly() {
    let mut s = Scanner::new("my_var _my_var example MY_NUMBER example123");

    assert_eq!(s.scan_token().source, "my_var");
    assert_eq!(s.scan_token().source, "_my_var");
    assert_eq!(s.scan_token().source, "example");
    assert_eq!(s.scan_token().source, "MY_NUMBER");
    assert_eq!(s.scan_token().source, "example123");
}

#[test]
fn check_complex_code_tokens() {
    let mut s = Scanner::new(
        "
        var a = 12.3;
        var b = 3.1416;
        
        print(a + b);
        print(\"Hello world from a test\");",
    );

    assert_eq!(s.scan_token().kind, TokenKind::Var);

    let mut token = s.scan_token();
    assert_eq!(token.kind, TokenKind::Identifier);
    assert_eq!(token.source, "a");

    assert_eq!(s.scan_token().kind, TokenKind::Equal);

    token = s.scan_token();
    assert_eq!(token.kind, TokenKind::Number);
    assert_eq!(token.source, "12.3");

    assert_eq!(s.scan_token().kind, TokenKind::Semicolon);

    assert_eq!(s.scan_token().kind, TokenKind::Var);

    token = s.scan_token();
    assert_eq!(token.kind, TokenKind::Identifier);
    assert_eq!(token.source, "b");

    assert_eq!(s.scan_token().kind, TokenKind::Equal);

    token = s.scan_token();
    assert_eq!(token.kind, TokenKind::Number);
    assert_eq!(token.source, "3.1416");

    assert_eq!(s.scan_token().kind, TokenKind::Semicolon);
    assert_eq!(s.scan_token().kind, TokenKind::Print);
    assert_eq!(s.scan_token().kind, TokenKind::LeftParen);

    token = s.scan_token();
    assert_eq!(token.kind, TokenKind::Identifier);
    assert_eq!(token.source, "a");

    assert_eq!(s.scan_token().kind, TokenKind::Plus);

    token = s.scan_token();
    assert_eq!(token.kind, TokenKind::Identifier);
    assert_eq!(token.source, "b");

    assert_eq!(s.scan_token().kind, TokenKind::RightParen);
    assert_eq!(s.scan_token().kind, TokenKind::Semicolon);

    assert_eq!(s.scan_token().kind, TokenKind::Print);
    assert_eq!(s.scan_token().kind, TokenKind::LeftParen);

    token = s.scan_token();
    assert_eq!(token.kind, TokenKind::String);
    assert_eq!(token.source, "\"Hello world from a test\"");

    assert_eq!(s.scan_token().kind, TokenKind::RightParen);
    assert_eq!(s.scan_token().kind, TokenKind::Semicolon);
    assert_eq!(s.scan_token().kind, TokenKind::EOF);
}

#[test]
fn check_whitespace_are_ignored() {
    let mut s = Scanner::new("// This is a comment. \n\t");
    assert_eq!(s.scan_token().kind, TokenKind::EOF);
}
