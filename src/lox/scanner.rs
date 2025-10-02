use super::token::Kind;
use super::token::Token;
use crate::error::LexicalError;
use std::collections::VecDeque;
use std::mem;

#[derive(Debug)]
pub struct Scanner {
    input: VecDeque<char>,
    current_lexeme: String,
    line: u32,
}

impl Iterator for Scanner {
    type Item = Result<Token, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        // Skips trivia.
        // Trivia refers to characters that don't contribute to the final program, such as comments
        // and whitespaces.
        self.skip_whitespace();
        self.skip_comments();

        if self.is_at_end() {
            return None;
        }

        return Some(self.next_token());
    }
}

impl Scanner {
    /// Creates a new instance of the Scanner struct.
    pub fn new(input: String) -> Self {
        Self {
            input: input.chars().collect(),
            current_lexeme: String::new(),
            line: 1,
        }
    }

    fn next_token(&mut self) -> Result<Token, LexicalError> {
        let token = match (self.input.get(0), self.input.get(1)) {
            // Special cases
            // (Some('.'), Some(digit)) if digit.is_ascii_digit() => return self.build_number_token(),
            (Some(digit), _) if digit.is_ascii_digit() => return self.build_number_token(),
            (Some('"'), _) => return self.build_string_token(),
            (Some(ch), _) if ch.is_alphabetic() || *ch == '_' => self.make_identifier_or_keyword(),
            // Single character
            (Some('('), _) => self.build_token(Kind::LeftParen),
            (Some(')'), _) => self.build_token(Kind::RightParen),
            (Some('{'), _) => self.build_token(Kind::LeftBrace),
            (Some('}'), _) => self.build_token(Kind::RightBrace),
            (Some(';'), _) => self.build_token(Kind::Semicolon),
            (Some(','), _) => self.build_token(Kind::Comma),
            (Some('.'), _) => self.build_token(Kind::Dot),
            (Some('-'), _) => self.build_token(Kind::Minus),
            (Some('+'), _) => self.build_token(Kind::Plus),
            (Some('/'), _) => self.build_token(Kind::Slash),
            (Some('*'), _) => self.build_token(Kind::Star),
            // Two characters match
            (Some('!'), Some('=')) => self.build_token(Kind::BangEqual),
            (Some('!'), _) => self.build_token(Kind::Bang),
            (Some('='), Some('=')) => self.build_token(Kind::EqualEqual),
            (Some('='), _) => self.build_token(Kind::Equal),
            (Some('<'), Some('=')) => self.build_token(Kind::LessEqual),
            (Some('<'), _) => self.build_token(Kind::Less),
            (Some('>'), Some('=')) => self.build_token(Kind::GreaterEqual),
            (Some('>'), _) => self.build_token(Kind::Greater),
            _ => return Err(LexicalError::UnexpectedCharacter(self.line)),
        };

        Ok(token)
    }

    fn make_identifier_or_keyword(&mut self) -> Token {
        while let Some(ch) = self.input.front()
            && (ch.is_ascii_alphanumeric() || *ch == '_')
        {
            self.current_lexeme.push(self.input.pop_front().unwrap());
        }

        let kind = match self.current_lexeme.as_str() {
            "and" => Kind::And,
            "class" => Kind::Class,
            "else" => Kind::Else,
            "if" => Kind::If,
            "nil" => Kind::Nil,
            "or" => Kind::Or,
            "print" => Kind::Print,
            "super" => Kind::Super,
            "var" => Kind::Var,
            "while" => Kind::While,
            "false" => Kind::False,
            "for" => Kind::For,
            "fun" => Kind::Fun,
            "return" => Kind::Return,
            "this" => Kind::This,
            "true" => Kind::True,
            _ => Kind::Identifier,
        };

        self.build_token(kind)
    }

