use crate::utils::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Newline,
    EOF,
    Identifier(String),
    Value(Value),
    Operator(String),
    Keyword(String),
    Symbol(String),
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Newline => writeln!(f),
            TokenType::EOF => write!(f, "EOF"),
            TokenType::Identifier(s) => write!(f, "ID[{}]", s),
            TokenType::Value(v) => write!(f, "VAL[{}]", v.to_string()),
            TokenType::Operator(s) => write!(f, "OP[{}]", s),
            TokenType::Keyword(s) => write!(f, "KEY[{}]", s),
            TokenType::Symbol(s) => write!(f, "SYM[{}]", s),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, column: usize) -> Self {
        Self {
            token_type,
            line,
            column,
        }
    }
}

pub struct Tokenizer {
    code: String,
    pointer: usize,
}

impl Tokenizer {
    pub fn new(code: String) -> Self {
        Self { code, pointer: 0 }
    }

    fn skip_whitespace(&mut self) {
        while self.pointer < self.code.len()
            && self.code[self.pointer..].starts_with(char::is_whitespace)
        {
            self.pointer += 1;
        }
    }

    fn get_number(&mut self) -> String {
        let start = self.pointer;
        while self.pointer < self.code.len()
            && self.code[self.pointer..].starts_with(char::is_numeric)
            || self.code[self.pointer..].starts_with('.')
        {
            self.pointer += 1;
        }
        self.code[start..self.pointer].to_string()
    }

    fn get_identifier(&mut self) -> String {
        let start = self.pointer;
        while self.pointer < self.code.len()
            && (self.code[self.pointer..].starts_with(char::is_alphanumeric)
                || self.code[self.pointer..].starts_with('_'))
        {
            self.pointer += 1;
        }
        self.code[start..self.pointer].to_string()
    }

    fn tokenize(&mut self) -> Option<Token> {
        let keyword_list = vec![
            "nop",
            "match",
            "if",
            "else",
            "while",
            "for",
            "in",
            "global",
            "assert",
            "setup",
            "update",
            "clone_setup",
            "clone_update",
            "when",
            "fn",
            "import",
        ];

        self.skip_whitespace();

        let line = self.code[..self.pointer].lines().count();
        let column = self.code[..self.pointer]
            .rfind('\n')
            .map_or(self.pointer, |pos| self.pointer - pos - 1)
            + 1;

        if self.pointer >= self.code.len() {
            return None;
        }

        // Newline
        if self.code[self.pointer..].starts_with('\n') {
            self.pointer += 1;
            return Some(Token::new(TokenType::Newline, line, column));
        }

        if self.pointer >= self.code.len() {
            return None;
        }

        let c = &self.code[self.pointer..];

        // Comments
        if c.starts_with("//") || c.starts_with("#") {
            while self.pointer < self.code.len() && !self.code[self.pointer..].starts_with('\n') {
                self.pointer += 1;
            }
            return Some(Token::new(TokenType::Newline, line, column));
        }

        if c.starts_with("/*") {
            self.pointer += 2;
            while self.pointer < self.code.len() && !self.code[self.pointer..].starts_with("*/") {
                self.pointer += 1;
            }
            if self.pointer < self.code.len() {
                self.pointer += 2; // Skip */
            }
            return Some(Token::new(TokenType::Newline, line, column));
        }

        // Null
        if c.starts_with("null") {
            self.pointer += 4;
            return Some(Token::new(TokenType::Value(Value::Null), line, column));
        }

        // Booleans
        if c.starts_with("true") {
            self.pointer += 4;
            return Some(Token::new(
                TokenType::Value(Value::Boolean(true)),
                line,
                column,
            ));
        }

        if c.starts_with("false") {
            self.pointer += 5;
            return Some(Token::new(
                TokenType::Value(Value::Boolean(false)),
                line,
                column,
            ));
        }

        // Strings
        if c.starts_with('"') {
            self.pointer += 1;
            let start = self.pointer;
            while self.pointer < self.code.len() && !self.code[self.pointer..].starts_with('"') {
                self.pointer += 1;
            }
            let string = &self.code[start..self.pointer]
                .replace("\\\"", "\"") // Unescape quotes
                .replace("\\n", "\n") // Unescape newlines
                .replace("\\t", "\t"); // Unescape tabs
            self.pointer += 1; // Skip closing "
            return Some(Token::new(
                TokenType::Value(Value::String(string.to_string())),
                line,
                column,
            ));
        }

        // Multi-char operators first
        let two = &self.code[self.pointer..self.pointer + 2.min(self.code.len() - self.pointer)];
        if [
            "+=", "-=", "*=", "/=", "==", "!=", "<=", ">=", "&&", "||", "..", "**", "<<", ">>",
            "++", "--",
        ]
        .contains(&two)
        {
            self.pointer += 2;
            return Some(Token::new(
                TokenType::Operator(two.to_string()),
                line,
                column,
            ));
        }

        // Single-char operators
        let one = &self.code[self.pointer..self.pointer + 1];
        if ["=", "+", "-", "*", "/", "%", "^", "&", "|", "<", ">", "!"].contains(&one) {
            self.pointer += 1;
            return Some(Token::new(
                TokenType::Operator(one.to_string()),
                line,
                column,
            ));
        }

        // Symbols
        if ["(", ")", "[", "]", "{", "}", ",", ":", "."].contains(&one) {
            self.pointer += 1;
            return Some(Token::new(TokenType::Symbol(one.to_string()), line, column));
        }

        // Numbers
        if c.starts_with("0x") || c.starts_with("0b") || c.starts_with("0o") {
            // Hex, binary, or octal numbers
            let base = if c.starts_with("0x") {
                16
            } else if c.starts_with("0b") {
                2
            } else {
                8
            };
            self.pointer += 2; // Skip 0x, 0b, or 0o
            let start = self.pointer;
            while self.pointer < self.code.len()
                && self.code[self.pointer..].starts_with(char::is_alphanumeric)
            {
                self.pointer += 1;
            }
            let number = &self.code[start..self.pointer];
            return Some(Token::new(
                TokenType::Value(Value::Number(
                    i64::from_str_radix(number, base).unwrap() as f32
                )),
                line,
                column,
            ));
        }

        if c.chars().next().unwrap().is_ascii_digit() {
            let number = self.get_number();
            return Some(Token::new(
                TokenType::Value(Value::Number(number.parse().unwrap())),
                line,
                column,
            ));
        }

        if c.starts_with('.') {
            self.pointer += 1;
            let number = self.get_number();
            return Some(Token::new(
                TokenType::Value(Value::Number(format!(".{}", number).parse().unwrap())),
                line,
                column,
            ));
        }

        // Identifiers or keywords
        if c.chars().next().unwrap().is_alphabetic() || c.starts_with('_') {
            let ident = self.get_identifier();
            if keyword_list.contains(&ident.as_str()) {
                return Some(Token::new(TokenType::Keyword(ident), line, column));
            } else {
                return Some(Token::new(TokenType::Identifier(ident), line, column));
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
        tokens.push(Token::new(
            TokenType::EOF,
            self.code.lines().count(),
            self.code.len() - self.code.rfind('\n').map(|pos| pos + 1).unwrap_or(0),
        ));
        tokens
    }
}
