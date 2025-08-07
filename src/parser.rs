use crate::lexer::{Token, Keyword};
use crate::ast::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ParseError {}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }
    
    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let mut statements = Vec::new();
        
        while !self.is_at_end() {
            statements.push(self.statement()?);
        }
        
        Ok(Program { body: statements })
    }
    
    fn statement(&mut self) -> Result<Statement, ParseError> {
        match self.peek() {
            Token::Keyword(Keyword::Var) => self.var_declaration(),
            Token::Keyword(Keyword::Function) => self.function_declaration(),
            Token::Keyword(Keyword::If) => self.if_statement(),
            Token::Keyword(Keyword::While) => self.while_statement(),
            Token::Keyword(Keyword::For) => self.for_statement(),
            Token::Keyword(Keyword::Return) => self.return_statement(),
            Token::LeftBrace => self.block_statement(),
            _ => {
                let expr = self.expression()?;
                self.consume_semicolon();
                Ok(Statement::Expression(expr))
            }
        }
    }
    
    fn var_declaration(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // consume 'var'
        
        let name = match self.advance() {
            Token::Identifier(name) => name.clone(),
            _ => return Err(ParseError {
                message: "Expected identifier after 'var'".to_string(),
            }),
        };
        
        let init = if self.match_token(&Token::Assign) {
            Some(self.expression()?)
        } else {
            None
        };
        
        self.consume_semicolon();
        
        Ok(Statement::VarDeclaration(VarDeclaration { name, init }))
    }
    
    fn function_declaration(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // consume 'function'
        
        let name = match self.advance() {
            Token::Identifier(name) => name.clone(),
            _ => return Err(ParseError {
                message: "Expected function name".to_string(),
            }),
        };
        
        if !self.match_token(&Token::LeftParen) {
            return Err(ParseError {
                message: "Expected '(' after function name".to_string(),
            });
        }
        
        let mut params = Vec::new();
        if !self.check(&Token::RightParen) {
            loop {
                match self.advance() {
                    Token::Identifier(param) => params.push(param.clone()),
                    _ => return Err(ParseError {
                        message: "Expected parameter name".to_string(),
                    }),
                }
                
                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
        }
        
        if !self.match_token(&Token::RightParen) {
            return Err(ParseError {
                message: "Expected ')' after parameters".to_string(),
            });
        }
        
        if !self.match_token(&Token::LeftBrace) {
            return Err(ParseError {
                message: "Expected '{' before function body".to_string(),
            });
        }
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.statement()?);
        }
        
        if !self.match_token(&Token::RightBrace) {
            return Err(ParseError {
                message: "Expected '}' after function body".to_string(),
            });
        }
        
        Ok(Statement::FunctionDeclaration(FunctionDeclaration {
            name,
            params,
            body,
        }))
    }
    
    fn if_statement(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // consume 'if'
        
        if !self.match_token(&Token::LeftParen) {
            return Err(ParseError {
                message: "Expected '(' after 'if'".to_string(),
            });
        }
        
        let condition = self.expression()?;
        
        if !self.match_token(&Token::RightParen) {
            return Err(ParseError {
                message: "Expected ')' after if condition".to_string(),
            });
        }
        
        let then_branch = Box::new(self.statement()?);
        
        let else_branch = if self.match_token(&Token::Keyword(Keyword::Else)) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };
        
        Ok(Statement::IfStatement(IfStatement {
            condition,
            then_branch,
            else_branch,
        }))
    }
    
    fn while_statement(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // consume 'while'
        
        if !self.match_token(&Token::LeftParen) {
            return Err(ParseError {
                message: "Expected '(' after 'while'".to_string(),
            });
        }
        
        let condition = self.expression()?;
        
        if !self.match_token(&Token::RightParen) {
            return Err(ParseError {
                message: "Expected ')' after while condition".to_string(),
            });
        }
        
        let body = Box::new(self.statement()?);
        
        Ok(Statement::WhileStatement(WhileStatement { condition, body }))
    }
    
    fn for_statement(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // consume 'for'
        
        if !self.match_token(&Token::LeftParen) {
            return Err(ParseError {
                message: "Expected '(' after 'for'".to_string(),
            });
        }
        
        let init = if self.match_token(&Token::Semicolon) {
            None
        } else {
            Some(Box::new(self.statement()?))
        };
        
        let condition = if self.check(&Token::Semicolon) {
            None
        } else {
            Some(self.expression()?)
        };
        
        if !self.match_token(&Token::Semicolon) {
            return Err(ParseError {
                message: "Expected ';' after for loop condition".to_string(),
            });
        }
        
        let update = if self.check(&Token::RightParen) {
            None
        } else {
            Some(self.expression()?)
        };
        
        if !self.match_token(&Token::RightParen) {
            return Err(ParseError {
                message: "Expected ')' after for clauses".to_string(),
            });
        }
        
        let body = Box::new(self.statement()?);
        
        Ok(Statement::ForStatement(ForStatement {
            init,
            condition,
            update,
            body,
        }))
    }
    
    fn return_statement(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // consume 'return'
        
        let value = if self.check(&Token::Semicolon) || self.is_at_end() {
            None
        } else {
            Some(self.expression()?)
        };
        
        self.consume_semicolon();
        Ok(Statement::ReturnStatement(value))
    }
    
    fn block_statement(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // consume '{'
        
        let mut statements = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            statements.push(self.statement()?);
        }
        
        if !self.match_token(&Token::RightBrace) {
            return Err(ParseError {
                message: "Expected '}' after block".to_string(),
            });
        }
        
        Ok(Statement::Block(statements))
    }
    
    fn function_expression(&mut self) -> Result<Expression, ParseError> {
        self.advance(); // consume 'function'
        
        // Function expressions can be anonymous
        let name = match self.peek() {
            Token::Identifier(_) => {
                match self.advance() {
                    Token::Identifier(name) => Some(name.clone()),
                    _ => None,
                }
            }
            _ => None,
        };
        
        if !self.match_token(&Token::LeftParen) {
            return Err(ParseError {
                message: "Expected '(' after 'function'".to_string(),
            });
        }
        
        let mut params = Vec::new();
        if !self.check(&Token::RightParen) {
            loop {
                match self.advance() {
                    Token::Identifier(param) => params.push(param.clone()),
                    _ => return Err(ParseError {
                        message: "Expected parameter name".to_string(),
                    }),
                }
                
                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
        }
        
        if !self.match_token(&Token::RightParen) {
            return Err(ParseError {
                message: "Expected ')' after parameters".to_string(),
            });
        }
        
        if !self.match_token(&Token::LeftBrace) {
            return Err(ParseError {
                message: "Expected '{' before function body".to_string(),
            });
        }
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.statement()?);
        }
        
        if !self.match_token(&Token::RightBrace) {
            return Err(ParseError {
                message: "Expected '}' after function body".to_string(),
            });
        }
        
        Ok(Expression::Function(crate::ast::FunctionExpression {
            name,
            params,
            body,
        }))
    }
    
    fn expression(&mut self) -> Result<Expression, ParseError> {
        self.assignment()
    }
    
    fn assignment(&mut self) -> Result<Expression, ParseError> {
        let expr = self.logical_or()?;
        
        if self.match_token(&Token::Assign) {
            let right = self.assignment()?;
            return Ok(Expression::Assignment(Assignment {
                left: Box::new(expr),
                right: Box::new(right),
            }));
        }
        
        Ok(expr)
    }
    
    fn logical_or(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.logical_and()?;
        
        while self.match_token(&Token::LogicalOr) {
            let right = self.logical_and()?;
            expr = Expression::BinaryOp(BinaryOp {
                left: Box::new(expr),
                operator: BinaryOperator::LogicalOr,
                right: Box::new(right),
            });
        }
        
        Ok(expr)
    }
    
    fn logical_and(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.equality()?;
        
        while self.match_token(&Token::LogicalAnd) {
            let right = self.equality()?;
            expr = Expression::BinaryOp(BinaryOp {
                left: Box::new(expr),
                operator: BinaryOperator::LogicalAnd,
                right: Box::new(right),
            });
        }
        
        Ok(expr)
    }
    
    fn equality(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.comparison()?;
        
        while let Some(op) = self.match_equality_operator() {
            let right = self.comparison()?;
            expr = Expression::BinaryOp(BinaryOp {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            });
        }
        
        Ok(expr)
    }
    
    fn comparison(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.term()?;
        
        while let Some(op) = self.match_comparison_operator() {
            let right = self.term()?;
            expr = Expression::BinaryOp(BinaryOp {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            });
        }
        
        Ok(expr)
    }
    
    fn term(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.factor()?;
        
        while let Some(op) = self.match_term_operator() {
            let right = self.factor()?;
            expr = Expression::BinaryOp(BinaryOp {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            });
        }
        
        Ok(expr)
    }
    
    fn factor(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.unary()?;
        
        while let Some(op) = self.match_factor_operator() {
            let right = self.unary()?;
            expr = Expression::BinaryOp(BinaryOp {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            });
        }
        
        Ok(expr)
    }
    
    fn unary(&mut self) -> Result<Expression, ParseError> {
        if let Some(op) = self.match_unary_operator() {
            let right = self.unary()?;
            return Ok(Expression::UnaryOp(UnaryOp {
                operator: op,
                operand: Box::new(right),
            }));
        }
        
        self.call()
    }
    
    fn call(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.primary()?;
        
        loop {
            if self.match_token(&Token::LeftParen) {
                expr = self.finish_call(expr)?;
            } else if self.match_token(&Token::Dot) {
                let name = match self.advance() {
                    Token::Identifier(name) => name.clone(),
                    _ => return Err(ParseError {
                        message: "Expected property name after '.'".to_string(),
                    }),
                };
                expr = Expression::Member(Member {
                    object: Box::new(expr),
                    property: Box::new(Expression::Literal(Literal::String(name))),
                    computed: false,
                });
            } else if self.match_token(&Token::LeftBracket) {
                let index = self.expression()?;
                if !self.match_token(&Token::RightBracket) {
                    return Err(ParseError {
                        message: "Expected ']' after array index".to_string(),
                    });
                }
                expr = Expression::Member(Member {
                    object: Box::new(expr),
                    property: Box::new(index),
                    computed: true,
                });
            } else {
                break;
            }
        }
        
        Ok(expr)
    }
    
    fn finish_call(&mut self, callee: Expression) -> Result<Expression, ParseError> {
        let mut arguments = Vec::new();
        
        if !self.check(&Token::RightParen) {
            loop {
                arguments.push(self.expression()?);
                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
        }
        
        if !self.match_token(&Token::RightParen) {
            return Err(ParseError {
                message: "Expected ')' after arguments".to_string(),
            });
        }
        
        Ok(Expression::Call(Call {
            callee: Box::new(callee),
            arguments,
        }))
    }
    
    fn primary(&mut self) -> Result<Expression, ParseError> {
        match self.advance() {
            Token::Boolean(value) => Ok(Expression::Literal(Literal::Boolean(*value))),
            Token::Null => Ok(Expression::Literal(Literal::Null)),
            Token::Undefined => Ok(Expression::Literal(Literal::Undefined)),
            Token::Number(value) => Ok(Expression::Literal(Literal::Number(*value))),
            Token::String(value) => Ok(Expression::Literal(Literal::String(value.clone()))),
            Token::Identifier(name) => Ok(Expression::Identifier(name.clone())),
            Token::LeftParen => {
                let expr = self.expression()?;
                if !self.match_token(&Token::RightParen) {
                    return Err(ParseError {
                        message: "Expected ')' after expression".to_string(),
                    });
                }
                Ok(expr)
            }
            Token::LeftBracket => {
                let mut elements = Vec::new();
                
                if !self.check(&Token::RightBracket) {
                    loop {
                        elements.push(self.expression()?);
                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                    }
                }
                
                if !self.match_token(&Token::RightBracket) {
                    return Err(ParseError {
                        message: "Expected ']' after array elements".to_string(),
                    });
                }
                
                Ok(Expression::Array(elements))
            }
            Token::LeftBrace => {
                let mut properties = Vec::new();
                
                if !self.check(&Token::RightBrace) {
                    loop {
                        let key = match self.advance() {
                            Token::Identifier(name) => name.clone(),
                            Token::String(name) => name.clone(),
                            _ => return Err(ParseError {
                                message: "Expected property name".to_string(),
                            }),
                        };
                        
                        if !self.match_token(&Token::Colon) {
                            return Err(ParseError {
                                message: "Expected ':' after property name".to_string(),
                            });
                        }
                        
                        // Check if the value is a function expression
                        let value = if self.check(&Token::Keyword(Keyword::Function)) {
                            self.function_expression()?
                        } else {
                            self.expression()?
                        };
                        
                        properties.push((key, value));
                        
                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                    }
                }
                
                if !self.match_token(&Token::RightBrace) {
                    return Err(ParseError {
                        message: "Expected '}' after object properties".to_string(),
                    });
                }
                
                Ok(Expression::Object(properties))
            }
            _ => Err(ParseError {
                message: "Unexpected token".to_string(),
            }),
        }
    }
    
    // Helper methods
    fn match_equality_operator(&mut self) -> Option<BinaryOperator> {
        match self.peek() {
            Token::Equal => {
                self.advance();
                Some(BinaryOperator::Equal)
            }
            Token::NotEqual => {
                self.advance();
                Some(BinaryOperator::NotEqual)
            }
            Token::StrictEqual => {
                self.advance();
                Some(BinaryOperator::StrictEqual)
            }
            Token::StrictNotEqual => {
                self.advance();
                Some(BinaryOperator::StrictNotEqual)
            }
            _ => None,
        }
    }
    
    fn match_comparison_operator(&mut self) -> Option<BinaryOperator> {
        match self.peek() {
            Token::GreaterThan => {
                self.advance();
                Some(BinaryOperator::GreaterThan)
            }
            Token::GreaterThanOrEqual => {
                self.advance();
                Some(BinaryOperator::GreaterThanOrEqual)
            }
            Token::LessThan => {
                self.advance();
                Some(BinaryOperator::LessThan)
            }
            Token::LessThanOrEqual => {
                self.advance();
                Some(BinaryOperator::LessThanOrEqual)
            }
            _ => None,
        }
    }
    
    fn match_term_operator(&mut self) -> Option<BinaryOperator> {
        match self.peek() {
            Token::Minus => {
                self.advance();
                Some(BinaryOperator::Subtract)
            }
            Token::Plus => {
                self.advance();
                Some(BinaryOperator::Add)
            }
            _ => None,
        }
    }
    
    fn match_factor_operator(&mut self) -> Option<BinaryOperator> {
        match self.peek() {
            Token::Divide => {
                self.advance();
                Some(BinaryOperator::Divide)
            }
            Token::Multiply => {
                self.advance();
                Some(BinaryOperator::Multiply)
            }
            Token::Modulo => {
                self.advance();
                Some(BinaryOperator::Modulo)
            }
            _ => None,
        }
    }
    
    fn match_unary_operator(&mut self) -> Option<UnaryOperator> {
        match self.peek() {
            Token::Not => {
                self.advance();
                Some(UnaryOperator::Not)
            }
            Token::Minus => {
                self.advance();
                Some(UnaryOperator::Minus)
            }
            Token::Plus => {
                self.advance();
                Some(UnaryOperator::Plus)
            }
            Token::BitwiseNot => {
                self.advance();
                Some(UnaryOperator::BitwiseNot)
            }
            _ => None,
        }
    }
    
    fn match_token(&mut self, token: &Token) -> bool {
        if self.check(token) {
            self.advance();
            true
        } else {
            false
        }
    }
    
    fn check(&self, token: &Token) -> bool {
        if self.is_at_end() {
            false
        } else {
            std::mem::discriminant(self.peek()) == std::mem::discriminant(token)
        }
    }
    
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    
    fn is_at_end(&self) -> bool {
        self.peek() == &Token::EOF
    }
    
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
    
    fn consume_semicolon(&mut self) {
        self.match_token(&Token::Semicolon);
    }
}