    fn build_number_token(&mut self) -> Result<Token, LexicalError> {
        // Looks for digits for left side of floating point numbers or for integers.
        while let Some(ch) = self.input.front()
            && ch.is_ascii_digit()
        {
            self.current_lexeme.push(self.input.pop_front().unwrap());
        }

        if self.input.get(0).is_some_and(|ch| *ch == '.') && self.input.get(1).is_some_and(|ch| ch.is_ascii_digit()) {
            // Consumes the floating point.
            self.current_lexeme.push(self.input.pop_front().unwrap());

            // Looks for digits for right side of floating point numbers.
            while let Some(ch) = self.input.front()
                && ch.is_ascii_digit()
            {
                self.current_lexeme.push(self.input.pop_front().unwrap());
            }
        }

        Ok(self.build_token(Kind::Number))
    }

    fn build_string_token(&mut self) -> Result<Token, LexicalError> {
        // Consumes the opening quote. The character has already been validated to as a quote.
        self.input.pop_front();

        while let Some(ch) = self.input.front()
            && *ch != '"'
        {
            // Increases the line counter if the character is a newline.
            if *ch == '\n' {
                self.line += 1;
            }

            // Consumes the character and adds it to the current lexeme.
            self.current_lexeme.push(*ch);
            self.input.pop_front();
        }

        if self.input.front().is_some_and(|ch| '"' == *ch) {
            // Consumes the closing quote.
            self.input.pop_front();
            Ok(self.build_token(Kind::String))
        } else {
            Err(LexicalError::UnterminatedString(self.line))
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.input.front()
            && ch.is_ascii_whitespace()
        {
            if *ch == '\n' {
                self.line += 1;
            }

            self.input.pop_front();
        }
    }

    /// Skips the comment lines
    fn skip_comments(&mut self) {
        // Look for the double slash match.
        while matches!((self.input.get(0), self.input.get(1)), (Some('/'), Some('/'))) {
            // Consumes the double slash.
            self.input.pop_front();
            self.input.pop_front();

            while let Some(ch) = self.input.front()
                && *ch != '\n'
            {
                self.input.pop_front();
            }

            self.skip_whitespace();
        }
    }

    fn is_at_end(&self) -> bool {
        self.input.is_empty()
    }

    fn build_token(&mut self, kind: Kind) -> Token {
        let lexeme = match kind {
            // Takes the value of self.current_lexeme, assigns it to lexeme, and replace the attribute with its default value.
            Kind::Identifier | Kind::Number | Kind::String if !self.current_lexeme.is_empty() => {
                Some(mem::take(&mut self.current_lexeme))
            }
            // Keywords
            Kind::And
            | Kind::Class
            | Kind::Else
            | Kind::If
            | Kind::Nil
            | Kind::Or
            | Kind::Print
            | Kind::Super
            | Kind::Var
            | Kind::While
            | Kind::False
            | Kind::For
            | Kind::Fun
            | Kind::Return
            | Kind::This
            | Kind::True => {
                // Clean the current lexeme value, it might have keywords saved.
                self.current_lexeme.clear();
                None
            }
            // Verification for two character tokens. It must pop another character.
            Kind::BangEqual | Kind::EqualEqual | Kind::LessEqual | Kind::GreaterEqual => {
                // Consumes the two-character operator.
                self.input.pop_front();
                self.input.pop_front();
                None
            }
            // One-character tokens
            _ => {
                // Consumes the character.
                self.input.pop_front();
                None
            }
        };

        Token::new(kind, self.line, lexeme)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_all_keywords_tokens_scan_correctly() {
        let input = "and class else if nil or print super var false for fun this true while return";
        let mut scanner = Scanner::new(input.to_string());

        let expected_kinds = vec![
            Kind::And,
            Kind::Class,
            Kind::Else,
            Kind::If,
            Kind::Nil,
            Kind::Or,
            Kind::Print,
            Kind::Super,
            Kind::Var,
            Kind::False,
            Kind::For,
            Kind::Fun,
            Kind::This,
            Kind::True,
            Kind::While,
            Kind::Return,
        ];

        for kind in expected_kinds {
            let token = scanner
                .next()
                .expect("Scanner returns None.")
                .expect("Scanner return an LexicalError.");

            assert_eq!(token.kind, kind);
        }

        assert!(scanner.next().is_none(), "Scanner still scanning for tokens.");
    }

    #[test]
    fn check_numbers_tokens_scan_correctly() {
        let input = "0 234 0.10";
        let mut scanner = Scanner::new(input.to_string());

        let expected_lexemes = vec!["0".to_string(), "234".to_string(), "0.10".to_string()];

        for lexeme in expected_lexemes {
            let token = scanner.next().expect("Scanner returns None.");
            let token = token.expect("Scanner return an LexicalError.");

            assert_eq!(token.lexeme.expect("Expected a lexeme."), lexeme);
            assert_eq!(token.kind, Kind::Number)
        }

        assert!(scanner.next().is_none(), "Scanner still scanning for tokens.");
    }

    #[test]
    fn check_numbers_parse_ignoring_partial_decimal_point() {
        let input = "1. .1";
        let mut scanner = Scanner::new(input.to_string());

        assert!(scanner.next().unwrap().is_ok_and(|token| token.kind == Kind::Number));
        assert!(scanner.next().unwrap().is_ok_and(|token| token.kind == Kind::Dot));

        assert!(scanner.next().unwrap().is_ok_and(|token| token.kind == Kind::Dot));
        assert!(scanner.next().unwrap().is_ok_and(|token| token.kind == Kind::Number));

        assert!(scanner.next().is_none(), "Scanner still scanning for tokens.");
    }

    #[test]
    fn check_string_tokens_scan_correctly() {
        let input = r#" "Hello world" "This is an example of a string" "#;
        let mut scanner = Scanner::new(input.to_string());

        let token = scanner
            .next()
            .expect("Expected some token")
            .expect("Expected an valid token.");

        assert!(token.lexeme.is_some_and(|lexeme| lexeme.eq("Hello world")));
        assert_eq!(token.kind, Kind::String);

        let token = scanner
            .next()
            .expect("Expected some token")
            .expect("Expected an valid token.");
        assert!(token.lexeme.is_some_and(|lexeme| lexeme.eq("This is an example of a string")));
        assert_eq!(token.kind, Kind::String);

        assert!(scanner.next().is_none(), "Scanner still scanning for tokens.");
    }

    #[test]
    fn check_identifiers_tokens_scan_correctly() {
        let input = "my_var _my_var example MY_NUMBER example123 _";
        let mut scanner = Scanner::new(input.to_string());

        let expexted_lexemes = vec!["my_var", "_my_var", "example", "MY_NUMBER", "example123", "_"];

        for expected_lexeme in expexted_lexemes {
            let token = scanner.next().expect("Expected a token").expect("Expected a valid token");
            assert_eq!(token.kind, Kind::Identifier);
            assert!(token.lexeme.is_some_and(|token_lexeme| token_lexeme.eq(&expected_lexeme)));
        }

        assert!(scanner.next().is_none(), "Scanner still scanning for tokens.");
    }

    #[test]
    fn check_whitespace_and_comments_are_ignored() {
        let input = "// This is a comment.\n\t//Another comment.//another\n";
        let mut scanner = Scanner::new(input.to_string());
        assert!(scanner.next().is_none(), "Scanner still scanning for tokens.");
    }

    #[test]
    fn check_complex_code_tokens() {
        let input = "
        var a = 12.3;
        var b = 3.1416;
        // This is a comment and should be ignored.
        print(a + b);
        print(\"Hello world from a test\");";
        let mut scanner = Scanner::new(input.to_string());

        let expected_kinds = [
            Kind::Var,
            Kind::Identifier,
            Kind::Equal,
            Kind::Number,
            Kind::Semicolon,
            Kind::Var,
            Kind::Identifier,
            Kind::Equal,
            Kind::Number,
            Kind::Semicolon,
            Kind::Print,
            Kind::LeftParen,
            Kind::Identifier,
            Kind::Plus,
            Kind::Identifier,
            Kind::RightParen,
            Kind::Semicolon,
            Kind::Print,
            Kind::LeftParen,
            Kind::String,
            Kind::RightParen,
            Kind::Semicolon,
        ];

        for kind in expected_kinds {
            let token = scanner
                .next()
                .expect("Scanner returns None.")
                .expect("Scanner return an LexicalError.");

            assert_eq!(token.kind, kind);
        }

        assert!(scanner.next().is_none(), "Scanner still scanning for tokens.");
    }
}
