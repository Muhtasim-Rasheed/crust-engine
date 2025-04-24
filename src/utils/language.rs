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
            "if", "else", "while", "repeat", "global", "setup", "update",
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
        
        // Check if the current char is a `#`
        if c.starts_with('#') {
            while self.pointer < self.code.len() && !self.code[self.pointer..].starts_with('\n') {
                self.pointer += 1;
            }
            return Some(Token::Newline);
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
        if ["+=", "-=", "*=", "/=", "==", "!=", "<=", ">="].contains(&two) {
            self.pointer += 2;
            return Some(Token::Operator(two.to_string()));
        }

        // Single-char operators
        let one = &self.code[self.pointer..self.pointer + 1];
        if ["=", "+", "-", "*", "/", "<", ">"].contains(&one) {
            self.pointer += 1;
            return Some(Token::Operator(one.to_string()));
        }

        // Symbols
        if ["(", ")", "{", "}", ",", ";"].contains(&one) {
            self.pointer += 1;
            return Some(Token::Symbol(one.to_string()));
        }

        // Numbers
        if c.chars().next().unwrap().is_ascii_digit() {
            let number = self.get_number();
            return Some(Token::Value(Value::Number(number.parse().unwrap())));
        }

        // Lists
        if c.starts_with('[') {
            self.pointer += 1;
            let mut list = vec![];
            while self.pointer < self.code.len() && !self.code[self.pointer..].starts_with(']') {
                if let Some(token) = self.tokenize() {
                    if let Token::Value(v) = token {
                        list.push(v);
                    }
                }
                if self.pointer < self.code.len() && self.code[self.pointer..].starts_with(',') {
                    self.pointer += 1;
                }
            }
            self.pointer += 1; // Skip closing ]
            return Some(Token::Value(Value::List(list)));
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
    Identifier(String),
    Binary {
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
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
            Expression::Identifier(id) => id.clone(),
            Expression::Binary { left, operator, right } => {
                format!("({} {} {})", left.to_string(), operator, right.to_string())
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
    Call(Expression),
}

impl std::fmt::Debug for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Assignment { is_global, identifier, value } => write!(f, "ASSIGN[{} {} = {}]", if *is_global { "global" } else { "" }, identifier, value.to_string()),
            Statement::If { condition, body, else_body } => write!(f, "IF[{}] {{ {:?} }} ELSE {{ {:?} }}", condition.to_string(), body, else_body),
            Statement::While { condition, body } => write!(f, "WHILE[{}] {{ {:?} }}", condition.to_string(), body),
            Statement::Repeat { times, body } => write!(f, "REPEAT[{}] {{ {:?} }}", times.to_string(), body),
            Statement::Setup { body } => write!(f, "SETUP {{ {:?} }}", body),
            Statement::Update { body } => write!(f, "UPDATE {{ {:?} }}", body),
            Statement::Call(expr) => write!(f, "CALL[{}]", expr.to_string()),
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

    fn current(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn next(&mut self) {
        if self.current < self.tokens.len() {
            self.current += 1;
        }
    }

    fn eat(&mut self, token_type: &Token) -> bool {
        if self.current() == token_type {
            self.next();
            return true;
        }
        false
    }
    
    fn precedence(op: &str) -> u8 {
        match op {
            "*" | "/" => 4,
            "+" | "-" => 3,
            "==" | "!=" | "<" | ">" | "<=" | ">=" => 2,
            "=" => 1,
            _ => 0,
        }
    }

    fn parse_expr(&mut self) -> Expression {
        let mut expr = self.parse_primary();

        loop {
            match self.current() {
                Token::Symbol(s) if s == "(" => {
                    self.next();
                    let mut args = vec![];
                    if self.current() != &Token::Symbol(")".to_string()) {
                        loop {
                            args.push(self.parse_expr());
                            if self.current() == &Token::Symbol(",".to_string()) {
                                self.next();
                            } else {
                                break;
                            }
                        }
                    }
                    if !self.eat(&Token::Symbol(")".to_string())) {
                        panic!("Expected ')'");
                    }

                    expr = Expression::Call {
                        function: expr.to_string(),
                        args,
                    };
                }
                _ => break,
            }
        }

        expr
    }

    fn parse_binary_expr(&mut self, min_prec: u8) -> Expression {
        let mut left = self.parse_expr();

        while let Token::Operator(op) = self.current() {
            let prec = Parser::precedence(op);
            if prec < min_prec {
                break;
            }

            let op = op.clone();
            self.next();

            let right = self.parse_binary_expr(prec + 1);

            left = Expression::Binary {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_primary(&mut self) -> Expression {
        match self.current() {
            Token::Value(v) => {
                let v = v.clone();
                self.next();
                Expression::Value(v)
            }
            Token::Identifier(name) => {
                let name = name.clone();
                self.next();
                Expression::Identifier(name)
            }
            Token::Symbol(s) if s == "(" => {
                self.next();
                let expr = self.parse_binary_expr(0);
                if let Token::Symbol(s) = self.current() {
                    if s == ")" {
                        self.next();
                        expr
                    } else {
                        panic!("Expected ')'");
                    }
                } else {
                    panic!("Expected ')'");
                }
            }
            _ => {
                panic!("Unexpected token in expression: {:?}", self.current());
            }
        }
    }

    fn parse_block(&mut self) -> Vec<Statement> {
        let mut statements = vec![];

        if !self.eat(&Token::Symbol("{".to_string())) {
            panic!("Expected '{{' at the start of block");
        }

        while self.current() != &Token::Symbol("}".to_string()) {
            if let Token::Newline = self.current() {
                self.next();
                continue;
            }
            let statement = self.parse_statement();
            statements.push(statement);
        }

        if !self.eat(&Token::Symbol("}".to_string())) {
            panic!("Expected '}}' at the end of block");
        }

        statements
    }

    fn parse_statement(&mut self) -> Statement {
        match self.current() {
            Token::Keyword(k) if k == "if" => {
                self.next();
                let condition = self.parse_binary_expr(0);
                let body = self.parse_block();
                let else_body = if self.eat(&Token::Keyword("else".to_string())) {
                    if self.eat(&Token::Keyword("if".to_string())) {
                        let condition = self.parse_binary_expr(0);
                        let body = self.parse_block();
                        let else_body = if self.eat(&Token::Keyword("else".to_string())) {
                            Some(self.parse_block())
                        } else {
                            None
                        };
                        Some(vec![Statement::If { condition, body, else_body }])
                    } else {
                        let body = self.parse_block();
                        Some(body)
                    }
                } else {
                    None
                };
                Statement::If { condition, body, else_body }
            }
            Token::Keyword(k) if k == "while" => {
                self.next();
                let condition = self.parse_binary_expr(0);
                let body = self.parse_block();
                Statement::While { condition, body }
            }
            Token::Keyword(k) if k == "repeat" => {
                self.next();
                let times = self.parse_binary_expr(0);
                let body = self.parse_block();
                Statement::Repeat { times, body }
            }
            Token::Keyword(k) if k == "setup" => {
                self.next();
                let body = self.parse_block();
                Statement::Setup { body }
            }
            Token::Keyword(k) if k == "update" => {
                self.next();
                let body = self.parse_block();
                Statement::Update { body }
            }
            Token::Keyword(k) if k == "global" => {
                self.next();
                let name = if let Token::Identifier(name) = self.current() {
                    name.clone()
                } else {
                    panic!("Expected identifier after 'global'");
                };
                self.next();
                if let Token::Operator(op) = self.current() {
                    match op.as_str() {
                        "=" => {
                            self.next();
                            let value = self.parse_binary_expr(0);
                            Statement::Assignment {
                                is_global: true,
                                identifier: name,
                                value,
                            }
                        }
                        "+=" | "-=" | "*=" | "/=" => {
                            let real_op = op[0..1].to_string(); // extract +, -, *, /
                            self.next();
                            let right = self.parse_binary_expr(0);
                            let left_expr = Expression::Identifier(name.clone());
                            let combined_expr = Expression::Binary {
                                left: Box::new(left_expr),
                                operator: real_op,
                                right: Box::new(right),
                            };
                            Statement::Assignment {
                                is_global: true,
                                identifier: name,
                                value: combined_expr,
                            }
                        }
                        _ => {
                            panic!("Unexpected operator after 'global {}': {}", name, op);
                        }
                    }
                } else {
                    panic!("Expected '=' after 'global'");
                }
            }
            Token::Identifier(name) => {
                let name = name.clone();
                self.next();
                if let Token::Operator(op) = self.current() {
                    match op.as_str() {
                        "=" => {
                            self.next();
                            let value = self.parse_binary_expr(0);
                            Statement::Assignment {
                                is_global: false,
                                identifier: name,
                                value,
                            }
                        }
                        "+=" | "-=" | "*=" | "/=" => {
                            let real_op = op[0..1].to_string(); // extract +, -, *, /
                            self.next();
                            let right = self.parse_binary_expr(0);
                            let left_expr = Expression::Identifier(name.clone());
                            let combined_expr = Expression::Binary {
                                left: Box::new(left_expr),
                                operator: real_op,
                                right: Box::new(right),
                            };
                            Statement::Assignment {
                                is_global: false,
                                identifier: name,
                                value: combined_expr,
                            }
                        }
                        _ => {
                            // Statement::Call(Expression::Identifier(name))
                            let mut args = vec![];
                            if self.eat(&Token::Symbol("(".to_string())) {
                                while self.current() != &Token::Symbol(")".to_string()) {
                                    if let Token::Newline = self.current() {
                                        self.next();
                                        continue;
                                    }
                                    let arg = self.parse_binary_expr(0);
                                    args.push(arg);
                                    if !self.eat(&Token::Symbol(",".to_string())) {
                                        break;
                                    }
                                }
                                if !self.eat(&Token::Symbol(")".to_string())) {
                                    panic!("Expected ')' after function call");
                                }
                            }
                            Statement::Call(Expression::Call {
                                function: name,
                                args,
                            })
                        }
                    }
                } else {
                    // Statement::Call(Expression::Identifier(name))
                    let mut args = vec![];
                    if self.eat(&Token::Symbol("(".to_string())) {
                        while self.current() != &Token::Symbol(")".to_string()) {
                            if let Token::Newline = self.current() {
                                self.next();
                                continue;
                            }
                            let arg = self.parse_binary_expr(0);
                            args.push(arg);
                            if !self.eat(&Token::Symbol(",".to_string())) {
                                break;
                            }
                        }
                        if !self.eat(&Token::Symbol(")".to_string())) {
                            panic!("Expected ')' after function call");
                        }
                    }
                    Statement::Call(Expression::Call {
                        function: name,
                        args,
                    })
                }
            }
            _ => panic!("Unexpected token in statement: {:?} at {}", self.current(), self.current),
        }
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = vec![];

        while self.current() != &Token::EOF {
            if let Token::Newline = self.current() {
                self.next();
                continue;
            }
            let statement = self.parse_statement();
            statements.push(statement);
        }

        statements
    }
}
