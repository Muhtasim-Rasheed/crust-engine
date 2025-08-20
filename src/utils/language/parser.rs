use crate::utils::{Token, TokenType, Value};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Value(Value),
    List(Vec<Expression>),
    Object(HashMap<String, Expression>),
    Closure {
        args: Vec<String>,
        body: Vec<Statement>,
        returns: Box<Expression>,
    },
    MemberAccess {
        object: Box<Expression>,
        key: Box<Expression>,
    },
    Identifier(String),
    PostIncrement(String),
    PostDecrement(String),
    PreIncrement(String),
    PreDecrement(String),
    Binary {
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    },
    Unary {
        operator: String,
        operand: Box<Expression>,
    },
    Call {
        function: Box<Expression>,
        args: Vec<Expression>,
    },
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Value(v) => write!(f, "VAL[{}]", v.to_string()),
            Expression::List(l) => {
                let list_str = l
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "[{}]", list_str)
            }
            Expression::Object(o) => {
                let obj_str = o
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_string()))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "{{ {} }}", obj_str)
            }
            Expression::Closure { args, returns, .. } => {
                let args_str = args.join(", ");
                write!(f, "({}) {} {{ ... }}", args_str, returns.to_string())
            }
            Expression::MemberAccess { object, key } => {
                write!(f, "{}[{}]", object.to_string(), key.to_string())
            }
            Expression::Identifier(id) => write!(f, "ID[{}]", id),
            Expression::PostIncrement(id) => write!(f, "{}++", id),
            Expression::PostDecrement(id) => write!(f, "{}--", id),
            Expression::PreIncrement(id) => write!(f, "++{}", id),
            Expression::PreDecrement(id) => write!(f, "--{}", id),
            Expression::Binary { left, operator, right } => {
                write!(f, "({} {} {})", left.to_string(), operator, right.to_string())
            }
            Expression::Unary { operator, operand } => {
                write!(f, "({}{})", operator, operand.to_string())
            }
            Expression::Call { function, args } => {
                let args_str = args
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "{}({})", function, args_str)
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Statement {
    Assignment {
        is_global: bool,
        identifier: Expression,
        value: Expression,
    },
    Nop,
    Assert {
        condition: Expression,
    },
    Match {
        value: Expression,
        cases: Vec<(Expression, Vec<Statement>)>,
        default: Option<Vec<Statement>>,
    },
    If {
        condition: Expression,
        body: Vec<Statement>,
        else_if_bodies: Vec<(Expression, Vec<Statement>)>,
        else_body: Option<Vec<Statement>>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    For {
        identifier: String,
        iterable: Expression,
        body: Vec<Statement>,
    },
    Setup {
        body: Vec<Statement>,
    },
    Update {
        body: Vec<Statement>,
    },
    CloneSetup {
        body: Vec<Statement>,
    },
    CloneUpdate {
        body: Vec<Statement>,
    },
    WhenBroadcasted {
        broadcast: String,
        body: Vec<Statement>,
    },
    WhenBoolean {
        condition: Expression,
        body: Vec<Statement>,
    },
    Import {
        path: String,
    },
    Call(Expression),
    FunctionDefinition {
        name: String,
        args: Vec<String>,
        body: Vec<Statement>,
        returns: Expression,
    },
}

impl std::fmt::Debug for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Assignment {
                is_global,
                identifier,
                value,
            } => write!(
                f,
                "ASSIGN[{}{:?} = {}]",
                if *is_global { "global " } else { "" },
                identifier,
                value.to_string()
            ),
            Statement::Nop => write!(f, "NOP"),
            Statement::Assert { condition } => write!(f, "ASSERT[{}]", condition.to_string()),
            Statement::Match {
                value,
                cases,
                default,
            } => {
                let cases_str = cases
                    .iter()
                    .map(|(case_value, body)| {
                        format!("CASE[{}] {{ {:?} }}", case_value.to_string(), body)
                    })
                    .collect::<Vec<_>>()
                    .join(" ");
                let default_str = if let Some(default_body) = default {
                    format!("DEFAULT {{ {:?} }}", default_body)
                } else {
                    "".to_string()
                };
                write!(
                    f,
                    "MATCH[{}] {{ {} }} {}",
                    value.to_string(),
                    cases_str,
                    default_str
                )
            }
            Statement::If {
                condition,
                body,
                else_if_bodies,
                else_body,
            } => {
                let else_if_str = else_if_bodies
                    .iter()
                    .map(|(cond, body)| format!("ELSE_IF[{}] {{ {:?} }}", cond.to_string(), body))
                    .collect::<Vec<_>>()
                    .join(" ");
                let else_str = if let Some(else_body) = else_body {
                    format!("ELSE {{ {:?} }}", else_body)
                } else {
                    "".to_string()
                };
                write!(
                    f,
                    "IF[{}] {{ {:?} }} {} {}",
                    condition.to_string(),
                    body,
                    else_if_str,
                    else_str
                )
            }
            Statement::While { condition, body } => {
                write!(f, "WHILE[{}] {{ {:?} }}", condition.to_string(), body)
            }
            Statement::For {
                identifier,
                iterable,
                body,
            } => write!(
                f,
                "FOR[{} IN {}] {{ {:?} }}",
                identifier,
                iterable.to_string(),
                body
            ),
            Statement::Setup { body } => write!(f, "SETUP {{ {:?} }}", body),
            Statement::Update { body } => write!(f, "UPDATE {{ {:?} }}", body),
            Statement::CloneSetup { body } => write!(f, "CLONE_SETUP {{ {:?} }}", body),
            Statement::CloneUpdate { body } => write!(f, "CLONE_UPDATE {{ {:?} }}", body),
            Statement::WhenBroadcasted { broadcast, body } => {
                write!(f, "WHEN_BROADCASTED[{}] {{ {:?} }}", broadcast, body)
            }
            Statement::WhenBoolean { condition, body } => write!(
                f,
                "WHEN_BOOLEAN[{}] {{ {:?} }}",
                condition.to_string(),
                body
            ),
            Statement::Import { path } => write!(f, "IMPORT[{}]", path),
            Statement::Call(expr) => write!(f, "CALL[{}]", expr.to_string()),
            Statement::FunctionDefinition {
                name,
                args,
                body,
                returns,
            } => write!(
                f,
                "FUNCTION[{}({:?}) -> {}] {{ {:?} }}",
                name,
                args,
                returns.to_string(),
                body
            ),
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn peek(&self) -> &Token {
        if self.current < self.tokens.len() {
            &self.tokens[self.current]
        } else {
            &self.tokens.last().unwrap()
        }
    }

    fn advance(&mut self) {
        if self.current < self.tokens.len() {
            self.current += 1;
        }
    }

    fn eat(&mut self, token: &TokenType) -> bool {
        if self.peek().token_type == *token {
            self.advance();
            return true;
        }
        false
    }

    fn eat_any(&mut self, tokens: &[TokenType]) -> Option<TokenType> {
        for token in tokens {
            if self.eat(token) {
                return Some(token.clone());
            }
        }
        None
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = vec![];

        let mut errors = vec![];

        while self.peek().token_type != TokenType::EOF {
            if self.eat(&TokenType::Newline) {
                continue;
            }
            let statement = self.parse_statement().unwrap_or_else(|e| {
                // Just skip it ¯\_(ツ)_/¯ (also show the error)
                errors.push(e.replace("\n", "\\n"));
                self.advance();
                Statement::Nop
            });
            statements.push(statement);
        }

        if errors.len() > 0 {
            if errors.len() == 1 {
                eprintln!("There was a parsing error. Details are given below:");
                eprintln!("\t=> {}", errors[0]);
            } else {
                eprintln!(
                    "There were {} parsing errors. Details are given below:",
                    errors.len()
                );
                for (i, error) in errors.into_iter().enumerate() {
                    eprintln!("\t#{} => {}", i + 1, error);
                }
            }
            eprintln!("");
        }

        statements
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.peek().token_type {
            TokenType::Keyword(ref k) if k == "nop" => {
                self.advance();
                Ok(Statement::Nop)
            }
            TokenType::Keyword(ref k) if k == "assert" => {
                self.advance();
                let condition = self.parse_binary(0)?;
                Ok(Statement::Assert { condition })
            }
            TokenType::Keyword(ref k) if k == "match" => self.parse_match(),
            TokenType::Keyword(ref k) if k == "if" => self.parse_if(),
            TokenType::Keyword(ref k) if k == "while" => self.parse_while(),
            TokenType::Keyword(ref k) if k == "for" => self.parse_for(),
            TokenType::Keyword(ref k) if k == "setup" => {
                self.advance();
                let body = self.parse_block()?;
                Ok(Statement::Setup { body })
            }
            TokenType::Keyword(ref k) if k == "update" => {
                self.advance();
                let body = self.parse_block()?;
                Ok(Statement::Update { body })
            }
            TokenType::Keyword(ref k) if k == "clone_setup" => {
                self.advance();
                let body = self.parse_block()?;
                Ok(Statement::CloneSetup { body })
            }
            TokenType::Keyword(ref k) if k == "clone_update" => {
                self.advance();
                let body = self.parse_block()?;
                Ok(Statement::CloneUpdate { body })
            }
            TokenType::Keyword(ref k) if k == "when" => {
                self.advance();
                if let TokenType::Value(Value::String(broadcast)) = self.peek().clone().token_type {
                    self.advance();
                    let body = self.parse_block()?;
                    Ok(Statement::WhenBroadcasted {
                        broadcast: broadcast.clone(),
                        body,
                    })
                } else {
                    self.advance();
                    let condition = self.parse_binary(0)?;
                    let body = self.parse_block()?;
                    Ok(Statement::WhenBoolean { condition, body })
                }
            }
            TokenType::Keyword(ref k) if k == "fn" => self.parse_function_definition(),
            TokenType::Keyword(ref k) if k == "import" => self.parse_import(),
            TokenType::Keyword(ref k) if k == "global" => self.parse_global_assignment(),
            TokenType::Identifier(_) => self.parse_assignment_or_call(),
            _ => Err(format!(
                "Unexpected token: {:?} at {}:{}",
                self.peek().token_type, self.peek().line, self.peek().column
            )),
        }
    }

    fn parse_block(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = vec![];

        if !self.eat(&TokenType::Symbol("{".to_string())) {
            return Err(format!(
                "Expected '{{' at the start of block at {}:{}",
                self.peek().line, self.peek().column
            ));
        }

        while self.peek().token_type != TokenType::Symbol("}".to_string()) {
            if self.eat(&TokenType::Newline) {
                continue;
            }
            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        if !self.eat(&TokenType::Symbol("}".to_string())) {
            return Err(format!(
                "Expected '}}' at the end of block at {}:{}",
                self.peek().line, self.peek().column
            ));
        }

        Ok(statements)
    }

    fn precedence(op: &str) -> u8 {
        match op {
            "++" | "--" => 9,
            "**" => 8,
            "^" | "&" | "|" | "<<" | ">>" => 7,
            "*" | "/" | "%" => 6,
            "+" | "-" => 5,
            "==" | "!=" | "<" | ">" | "<=" | ">=" => 4,
            "!" => 3,
            "&&" | "||" => 2,
            "=" => 1,
            _ => 0,
        }
    }

    fn parse_binary(&mut self, min_prec: u8) -> Result<Expression, String> {
        let mut left = self.parse_primary()?;

        loop {
            let token = self.peek().clone();
            let op = match token.token_type {
                TokenType::Operator(op) => op,
                TokenType::Keyword(k) if k == "in" => k,
                _ => break,
            };

            let prec = Parser::precedence(op.as_str());
            if prec < min_prec {
                break;
            }

            if op == "++" || op == "--" {
                // Handle post-increment and post-decrement
                self.advance();
                if let Expression::Identifier(name) = left {
                    if op == "++" {
                        left = Expression::PostIncrement(name);
                    } else {
                        left = Expression::PostDecrement(name);
                    }
                } else {
                    return Err(format!(
                        "Expected identifier for post-{} but got {:?} at {}:{}",
                        if op == "++" { "increment" } else { "decrement" },
                        left, self.peek().line, self.peek().column
                    ));
                }
                continue;
            }

            let op = op.clone();
            self.advance();

            let right = self.parse_binary(prec + 1)?;

            left = Expression::Binary {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_list(&mut self) -> Result<Expression, String> {
        let mut list = vec![];
        while self.peek().token_type != TokenType::Symbol("]".to_string()) {
            if self.eat(&TokenType::Newline) {
                continue;
            }
            let expr = self.parse_binary(0)?;
            list.push(expr);
            if !self.eat(&TokenType::Symbol(",".to_string())) {
                break;
            }
        }

        self.eat(&TokenType::Newline);

        if !self.eat(&TokenType::Symbol("]".to_string())) {
            return Err(format!(
                "Expected ']' at the end of list at {}:{}",
                self.peek().line, self.peek().column
            ));
        }

        Ok(Expression::List(list))
    }

    fn parse_object(&mut self) -> Result<Expression, String> {
        let mut object = HashMap::new();
        while self.peek().token_type != TokenType::Symbol("}".to_string()) {
            if self.eat(&TokenType::Newline) {
                continue;
            }
            let peeked = self.peek().clone();
            if let TokenType::Identifier(key) | TokenType::Value(Value::String(key)) = peeked.token_type {
                self.advance();
                match self.peek().token_type {
                    TokenType::Symbol(ref s) if s == ":" => {
                        self.advance();
                        let value = self.parse_binary(0)?;
                        object.insert(key, value);
                        if !self.eat(&TokenType::Symbol(",".to_string())) {
                            break;
                        }
                    }
                    TokenType::Symbol(ref s) if s == "," => {
                        self.advance();
                        object.insert(key.clone(), Expression::Identifier(key));
                    }
                    _ => {
                        object.insert(key.clone(), Expression::Identifier(key));
                    }
                }
            } else {
                return Err(format!(
                    "Expected identifier or string as key in object but got {:?} at {}:{}",
                    peeked, self.peek().line, self.peek().column
                ));
            }
        }

        self.eat(&TokenType::Newline);

        if !self.eat(&TokenType::Symbol("}".to_string())) {
            return Err(format!(
                "Expected '}}' at the end of object at {}:{}",
                self.peek().line, self.peek().column
            ));
        }

        Ok(Expression::Object(object))
    }

    fn parse_closure(&mut self) -> Result<Expression, String> {
        if !self.eat(&TokenType::Symbol("(".to_string())) {
            return Err(format!(
                "Expected '(' after 'fn' at {}:{}",
                self.peek().line, self.peek().column
            ));
        }
        let mut args = vec![];
        while self.peek().token_type != TokenType::Symbol(")".to_string()) {
            if self.eat(&TokenType::Newline) {
                continue;
            }
            if let TokenType::Identifier(ref arg) = self.peek().token_type {
                args.push(arg.clone());
                self.advance();
            } else {
                return Err(format!(
                    "Expected identifier in closure arguments but got {:?} at {}:{}",
                    self.peek().token_type, self.peek().line, self.peek().column
                ));
            }
            if !self.eat(&TokenType::Symbol(",".to_string())) {
                break;
            }
        }
        self.eat(&TokenType::Newline);
        if !self.eat(&TokenType::Symbol(")".to_string())) {
            return Err(format!(
                "Expected ')' after closure arguments at {}:{}",
                self.peek().line, self.peek().column
            ));
        }
        let returns = self.parse_binary(0)?;
        let body = self.parse_block()?;
        Ok(Expression::Closure {
            args,
            body,
            returns: Box::new(returns),
        })
    }

    fn parse_function_call(&mut self, base: Expression) -> Result<Expression, String> {
        let mut args = vec![];
        while self.peek().token_type != TokenType::Symbol(")".to_string()) {
            if self.eat(&TokenType::Newline) {
                continue;
            }
            let arg = self.parse_binary(0)?;
            args.push(arg);
            if !self.eat(&TokenType::Symbol(",".to_string())) {
                break;
            }
        }
        if !self.eat(&TokenType::Symbol(")".to_string())) {
            return Err(format!(
                "Expected ')' after function call at {}:{}",
                self.peek().line, self.peek().column
            ));
        }
        Ok(Expression::Call {
            function: Box::new(base),
            args,
        })
    }

    fn parse_bracket_access(&mut self, base: Expression) -> Result<Expression, String> {
        let key = self.parse_binary(0)?;
        if !self.eat(&TokenType::Symbol("]".to_string())) {
            return Err(format!(
                "Expected ']' after list member access at {}:{}",
                self.peek().line, self.peek().column
            ));
        }
        Ok(Expression::MemberAccess {
            object: Box::new(base),
            key: Box::new(key),
        })
    }

    fn parse_dot_access(&mut self, base: Expression) -> Result<Expression, String> {
        let key = self.parse_primary()?;
        match key {
            Expression::Identifier(key_name) => Ok(Expression::MemberAccess {
                object: Box::new(base),
                key: Box::new(Expression::Value(Value::String(key_name))),
            }),
            Expression::Value(Value::Number(num)) => Ok(Expression::MemberAccess {
                object: Box::new(base),
                key: Box::new(Expression::Value(Value::Number(num))),
            }),
            _ => Err(format!(
                "Expected identifier or number after '.' but got {:?} at {}:{}",
                key, self.peek().line, self.peek().column
            )),
        }
    }

    fn parse_identifier_expr(&mut self, name: String) -> Result<Expression, String> {
        let mut expr = Expression::Identifier(name);

        loop {
            if self.eat(&TokenType::Symbol("(".to_string())) {
                expr = self.parse_function_call(expr)?;
            } else if self.eat(&TokenType::Symbol("[".to_string())) {
                expr = self.parse_bracket_access(expr)?;
            } else if self.eat(&TokenType::Symbol(".".to_string())) {
                expr = self.parse_dot_access(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn parse_pre_incdec(&mut self, op: &str) -> Result<Expression, String> {
        if let TokenType::Identifier(id) = self.peek().clone().token_type {
            self.advance();
            if op == "++" {
                Ok(Expression::PreIncrement(id.clone()))
            } else {
                Ok(Expression::PreDecrement(id.clone()))
            }
        } else {
            Err(format!(
                "Expected identifier after '{}' but got {:?} at {}:{}",
                op, self.peek().token_type, self.peek().line, self.peek().column
            ))
        }
    }

    fn parse_primary(&mut self) -> Result<Expression, String> {
        let peeked = self.peek().clone();
        match peeked.token_type {
            TokenType::Value(v) => {
                self.advance();
                Ok(Expression::Value(v.clone()))
            }
            TokenType::Identifier(id) => {
                self.advance();
                let name = id.clone();
                self.parse_identifier_expr(name)
            }
            TokenType::Operator(op) if op == "-" || op == "!" => {
                self.advance();
                let operand = self.parse_primary()?;
                Ok(Expression::Unary {
                    operator: op.clone(),
                    operand: Box::new(operand),
                })
            }
            TokenType::Operator(op) if op == "++" || op == "--" => {
                self.advance();
                self.parse_pre_incdec(op.as_str())
            }
            TokenType::Symbol(s) if s == "(" => {
                self.advance();
                let expr = self.parse_binary(0)?;
                if !self.eat(&TokenType::Symbol(")".to_string())) {
                    return Err(format!(
                        "Expected ')' at the end of expression at {}:{}",
                        self.peek().line, self.peek().column
                    ));
                }
                Ok(expr)
            }
            TokenType::Symbol(s) if s == "[" => {
                self.advance();
                Ok(self.parse_list()?)
            }
            TokenType::Symbol(s) if s == "{" => {
                self.advance();
                Ok(self.parse_object()?)
            }
            TokenType::Keyword(k) if k == "fn" => {
                self.advance();
                Ok(self.parse_closure()?)
            }
            _ => Err(format!(
                "Unexpected token in expression: {:?} at {}:{}",
                self.peek().token_type,
                self.peek().line,
                self.peek().column
            )),
        }
    }

    fn parse_match(&mut self) -> Result<Statement, String> {
        self.advance();
        let value = self.parse_binary(0)?;
        if !self.eat(&TokenType::Symbol("{".to_string())) {
            return Err(format!(
                "Expected '{{' after 'match' at {}:{}",
                self.peek().line, self.peek().column
            ));
        }
        let mut cases = vec![];
        while self.peek().token_type != TokenType::Symbol("}".to_string()) {
            if self.eat(&TokenType::Newline) {
                continue;
            }
            let case_value = self.parse_binary(0)?;
            if !self.eat(&TokenType::Symbol(":".to_string())) {
                return Err(format!(
                    "Expected ':' after case value at {}:{}",
                    self.peek().line, self.peek().column
                ));
            }
            let body = self.parse_block()?;
            cases.push((case_value, body));
        }
        if !self.eat(&TokenType::Symbol("}".to_string())) {
            return Err(format!(
                "Expected '}}' at the end of match at {}:{}",
                self.peek().line, self.peek().column
            ));
        }
        let default = if self.eat(&TokenType::Keyword("else".to_string())) {
            let body = self.parse_block()?;
            Some(body)
        } else {
            None
        };
        Ok(Statement::Match {
            value,
            cases,
            default,
        })
    }

    fn parse_if(&mut self) -> Result<Statement, String> {
        self.advance();
        let condition = self.parse_binary(0)?;
        let body = self.parse_block()?;
        let mut else_body = None;
        let mut else_if_bodies = vec![];
        while self.eat(&TokenType::Keyword("else".to_string())) {
            if self.eat(&TokenType::Keyword("if".to_string())) {
                let else_if_condition = self.parse_binary(0)?;
                let else_if_body = self.parse_block()?;
                else_if_bodies.push((else_if_condition, else_if_body))
            } else {
                let else_body_ = self.parse_block()?;
                else_body = Some(else_body_);
            }
        }

        Ok(Statement::If {
            condition,
            body,
            else_if_bodies,
            else_body,
        })
    }

    fn parse_while(&mut self) -> Result<Statement, String> {
        self.advance();
        let condition = self.parse_binary(0)?;
        let body = self.parse_block()?;
        Ok(Statement::While { condition, body })
    }

    fn parse_for(&mut self) -> Result<Statement, String> {
        self.advance();
        if let TokenType::Identifier(ref id) = self.peek().token_type {
            let identifier = id.clone();
            self.advance();
            if !self.eat(&TokenType::Operator("in".to_string())) {
                return Err(format!(
                    "Expected 'in' after for loop identifier at {}:{}",
                    self.peek().line, self.peek().column
                ));
            }
            let iterable = self.parse_binary(0)?;
            let body = self.parse_block()?;
            Ok(Statement::For {
                identifier,
                iterable,
                body,
            })
        } else {
            Err(format!(
                "Expected identifier after 'for' at {}:{}",
                self.peek().line, self.peek().column
            ))
        }
    }

    fn parse_function_definition(&mut self) -> Result<Statement, String> {
        self.advance();
        if let TokenType::Identifier(ref id) = self.peek().token_type {
            let name = id.clone();
            self.advance();
            if !self.eat(&TokenType::Symbol("(".to_string())) {
                return Err(format!(
                    "Expected '(' after function name at {}:{}",
                    self.peek().line, self.peek().column
                ));
            }
            let mut args = vec![];
            while self.peek().token_type != TokenType::Symbol(")".to_string()) {
                if self.eat(&TokenType::Newline) {
                    continue;
                }
                if let TokenType::Identifier(ref arg) = self.peek().token_type {
                    args.push(arg.clone());
                    self.advance();
                } else {
                    return Err(format!(
                        "Expected identifier in function arguments but got {:?} at {}:{}",
                        self.peek().token_type, self.peek().line, self.peek().column
                    ));
                }
                if !self.eat(&TokenType::Symbol(",".to_string())) {
                    break;
                }
            }
            if !self.eat(&TokenType::Symbol(")".to_string())) {
                return Err(format!(
                    "Expected ')' after function arguments at {}:{}",
                    self.peek().line, self.peek().column
                ));
            }
            let returns = self.parse_binary(0)?;
            let body = self.parse_block()?;
            Ok(Statement::FunctionDefinition {
                name,
                args,
                body,
                returns,
            })
        } else {
            Err(format!(
                "Expected identifier after 'fn' at {}:{}",
                self.peek().line, self.peek().column
            ))
        }
    }

    fn parse_import(&mut self) -> Result<Statement, String> {
        self.advance();
        if let TokenType::Value(Value::String(ref path)) = self.peek().token_type {
            let path = path.clone();
            self.advance();
            Ok(Statement::Import { path })
        } else {
            Err(format!(
                "Expected string path after 'import' at {}:{}",
                self.peek().line, self.peek().column
            ))
        }
    }

    fn parse_assignment_or_call(&mut self) -> Result<Statement, String> {
        let name = if let TokenType::Identifier(ref id) = self.peek().token_type {
            id.clone()
        } else {
            return Err(format!(
                "Expected identifier but got {:?} at {}:{}",
                self.peek().token_type, self.peek().line, self.peek().column
            ));
        };
        self.advance();

        let target_expr = self.parse_identifier_expr(name)?;

        if self.eat(&TokenType::Operator("=".to_string())) {
            let value = self.parse_binary(0)?;
            Ok(Statement::Assignment {
                is_global: false,
                identifier: target_expr,
                value,
            })
        } else if let Some(TokenType::Operator(op)) = self.eat_any(&[
            TokenType::Operator("+=".to_string()),
            TokenType::Operator("-=".to_string()),
            TokenType::Operator("*=".to_string()),
            TokenType::Operator("/=".to_string()),
        ]) {
            let real_op = op[0..1].to_string();
            let right = self.parse_binary(0)?;
            let combined_expr = Expression::Binary {
                left: Box::new(target_expr.clone()),
                operator: real_op,
                right: Box::new(right),
            };
            Ok(Statement::Assignment {
                is_global: false,
                identifier: target_expr,
                value: combined_expr,
            })
        } else if matches!(target_expr, Expression::Call { .. }) {
            Ok(Statement::Call(target_expr))
        } else {
            Err(format!(
                "Unexpected token after identifier expression at {}:{}",
                self.peek().line, self.peek().column
            ))
        }
    }

    pub fn parse_global_assignment(&mut self) -> Result<Statement, String> {
        if self.eat(&TokenType::Keyword("global".to_string())) {
            let name = if let TokenType::Identifier(ref id) = self.peek().token_type {
                id.clone()
            } else {
                return Err("Expected identifier after 'global'".to_string());
            };
            self.advance();

            let target_expr = self.parse_identifier_expr(name)?;

            if self.eat(&TokenType::Operator("=".to_string())) {
                let value = self.parse_binary(0)?;
                Ok(Statement::Assignment {
                    is_global: false,
                    identifier: target_expr,
                    value,
                })
            } else if let Some(TokenType::Operator(op)) = self.eat_any(&[
                TokenType::Operator("+=".to_string()),
                TokenType::Operator("-=".to_string()),
                TokenType::Operator("*=".to_string()),
                TokenType::Operator("/=".to_string()),
            ]) {
                let real_op = op[0..1].to_string();
                let right = self.parse_binary(0)?;
                let combined_expr = Expression::Binary {
                    left: Box::new(target_expr.clone()),
                    operator: real_op,
                    right: Box::new(right),
                };
                Ok(Statement::Assignment {
                    is_global: false,
                    identifier: target_expr,
                    value: combined_expr,
                })
            } else if matches!(target_expr, Expression::Call { .. }) {
                Ok(Statement::Call(target_expr))
            } else {
                Err(format!(
                    "Unexpected token after identifier expression at {}:{}",
                    self.peek().line, self.peek().column
                ))
            }
        } else {
            Err(format!(
                "Expected 'global' keyword at {}:{}",
                self.peek().line, self.peek().column
            ))
        }
    }
}
