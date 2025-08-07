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
