use super::Value;

// ========================= Tokenizer ========================= \\

#[derive(Clone, PartialEq)]
pub enum Token {
    Newline,
    EOF,
    Identifier(String),
    Value(Value),
    Operator(String),
    Keyword(String),
    Symbol(String),
}

impl Token {
    pub fn to_string(&self) -> String {
        match self {
            Token::Newline => "\n".to_string(),
            Token::EOF => "EOF".to_string(),
            Token::Identifier(s) => format!("ID[{}]", s),
            Token::Value(v) => format!("VAL[{}]", v.to_string()),
            Token::Operator(s) => format!("OP[{}]", s),
            Token::Keyword(s) => format!("KEY[{}]", s),
            Token::Symbol(s) => format!("SYM[{}]", s),
        }
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub struct Tokenizer {
    code: String,
    pointer: usize,
}

impl Tokenizer {
    pub fn new(code: String) -> Self {
        Self {
            code,
            pointer: 0,
        }
    }

    fn skip_whitespace(&mut self) {
        while self.pointer < self.code.len() && self.code[self.pointer..].starts_with(char::is_whitespace) {
            self.pointer += 1;
        }
    }

    fn get_number(&mut self) -> String {
        let start = self.pointer;
        while self.pointer < self.code.len() && self.code[self.pointer..].starts_with(char::is_numeric) |
              self.code[self.pointer..].starts_with('.') {
            self.pointer += 1;
        }
        self.code[start..self.pointer].to_string()
    }

    fn get_identifier(&mut self) -> String {
        let start = self.pointer;
        while self.pointer < self.code.len() && (self.code[self.pointer..].starts_with(char::is_alphanumeric) ||
              self.code[self.pointer..].starts_with('_')) {
            self.pointer += 1;
        }
        self.code[start..self.pointer].to_string()
    }

    fn tokenize(&mut self) -> Option<Token> {
        let keyword_list = vec![
            "if", "else", "while", "repeat", "global",
            "setup", "update", "fn",
            "import",
        ];

        if self.pointer >= self.code.len() {
            return None;
        }

        // Newline
        if self.code[self.pointer..].starts_with('\n') {
            self.pointer += 1;
            return Some(Token::Newline);
        }

        self.skip_whitespace();

        if self.pointer >= self.code.len() {
            return None;
        }

        let c = &self.code[self.pointer..];
        
        // Comments
        if c.starts_with("//") {
            while self.pointer < self.code.len() && !self.code[self.pointer..].starts_with('\n') {
                self.pointer += 1;
            }
            return Some(Token::Newline);
        }

        if c.starts_with("/*") {
            self.pointer += 2;
            while self.pointer < self.code.len() && !self.code[self.pointer..].starts_with("*/") {
                self.pointer += 1;
            }
            if self.pointer < self.code.len() {
                self.pointer += 2; // Skip */
            }
            return Some(Token::Newline);
        }

        // Null
        if c.starts_with("null") {
            self.pointer += 4;
            return Some(Token::Value(Value::Null));
        }

        // Booleans
        if c.starts_with("true") {
            self.pointer += 4;
            return Some(Token::Value(Value::Boolean(true)));
        }

        if c.starts_with("false") {
            self.pointer += 5;
            return Some(Token::Value(Value::Boolean(false)));
        }

        // Strings
        if c.starts_with('"') {
            self.pointer += 1;
            let start = self.pointer;
            while self.pointer < self.code.len() && !self.code[self.pointer..].starts_with('"') {
                self.pointer += 1;
            }
            let string = &self.code[start..self.pointer];
            self.pointer += 1; // Skip closing "
            return Some(Token::Value(Value::String(string.to_string())));
        }

        // Multi-char operators first
        let two = &self.code[self.pointer..self.pointer + 2.min(self.code.len() - self.pointer)];
        if ["+=", "-=", "*=", "/=", "==", "!=", "<=", ">=", "&&", "||"].contains(&two) {
            self.pointer += 2;
            return Some(Token::Operator(two.to_string()));
        }

        // Single-char operators
        let one = &self.code[self.pointer..self.pointer + 1];
        if ["=", "+", "-", "*", "/", "%", "^", "<", ">", "!"].contains(&one) {
            self.pointer += 1;
            return Some(Token::Operator(one.to_string()));
        }

        // Symbols
        if ["(", ")", "[", "]", "{", "}", ","].contains(&one) {
            self.pointer += 1;
            return Some(Token::Symbol(one.to_string()));
        }

        // Numbers
        if c.chars().next().unwrap().is_ascii_digit() {
            let number = self.get_number();
            return Some(Token::Value(Value::Number(number.parse().unwrap())));
        }

        if c.chars().next().unwrap() == '.' {
            self.pointer += 1;
            let number = self.get_number();
            return Some(Token::Value(Value::Number(format!(".{}", number).parse().unwrap())));
        }

        // Identifiers or keywords
        if c.chars().next().unwrap().is_alphabetic() || c.starts_with('_') {
            let ident = self.get_identifier();
            if keyword_list.contains(&ident.as_str()) {
                return Some(Token::Keyword(ident));
            } else {
                return Some(Token::Identifier(ident));
            }
        }

        // Skip unknown character
        self.pointer += 1;
        self.tokenize()
    }

    pub fn tokenize_full(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        while let Some(token) = self.tokenize() {
            tokens.push(token);
        }
        tokens.push(Token::EOF);
        tokens
    }
}

// ========================= Parser ========================= \\

#[derive(Clone)]
pub enum Expression {
    Value(Value),
    List(Vec<Expression>),
    ListMemberAccess {
        list: Box<Expression>,
        index: Box<Expression>,
    },
    Identifier(String),
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

impl Expression {
    pub fn to_string(&self) -> String {
        match self {
            Expression::Value(v) => v.to_string(),
            Expression::List(l) => {
                let list_str = l.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", ");
                format!("[{}]", list_str)
            }
            Expression::ListMemberAccess { list, index } => {
                format!("{}[{}]", list.to_string(), index.to_string())
            }
            Expression::Identifier(id) => id.clone(),
            Expression::Binary { left, operator, right } => {
                format!("({} {} {})", left.to_string(), operator, right.to_string())
            }
            Expression::Unary { operator, operand } => {
                format!("({}{})", operator, operand.to_string())
            }
            Expression::Call { function, args } => {
                let args_str = args.iter().map(|arg| arg.to_string()).collect::<Vec<_>>().join(", ");
                format!("{}({})", function, args_str)
            }
        }
    }
}

impl std::fmt::Debug for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Clone)]
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
    If {
        condition: Expression,
        body: Vec<Statement>,
        else_body: Option<Vec<Statement>>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    Repeat {
        times: Expression,
        body: Vec<Statement>,
    },
    Setup {
        body: Vec<Statement>,
    },
    Update {
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
            Statement::Assignment { is_global, identifier, value } => write!(f, "ASSIGN[{} {:?} = {}]", if *is_global { "global" } else { "" }, identifier, value.to_string()),
            Statement::ListMemberAssignment { is_global, identifier, index, value } => write!(f, "LIST_ASSIGN[{} {:?}[{}] = {}]", if *is_global { "global" } else { "" }, identifier, index.to_string(), value.to_string()),
            Statement::If { condition, body, else_body } => write!(f, "IF[{}] {{ {:?} }} ELSE {{ {:?} }}", condition.to_string(), body, else_body),
            Statement::While { condition, body } => write!(f, "WHILE[{}] {{ {:?} }}", condition.to_string(), body),
            Statement::Repeat { times, body } => write!(f, "REPEAT[{}] {{ {:?} }}", times.to_string(), body),
            Statement::Setup { body } => write!(f, "SETUP {{ {:?} }}", body),
            Statement::Update { body } => write!(f, "UPDATE {{ {:?} }}", body),
            Statement::Import { path } => write!(f, "IMPORT[{}]", path),
            Statement::Call(expr) => write!(f, "CALL[{}]", expr.to_string()),
            Statement::FunctionDefinition { name, args, body, returns } => write!(f, "FUNCTION[{}({:?}) -> {}] {{ {:?} }}", name, args, returns.to_string(), body),
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
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

        while self.peek() != Some(&Token::EOF) {
            if self.eat(&Token::Newline) {
                continue;
            }
            let statement = self.parse_statement().unwrap();
            statements.push(statement);
        }

        statements
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.peek().unwrap_or(&Token::EOF) {
            Token::Keyword(k) if k == "if" => self.parse_if(),
            Token::Keyword(k) if k == "while" => self.parse_while(),
            Token::Keyword(k) if k == "repeat" => self.parse_repeat(),
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
            Token::Keyword(k) if k == "fn" => self.parse_function_definition(),
            Token::Keyword(k) if k == "import" => self.parse_import(),
            Token::Identifier(_) => self.parse_assignment_or_call(),
            _ => Err(format!("Unexpected token: {:?}", self.peek().unwrap_or(&Token::EOF))),
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

        while let Some(Token::Operator(op)) = self.peek() {
            let prec = Parser::precedence(op);
            if prec < min_prec {
                break;
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

        if !self.eat(&Token::Symbol("]".to_string())) {
            return Err("Expected ']' at the end of list".to_string());
        }

        Ok(Expression::List(list))
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
                    Ok(Expression::Call { function: name, args })
                } else {
                    // Wait! Might be a list member access!
                    if self.eat(&Token::Symbol("[".to_string())) {
                        let index = self.parse_binary(0)?;
                        if !self.eat(&Token::Symbol("]".to_string())) {
                            return Err("Expected ']' after list member access".to_string());
                        }
                        Ok(Expression::ListMemberAccess {
                            list: Box::new(Expression::Identifier(name)),
                            index: Box::new(index),
                        })
                    } else {
                        // * ACTUALLY * an identifier
                        Ok(Expression::Identifier(name))
                    }
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
            _ => Err(format!("Unexpected token in expression: {:?}", self.peek())),
        }
    }

    fn parse_if(&mut self) -> Result<Statement, String> {
        self.advance();
        let condition = self.parse_binary(0)?;
        let body = self.parse_block()?;
        let else_body = if self.eat(&Token::Keyword("else".to_string())) {
            if self.eat(&Token::Keyword("if".to_string())) {
                if let Statement::If { body, .. } = self.parse_if()? {
                    Some(body)
                } else {
                    return Err(format!("This error should not happen here"))
                }
            } else {
                Some(self.parse_block()?)
            }
        } else { None };

        Ok(Statement::If { condition, body, else_body })
    }

    fn parse_while(&mut self) -> Result<Statement, String> {
        self.advance();
        let condition = self.parse_binary(0)?;
        let body = self.parse_block()?;
        Ok(Statement::While { condition, body })
    }

    fn parse_repeat(&mut self) -> Result<Statement, String> {
        self.advance();
        let times = self.parse_binary(0)?;
        let body = self.parse_block()?;
        Ok(Statement::Repeat { times, body })
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
            Ok(Statement::FunctionDefinition { name, args, body, returns })
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
        let is_global = self.eat(&Token::Keyword("global".to_string()));
        let identifier = if let Some(Token::Identifier(id)) = self.peek() {
            id.clone()
        } else {
            return Err("Expected identifier".to_string());
        };
        self.advance();

        if self.eat(&Token::Operator("=".to_string())) {
            let value = self.parse_binary(0)?;
            Ok(Statement::Assignment {
                is_global,
                identifier,
                value,
            })
        } else if let Some(Token::Operator(op)) = self.eat_any(
            &[
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
                is_global,
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
                    is_global,
                    identifier: Expression::Identifier(identifier),
                    index,
                    value,
                })
            } else if let Some(Token::Operator(op)) = self.eat_any(
                &[
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
                println!("combined_expr: {:?}", combined_expr);
                Ok(Statement::ListMemberAssignment {
                    is_global,
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
                Ok(Statement::Call(Expression::Call { function: identifier, args }))
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
            Ok(Statement::Call(Expression::Call { function: identifier, args }))
        }
    }
}
