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
        self.skip_comments();
        self.skip_whitespace();

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
            (Some('.'), Some(digit)) if digit.is_ascii_digit() => return self.build_number_token(),
            (Some(digit), _) if digit.is_ascii_digit() => return self.build_number_token(),
            (Some('"'), _) => return self.build_string_token(),
            (Some(character), _) if character.is_alphabetic() || *character == '_' => self.make_identifier_or_keyword(),
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
        while let Some(character) = self.input.front() && (character.is_ascii_alphanumeric() || '_'.eq(character)) {
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
            "this" => Kind::This,
            "true" => Kind::True,
            _ => Kind::Identifier,
        };

        self.build_token(kind)
    }

    fn build_number_token(&mut self) -> Result<Token, LexicalError> {
        // Looks for digits for left side of floating point numbers or for integers.
        while let Some(digit) = self.input.front().filter(|character| character.is_ascii_digit()) {
            self.current_lexeme.push(*digit);
            self.input.pop_front();
        }

        if self.input.front() == Some(&'.') {
            // Consumes the floating point.
            self.input.pop_front();
            self.current_lexeme.push('.');

            // Return and error if there's not digits after the floating point.
            if self.input.front().is_none_or(|character| !character.is_ascii_digit()) {
                return Err(LexicalError::InvalidNumberFormat(self.line));
            }

            // Looks for digits for right side of floating point numbers.
            while let Some(digit) = self.input.front().filter(|character| character.is_ascii_digit()) {
                self.current_lexeme.push(*digit);
                self.input.pop_front();
            }
        }

        Ok(self.build_token(Kind::Number))
    }

    fn build_string_token(&mut self) -> Result<Token, LexicalError> {
        // Consumes the opening quote. The character has already been validated to as a quote.
        self.input.pop_front();

        while let Some(character) = self.input.front().filter(|character| **character != '"') {
            // Increases the line counter if the character is a newline.
            if *character == '\n' {
                self.line += 1;
            }

            // Consumes the character and adds it to the current lexeme.
            self.current_lexeme.push(*character);
            self.input.pop_front();
        }

        if self.input.front() == Some(&'"') {
            // Consumes the closing quote.
            self.input.pop_front();
            Ok(self.build_token(Kind::String))
        } else {
            Err(LexicalError::UnterminatedString(self.line))
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(character) = self.input.front().filter(|c| c.is_ascii_whitespace()) {
            if character == &'\n' {
                self.line += 1;
            }

            self.input.pop_front();
        }
    }

    /// Skips the comment lines
    fn skip_comments(&mut self) {
        // Look for the double slash match.
        while self.input.get(0) == Some(&'/') && self.input.get(1) == Some(&'/') {
            // Consumes the double slash.
            self.input.pop_front();
            self.input.pop_front();

            while let Some(character) = self.input.front()
                && '\n'.eq(character)
            {
                // If the current character is a newline, consume it, increment the line count,
                // and break out of the inner loop (end of the comment line).
                // if character.eq(&'\n') {
                self.input.pop_front();
                self.line += 1;
                // break;
                // }

                // Consumes the character of the comment.
                self.input.pop_front();
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.input.is_empty()
    }

    fn build_token(&mut self, kind: Kind) -> Token {
        let lexeme = match kind {
            // Takes the value of self.current_lexeme, assigns it to lexeme, and replace the attribute with its default value.
            Kind::Identifier | Kind::Number | Kind::String if !self.current_lexeme.is_empty() => Some(mem::take(&mut self.current_lexeme)),
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
