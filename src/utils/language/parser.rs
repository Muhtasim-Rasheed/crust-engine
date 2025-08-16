use crate::utils::{Token, Value};
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
    ListMemberAccess {
        list: Box<Expression>,
        index: Box<Expression>,
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
        function: String,
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
            Expression::ListMemberAccess { list, index } => {
                write!(f, "{}[{}]", list.to_string(), index.to_string())
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
        identifier: String,
        value: Expression,
    },
    ListMemberAssignment {
        is_global: bool,
        identifier: Expression,
        index: Expression,
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
            Statement::ListMemberAssignment {
                is_global,
                identifier,
                index,
                value,
            } => write!(
                f,
                "LIST_ASSIGN[{}{:?}[{}] = {}]",
                if *is_global { "global " } else { "" },
                identifier,
                index.to_string(),
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

    fn peek(&self) -> Option<&Token> {
        if self.current < self.tokens.len() {
            Some(&self.tokens[self.current])
        } else {
            None
        }
    }

    fn advance(&mut self) {
        if self.current < self.tokens.len() {
            self.current += 1;
        }
    }

    fn eat(&mut self, token: &Token) -> bool {
        if self.peek() == Some(token) {
            self.advance();
            return true;
        }
        false
    }

    fn eat_any(&mut self, tokens: &[Token]) -> Option<Token> {
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

        while self.peek() != Some(&Token::EOF) {
            if self.eat(&Token::Newline) {
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
        match self.peek().unwrap_or(&Token::EOF) {
            Token::Keyword(k) if k == "nop" => {
                self.advance();
                Ok(Statement::Nop)
            }
            Token::Keyword(k) if k == "assert" => {
                self.advance();
                let condition = self.parse_binary(0)?;
                Ok(Statement::Assert { condition })
            }
            Token::Keyword(k) if k == "match" => self.parse_match(),
            Token::Keyword(k) if k == "if" => self.parse_if(),
            Token::Keyword(k) if k == "while" => self.parse_while(),
            Token::Keyword(k) if k == "for" => self.parse_for(),
            Token::Keyword(k) if k == "setup" => {
                self.advance();
                let body = self.parse_block()?;
                Ok(Statement::Setup { body })
            }
            Token::Keyword(k) if k == "update" => {
                self.advance();
                let body = self.parse_block()?;
                Ok(Statement::Update { body })
            }
            Token::Keyword(k) if k == "clone_setup" => {
                self.advance();
                let body = self.parse_block()?;
                Ok(Statement::CloneSetup { body })
            }
            Token::Keyword(k) if k == "clone_update" => {
                self.advance();
                let body = self.parse_block()?;
                Ok(Statement::CloneUpdate { body })
            }
            Token::Keyword(k) if k == "when" => {
                self.advance();
                if let Some(Token::Value(Value::String(broadcast))) = self.peek().cloned() {
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
            Token::Keyword(k) if k == "fn" => self.parse_function_definition(),
            Token::Keyword(k) if k == "import" => self.parse_import(),
            Token::Keyword(k) if k == "global" => self.parse_global_assignment(),
            Token::Identifier(_) => self.parse_assignment_or_call(),
            _ => Err(format!(
                "Unexpected token: {:?}",
                self.peek().unwrap_or(&Token::EOF)
            )),
        }
    }

    fn parse_block(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = vec![];

        if !self.eat(&Token::Symbol("{".to_string())) {
            return Err("Expected '{' at the start of block".to_string());
        }

        while self.peek() != Some(&Token::Symbol("}".to_string())) {
            if self.eat(&Token::Newline) {
                continue;
            }
            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        if !self.eat(&Token::Symbol("}".to_string())) {
            return Err("Expected '}' at the end of block".to_string());
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

        while let Some(token) = self.peek().cloned() {
            let op = match token {
                Token::Operator(op) => op,
                Token::Keyword(k) if k == "in" => k,
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
                        "Expected identifier for post-{} but got {:?}",
                        if op == "++" { "increment" } else { "decrement" },
                        left
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
        while self.peek() != Some(&Token::Symbol("]".to_string())) {
            if self.eat(&Token::Newline) {
                continue;
            }
            let expr = self.parse_binary(0)?;
            list.push(expr);
            if !self.eat(&Token::Symbol(",".to_string())) {
                break;
            }
        }

        self.eat(&Token::Newline);

        if !self.eat(&Token::Symbol("]".to_string())) {
            return Err("Expected ']' at the end of list".to_string());
        }

        Ok(Expression::List(list))
    }

    fn parse_object(&mut self) -> Result<Expression, String> {
        let mut object = HashMap::new();
        while self.peek() != Some(&Token::Symbol("}".to_string())) {
            if self.eat(&Token::Newline) {
                continue;
            }
            let peeked = self.peek().unwrap_or(&Token::EOF).clone();
            if let Token::Identifier(key) | Token::Value(Value::String(key)) = peeked {
                self.advance();
                if !self.eat(&Token::Symbol(":".to_string())) {
                    return Err(format!(
                        "Expected ':' after key in object but got {:?}",
                        self.peek()
                    ));
                }
                let value = self.parse_binary(0)?;
                object.insert(key.clone(), value);
                if !self.eat(&Token::Symbol(",".to_string())) {
                    break;
                }
            } else {
                return Err("Expected identifier as key in object".to_string());
            }
        }

        self.eat(&Token::Newline);

        if !self.eat(&Token::Symbol("}".to_string())) {
            return Err("Expected '}' at the end of object".to_string());
        }

        Ok(Expression::Object(object))
    }

    fn parse_closure(&mut self) -> Result<Expression, String> {
        if !self.eat(&Token::Symbol("(".to_string())) {
            return Err("Expected '(' after 'fn'".to_string());
        }
        let mut args = vec![];
        while self.peek() != Some(&Token::Symbol(")".to_string())) {
            if self.eat(&Token::Newline) {
                continue;
            }
            if let Some(Token::Identifier(arg)) = self.peek() {
                args.push(arg.clone());
                self.advance();
            } else {
                return Err("Expected identifier in closure arguments".to_string());
            }
            if !self.eat(&Token::Symbol(",".to_string())) {
                break;
            }
        }
        self.eat(&Token::Newline);
        if !self.eat(&Token::Symbol(")".to_string())) {
            return Err("Expected ')' after closure arguments".to_string());
        }
        let returns = self.parse_binary(0)?;
        let body = self.parse_block()?;
        Ok(Expression::Closure {
            args,
            body,
            returns: Box::new(returns),
        })
    }

    fn parse_primary(&mut self) -> Result<Expression, String> {
        let peeked = self.peek().unwrap_or(&Token::EOF).clone();
        match peeked {
            Token::Value(v) => {
                self.advance();
                Ok(Expression::Value(v.clone()))
            }
            Token::Identifier(id) => {
                self.advance();
                let name = id.clone();
                if self.eat(&Token::Symbol("(".to_string())) {
                    // Function call
                    let mut args = vec![];
                    while self.peek() != Some(&Token::Symbol(")".to_string())) {
                        if self.eat(&Token::Newline) {
                            continue;
                        }
                        let arg = self.parse_binary(0)?;
                        args.push(arg);
                        if !self.eat(&Token::Symbol(",".to_string())) {
                            break;
                        }
                    }
                    if !self.eat(&Token::Symbol(")".to_string())) {
                        return Err("Expected ')' after function call".to_string());
                    }
                    Ok(Expression::Call {
                        function: name,
                        args,
                    })
                } else if self.eat(&Token::Symbol("[".to_string())) {
                    let index = self.parse_binary(0)?;
                    if !self.eat(&Token::Symbol("]".to_string())) {
                        return Err("Expected ']' after list member access".to_string());
                    }
                    Ok(Expression::ListMemberAccess {
                        list: Box::new(Expression::Identifier(name)),
                        index: Box::new(index),
                    })
                } else if self.eat(&Token::Symbol(".".to_string())) {
                    let index = self.parse_primary()?;
                    match index {
                        Expression::Identifier(index_name) => Ok(Expression::ListMemberAccess {
                            list: Box::new(Expression::Identifier(name)),
                            index: Box::new(Expression::Value(Value::String(index_name))),
                        }),
                        Expression::Value(Value::Number(num)) => Ok(Expression::ListMemberAccess {
                            list: Box::new(Expression::Identifier(name)),
                            index: Box::new(Expression::Value(Value::Number(num))),
                        }),
                        _ => Err(format!(
                            "Expected identifier or number after '.' but got {:?}",
                            index
                        )),
                    }
                } else {
                    // * ACTUALLY * an identifier
                    Ok(Expression::Identifier(name))
                }
            }
            Token::Operator(op) if op == "-" || op == "!" => {
                self.advance();
                let operand = self.parse_primary()?;
                Ok(Expression::Unary {
                    operator: op.clone(),
                    operand: Box::new(operand),
                })
            }
            Token::Operator(op) if op == "++" || op == "--" => {
                self.advance();
                if let Some(Token::Identifier(id)) = self.peek().cloned() {
                    self.advance();
                    if op == "++" {
                        Ok(Expression::PreIncrement(id.clone()))
                    } else {
                        Ok(Expression::PreDecrement(id.clone()))
                    }
                } else {
                    Err("Expected identifier after '++' or '--'".to_string())
                }
            }
            Token::Symbol(s) if s == "(" => {
                self.advance();
                let expr = self.parse_binary(0)?;
                if !self.eat(&Token::Symbol(")".to_string())) {
                    return Err("Expected ')'".to_string());
                }
                Ok(expr)
            }
            Token::Symbol(s) if s == "[" => {
                self.advance();
                Ok(self.parse_list()?)
            }
            Token::Symbol(s) if s == "{" => {
                self.advance();
                Ok(self.parse_object()?)
            }
            Token::Keyword(k) if k == "fn" => {
                self.advance();
                Ok(self.parse_closure()?)
            }
            _ => Err(format!("Unexpected token in expression: {:?}", self.peek())),
        }
    }

    fn parse_match(&mut self) -> Result<Statement, String> {
        self.advance();
        let value = self.parse_binary(0)?;
        if !self.eat(&Token::Symbol("{".to_string())) {
            return Err("Expected '{' after 'match'".to_string());
        }
        let mut cases = vec![];
        while self.peek() != Some(&Token::Symbol("}".to_string())) {
            if self.eat(&Token::Newline) {
                continue;
            }
            let case_value = self.parse_binary(0)?;
            if !self.eat(&Token::Symbol(":".to_string())) {
                return Err("Expected ':' after case value".to_string());
            }
            let body = self.parse_block()?;
            cases.push((case_value, body));
        }
        if !self.eat(&Token::Symbol("}".to_string())) {
            return Err("Expected '}' at the end of match".to_string());
        }
        let default = if self.eat(&Token::Keyword("else".to_string())) {
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
        while self.eat(&Token::Keyword("else".to_string())) {
            if self.eat(&Token::Keyword("if".to_string())) {
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
        if let Some(Token::Identifier(id)) = self.peek() {
            let identifier = id.clone();
            self.advance();
            if !self.eat(&Token::Operator("in".to_string())) {
                return Err("Expected 'in' after for loop identifier".to_string());
            }
            let iterable = self.parse_binary(0)?;
            let body = self.parse_block()?;
            Ok(Statement::For {
                identifier,
                iterable,
                body,
            })
        } else {
            Err("Expected identifier after 'for'".to_string())
        }
    }

    fn parse_function_definition(&mut self) -> Result<Statement, String> {
        self.advance();
        if let Some(Token::Identifier(id)) = self.peek() {
            let name = id.clone();
            self.advance();
            if !self.eat(&Token::Symbol("(".to_string())) {
                return Err("Expected '(' after function name".to_string());
            }
            let mut args = vec![];
            while self.peek() != Some(&Token::Symbol(")".to_string())) {
                if self.eat(&Token::Newline) {
                    continue;
                }
                if let Some(Token::Identifier(arg)) = self.peek() {
                    args.push(arg.clone());
                    self.advance();
                } else {
                    return Err("Expected identifier in function arguments".to_string());
                }
                if !self.eat(&Token::Symbol(",".to_string())) {
                    break;
                }
            }
            if !self.eat(&Token::Symbol(")".to_string())) {
                return Err("Expected ')' after function arguments".to_string());
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
            Err("Expected name (identifier) after 'fn'".to_string())
        }
    }

    fn parse_import(&mut self) -> Result<Statement, String> {
        self.advance();
        if let Some(Token::Value(Value::String(path))) = self.peek() {
            let path = path.clone();
            self.advance();
            if !self.eat(&Token::Newline) {
                return Err("Expected newline after import statement".to_string());
            }
            Ok(Statement::Import { path: path.clone() })
        } else {
            Err("Expected string path after 'import'".to_string())
        }
    }

    fn parse_assignment_or_call(&mut self) -> Result<Statement, String> {
        let identifier = if let Some(Token::Identifier(id)) = self.peek() {
            id.clone()
        } else {
            return Err("Expected identifier".to_string());
        };
        self.advance();

        if self.eat(&Token::Operator("=".to_string())) {
            let value = self.parse_binary(0)?;
            Ok(Statement::Assignment {
                is_global: false,
                identifier,
                value,
            })
        } else if let Some(Token::Operator(op)) = self.eat_any(&[
            Token::Operator("+=".to_string()),
            Token::Operator("-=".to_string()),
            Token::Operator("*=".to_string()),
            Token::Operator("/=".to_string()),
        ]) {
            let real_op = op[0..1].to_string(); // extract +, -, *, /
            let right = self.parse_binary(0)?;
            let left_expr = Expression::Identifier(identifier.clone());
            let combined_expr = Expression::Binary {
                left: Box::new(left_expr),
                operator: real_op,
                right: Box::new(right),
            };
            Ok(Statement::Assignment {
                is_global: false,
                identifier,
                value: combined_expr,
            })
        } else if self.eat(&Token::Symbol("[".to_string())) {
            // List member access
            let index = self.parse_binary(0)?;
            if !self.eat(&Token::Symbol("]".to_string())) {
                return Err("Expected ']' after list member access".to_string());
            }
            if self.eat(&Token::Operator("=".to_string())) {
                let value = self.parse_binary(0)?;
                Ok(Statement::ListMemberAssignment {
                    is_global: false,
                    identifier: Expression::Identifier(identifier),
                    index,
                    value,
                })
            } else if let Some(Token::Operator(op)) = self.eat_any(&[
                Token::Operator("+=".to_string()),
                Token::Operator("-=".to_string()),
                Token::Operator("*=".to_string()),
                Token::Operator("/=".to_string()),
            ]) {
                let real_op = op[0..1].to_string(); // extract +, -, *, /
                let right = self.parse_binary(0)?;
                let left_expr = Expression::ListMemberAccess {
                    list: Box::new(Expression::Identifier(identifier.clone())),
                    index: Box::new(index.clone()),
                };
                let combined_expr = Expression::Binary {
                    left: Box::new(left_expr),
                    operator: real_op,
                    right: Box::new(right),
                };
                Ok(Statement::ListMemberAssignment {
                    is_global: false,
                    identifier: Expression::Identifier(identifier),
                    index,
                    value: combined_expr,
                })
            } else {
                // Function call
                let mut args = vec![];
                if self.eat(&Token::Symbol("(".to_string())) {
                    while self.peek() != Some(&Token::Symbol(")".to_string())) {
                        if self.eat(&Token::Newline) {
                            continue;
                        }
                        let arg = self.parse_binary(0)?;
                        args.push(arg);
                        if !self.eat(&Token::Symbol(",".to_string())) {
                            break;
                        }
                    }
                    if !self.eat(&Token::Symbol(")".to_string())) {
                        return Err("Expected ')' after function call".to_string());
                    }
                }
                Ok(Statement::Call(Expression::Call {
                    function: identifier,
                    args,
                }))
            }
        } else if self.eat(&Token::Symbol(".".to_string())) {
            // List member access with dot notation
            let index = self.parse_primary()?;
            match index {
                Expression::Identifier(index_name) => {
                    if self.eat(&Token::Operator("=".to_string())) {
                        let value = self.parse_binary(0)?;
                        Ok(Statement::ListMemberAssignment {
                            is_global: false,
                            identifier: Expression::Identifier(identifier),
                            index: Expression::Value(Value::String(index_name)),
                            value,
                        })
                    } else if let Some(Token::Operator(op)) = self.eat_any(&[
                        Token::Operator("+=".to_string()),
                        Token::Operator("-=".to_string()),
                        Token::Operator("*=".to_string()),
                        Token::Operator("/=".to_string()),
                    ]) {
                        let real_op = op[0..1].to_string(); // extract +, -, *, /
                        let right = self.parse_binary(0)?;
                        let left_expr = Expression::ListMemberAccess {
                            list: Box::new(Expression::Identifier(identifier.clone())),
                            index: Box::new(Expression::Value(Value::String(index_name.clone()))),
                        };
                        let combined_expr = Expression::Binary {
                            left: Box::new(left_expr),
                            operator: real_op,
                            right: Box::new(right),
                        };
                        Ok(Statement::ListMemberAssignment {
                            is_global: false,
                            identifier: Expression::Identifier(identifier),
                            index: Expression::Value(Value::String(index_name)),
                            value: combined_expr,
                        })
                    } else {
                        Err("Expected '=' or operator after '.'".to_string())
                    }
                }
                _ => Err(format!("Expected identifier after '.' but got {:?}", index)),
            }
        } else {
            // Function call
            let mut args = vec![];
            if self.eat(&Token::Symbol("(".to_string())) {
                while self.peek() != Some(&Token::Symbol(")".to_string())) {
                    if self.eat(&Token::Newline) {
                        continue;
                    }
                    let arg = self.parse_binary(0)?;
                    args.push(arg);
                    if !self.eat(&Token::Symbol(",".to_string())) {
                        break;
                    }
                }
                if !self.eat(&Token::Symbol(")".to_string())) {
                    return Err("Expected ')' after function call".to_string());
                }
            }
            Ok(Statement::Call(Expression::Call {
                function: identifier,
                args,
            }))
        }
    }

    pub fn parse_global_assignment(&mut self) -> Result<Statement, String> {
        if self.eat(&Token::Keyword("global".to_string())) {
            let identifier = if let Some(Token::Identifier(id)) = self.peek() {
                id.clone()
            } else {
                return Err("Expected identifier after 'global'".to_string());
            };
            self.advance();

            if self.eat(&Token::Operator("=".to_string())) {
                let value = self.parse_binary(0)?;
                Ok(Statement::Assignment {
                    is_global: true,
                    identifier,
                    value,
                })
            } else if let Some(Token::Operator(op)) = self.eat_any(&[
                Token::Operator("+=".to_string()),
                Token::Operator("-=".to_string()),
                Token::Operator("*=".to_string()),
                Token::Operator("/=".to_string()),
            ]) {
                let real_op = op[0..1].to_string(); // extract +, -, *, /
                let right = self.parse_binary(0)?;
                let left_expr = Expression::Identifier(identifier.clone());
                let combined_expr = Expression::Binary {
                    left: Box::new(left_expr),
                    operator: real_op,
                    right: Box::new(right),
                };
                Ok(Statement::Assignment {
                    is_global: true,
                    identifier,
                    value: combined_expr,
                })
            } else if self.eat(&Token::Symbol("[".to_string())) {
                // List member access
                let index = self.parse_binary(0)?;
                if !self.eat(&Token::Symbol("]".to_string())) {
                    return Err("Expected ']' after list member access".to_string());
                }
                if self.eat(&Token::Operator("=".to_string())) {
                    let value = self.parse_binary(0)?;
                    Ok(Statement::ListMemberAssignment {
                        is_global: true,
                        identifier: Expression::Identifier(identifier),
                        index,
                        value,
                    })
                } else if let Some(Token::Operator(op)) = self.eat_any(&[
                    Token::Operator("+=".to_string()),
                    Token::Operator("-=".to_string()),
                    Token::Operator("*=".to_string()),
                    Token::Operator("/=".to_string()),
                ]) {
                    let real_op = op[0..1].to_string(); // extract +, -, *, /
                    let right = self.parse_binary(0)?;
                    let left_expr = Expression::ListMemberAccess {
                        list: Box::new(Expression::Identifier(identifier.clone())),
                        index: Box::new(index.clone()),
                    };
                    let combined_expr = Expression::Binary {
                        left: Box::new(left_expr),
                        operator: real_op,
                        right: Box::new(right),
                    };
                    Ok(Statement::ListMemberAssignment {
                        is_global: true,
                        identifier: Expression::Identifier(identifier),
                        index,
                        value: combined_expr,
                    })
                } else {
                    Err("Expected '=' or operator after list member access".to_string())
                }
            } else if self.eat(&Token::Symbol(".".to_string())) {
                let index = self.parse_primary()?;
                match index {
                    Expression::Identifier(index_name) => {
                        if self.eat(&Token::Operator("=".to_string())) {
                            let value = self.parse_binary(0)?;
                            Ok(Statement::ListMemberAssignment {
                                is_global: true,
                                identifier: Expression::Identifier(identifier),
                                index: Expression::Value(Value::String(index_name)),
                                value,
                            })
                        } else if let Some(Token::Operator(op)) = self.eat_any(&[
                            Token::Operator("+=".to_string()),
                            Token::Operator("-=".to_string()),
                            Token::Operator("*=".to_string()),
                            Token::Operator("/=".to_string()),
                        ]) {
                            let real_op = op[0..1].to_string(); // extract +, -, *, /
                            let right = self.parse_binary(0)?;
                            let left_expr = Expression::ListMemberAccess {
                                list: Box::new(Expression::Identifier(identifier.clone())),
                                index: Box::new(Expression::Value(Value::String(
                                    index_name.clone(),
                                ))),
                            };
                            let combined_expr = Expression::Binary {
                                left: Box::new(left_expr),
                                operator: real_op,
                                right: Box::new(right),
                            };
                            Ok(Statement::ListMemberAssignment {
                                is_global: true,
                                identifier: Expression::Identifier(identifier),
                                index: Expression::Value(Value::String(index_name)),
                                value: combined_expr,
                            })
                        } else {
                            Err("Expected '=' or operator after '.'".to_string())
                        }
                    }
                    _ => Err(format!("Expected identifier after '.' but got {:?}", index)),
                }
            } else {
                Err("Expected '=' or operator after identifier".to_string())
            }
        } else {
            Err("Expected 'global' keyword".to_string())
        }
    }
}
