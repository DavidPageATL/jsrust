#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Undefined,
    
    // Identifiers and keywords
    Identifier(String),
    Keyword(Keyword),
    
    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Assign,
    Equal,
    NotEqual,
    StrictEqual,
    StrictNotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LogicalAnd,
    LogicalOr,
    Not,
    BitwiseNot,
    
    // Punctuation
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Semicolon,
    Comma,
    Dot,
    Colon,
    
    // Special
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Var,
    Let,
    Const,
    Function,
    If,
    Else,
    While,
    For,
    Return,
    True,
    False,
    Null,
    Undefined,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current_char = chars.get(0).copied();
        
        Lexer {
            input: chars,
            position: 0,
            current_char,
        }
    }
    
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        
        while let Some(token) = self.next_token() {
            if token == Token::EOF {
                break;
            }
            tokens.push(token);
        }
        
        tokens.push(Token::EOF);
        tokens
    }
    
    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        
        match self.current_char {
            None => Some(Token::EOF),
            Some(ch) => {
                match ch {
                    // Single character tokens
                    '+' => {
                        self.advance();
                        Some(Token::Plus)
                    }
                    '-' => {
                        self.advance();
                        Some(Token::Minus)
                    }
                    '*' => {
                        self.advance();
                        Some(Token::Multiply)
                    }
                    '/' => {
                        self.advance();
                        // Check for comments
                        if self.current_char == Some('/') {
                            self.skip_line_comment();
                            return self.next_token();
                        } else if self.current_char == Some('*') {
                            self.skip_block_comment();
                            return self.next_token();
                        }
                        Some(Token::Divide)
                    }
                    '%' => {
                        self.advance();
                        Some(Token::Modulo)
                    }
                    '(' => {
                        self.advance();
                        Some(Token::LeftParen)
                    }
                    ')' => {
                        self.advance();
                        Some(Token::RightParen)
                    }
                    '{' => {
                        self.advance();
                        Some(Token::LeftBrace)
                    }
                    '}' => {
                        self.advance();
                        Some(Token::RightBrace)
                    }
                    '[' => {
                        self.advance();
                        Some(Token::LeftBracket)
                    }
                    ']' => {
                        self.advance();
                        Some(Token::RightBracket)
                    }
                    ';' => {
                        self.advance();
                        Some(Token::Semicolon)
                    }
                    ',' => {
                        self.advance();
                        Some(Token::Comma)
                    }
                    '.' => {
                        self.advance();
                        Some(Token::Dot)
                    }
                    ':' => {
                        self.advance();
                        Some(Token::Colon)
                    }
                    '~' => {
                        self.advance();
                        Some(Token::BitwiseNot)
                    }
                    
                    // Multi-character tokens
                    '=' => {
                        self.advance();
                        if self.current_char == Some('=') {
                            self.advance();
                            if self.current_char == Some('=') {
                                self.advance();
                                Some(Token::StrictEqual)
                            } else {
                                Some(Token::Equal)
                            }
                        } else {
                            Some(Token::Assign)
                        }
                    }
                    '!' => {
                        self.advance();
                        if self.current_char == Some('=') {
                            self.advance();
                            if self.current_char == Some('=') {
                                self.advance();
                                Some(Token::StrictNotEqual)
                            } else {
                                Some(Token::NotEqual)
                            }
                        } else {
                            Some(Token::Not)
                        }
                    }
                    '<' => {
                        self.advance();
                        if self.current_char == Some('=') {
                            self.advance();
                            Some(Token::LessThanOrEqual)
                        } else {
                            Some(Token::LessThan)
                        }
                    }
                    '>' => {
                        self.advance();
                        if self.current_char == Some('=') {
                            self.advance();
                            Some(Token::GreaterThanOrEqual)
                        } else {
                            Some(Token::GreaterThan)
                        }
                    }
                    '&' => {
                        self.advance();
                        if self.current_char == Some('&') {
                            self.advance();
                            Some(Token::LogicalAnd)
                        } else {
                            // Single & is not handled for simplicity
                            self.next_token()
                        }
                    }
                    '|' => {
                        self.advance();
                        if self.current_char == Some('|') {
                            self.advance();
                            Some(Token::LogicalOr)
                        } else {
                            // Single | is not handled for simplicity
                            self.next_token()
                        }
                    }
                    
                    // String literals
                    '"' | '\'' => {
                        Some(self.read_string())
                    }
                    
                    // Numbers
                    '0'..='9' => {
                        Some(self.read_number())
                    }
                    
                    // Identifiers and keywords
                    'a'..='z' | 'A'..='Z' | '_' | '$' => {
                        Some(self.read_identifier())
                    }
                    
                    _ => {
                        self.advance();
                        self.next_token() // Skip unknown characters
                    }
                }
            }
        }
    }
    
    fn advance(&mut self) {
        self.position += 1;
        self.current_char = self.input.get(self.position).copied();
    }
    
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
    
    fn skip_line_comment(&mut self) {
        while let Some(ch) = self.current_char {
            if ch == '\n' {
                break;
            }
            self.advance();
        }
    }
    
    fn skip_block_comment(&mut self) {
        self.advance(); // Skip the '*'
        
        while let Some(ch) = self.current_char {
            if ch == '*' {
                self.advance();
                if self.current_char == Some('/') {
                    self.advance();
                    break;
                }
            } else {
                self.advance();
            }
        }
    }
    
    fn read_string(&mut self) -> Token {
        let quote_char = self.current_char.unwrap();
        self.advance(); // Skip opening quote
        
        let mut value = String::new();
        
        while let Some(ch) = self.current_char {
            if ch == quote_char {
                self.advance(); // Skip closing quote
                break;
            } else if ch == '\\' {
                self.advance();
                if let Some(escaped) = self.current_char {
                    match escaped {
                        'n' => value.push('\n'),
                        't' => value.push('\t'),
                        'r' => value.push('\r'),
                        '\\' => value.push('\\'),
                        '\'' => value.push('\''),
                        '"' => value.push('"'),
                        _ => {
                            value.push('\\');
                            value.push(escaped);
                        }
                    }
                    self.advance();
                }
            } else {
                value.push(ch);
                self.advance();
            }
        }
        
        Token::String(value)
    }
    
    fn read_number(&mut self) -> Token {
        let mut value = String::new();
        
        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() || ch == '.' {
                value.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        let number = value.parse::<f64>().unwrap_or(0.0);
        Token::Number(number)
    }
    
    fn read_identifier(&mut self) -> Token {
        let mut value = String::new();
        
        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' || ch == '$' {
                value.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        // Check if it's a keyword
        match value.as_str() {
            "var" => Token::Keyword(Keyword::Var),
            "let" => Token::Keyword(Keyword::Let),
            "const" => Token::Keyword(Keyword::Const),
            "function" => Token::Keyword(Keyword::Function),
            "if" => Token::Keyword(Keyword::If),
            "else" => Token::Keyword(Keyword::Else),
            "while" => Token::Keyword(Keyword::While),
            "for" => Token::Keyword(Keyword::For),
            "return" => Token::Keyword(Keyword::Return),
            "true" => Token::Boolean(true),
            "false" => Token::Boolean(false),
            "null" => Token::Null,
            "undefined" => Token::Undefined,
            _ => Token::Identifier(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizes_numbers_strings_keywords_and_operators() {
        let mut lexer = Lexer::new("var answer = 42 + \"hi\"; if (answer >= 40 && answer !== 41) { return true; }");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Keyword(Keyword::Var));
        assert_eq!(tokens[1], Token::Identifier("answer".to_string()));
        assert_eq!(tokens[2], Token::Assign);
        assert_eq!(tokens[3], Token::Number(42.0));
        assert_eq!(tokens[4], Token::Plus);
        assert_eq!(tokens[5], Token::String("hi".to_string()));
        assert_eq!(tokens[6], Token::Semicolon);
        assert_eq!(tokens[7], Token::Keyword(Keyword::If));
        assert_eq!(tokens[8], Token::LeftParen);
        assert_eq!(tokens[9], Token::Identifier("answer".to_string()));
        assert_eq!(tokens[10], Token::GreaterThanOrEqual);
        assert_eq!(tokens[11], Token::Number(40.0));
        assert_eq!(tokens[12], Token::LogicalAnd);
        assert_eq!(tokens[13], Token::Identifier("answer".to_string()));
        assert_eq!(tokens[14], Token::StrictNotEqual);
        assert_eq!(tokens[15], Token::Number(41.0));
        assert_eq!(tokens[16], Token::RightParen);
        assert_eq!(tokens[17], Token::LeftBrace);
        assert_eq!(tokens[18], Token::Keyword(Keyword::Return));
        assert_eq!(tokens[19], Token::Boolean(true));
        assert_eq!(tokens[20], Token::Semicolon);
        assert_eq!(tokens[21], Token::RightBrace);
        assert_eq!(tokens[22], Token::EOF);
    }

    #[test]
    fn skips_comments_and_handles_multichar_tokens() {
        let mut lexer = Lexer::new("x === y || z != 3 // comment\n/* block */\n20 <= 21");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Identifier("x".to_string()));
        assert_eq!(tokens[1], Token::StrictEqual);
        assert_eq!(tokens[2], Token::Identifier("y".to_string()));
        assert_eq!(tokens[3], Token::LogicalOr);
        assert_eq!(tokens[4], Token::Identifier("z".to_string()));
        assert_eq!(tokens[5], Token::NotEqual);
        assert_eq!(tokens[6], Token::Number(3.0));
        assert_eq!(tokens[7], Token::Number(20.0));
        assert_eq!(tokens[8], Token::LessThanOrEqual);
        assert_eq!(tokens[9], Token::Number(21.0));
        assert_eq!(tokens[10], Token::EOF);
    }
}
