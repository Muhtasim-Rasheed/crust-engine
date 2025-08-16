use crate::utils::Value;

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
                | self.code[self.pointer..].starts_with('.')
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
        if c.starts_with("//") || c.starts_with("#") {
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
            let string = &self.code[start..self.pointer]
                .replace("\\\"", "\"") // Unescape quotes
                .replace("\\n", "\n") // Unescape newlines
                .replace("\\t", "\t"); // Unescape tabs
            self.pointer += 1; // Skip closing "
            return Some(Token::Value(Value::String(string.to_string())));
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
            return Some(Token::Operator(two.to_string()));
        }

        // Single-char operators
        let one = &self.code[self.pointer..self.pointer + 1];
        if ["=", "+", "-", "*", "/", "%", "^", "&", "|", "<", ">", "!"].contains(&one) {
            self.pointer += 1;
            return Some(Token::Operator(one.to_string()));
        }

        // Symbols
        if ["(", ")", "[", "]", "{", "}", ",", ":", "."].contains(&one) {
            self.pointer += 1;
            return Some(Token::Symbol(one.to_string()));
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
            return Some(Token::Value(Value::Number(
                i64::from_str_radix(number, base).unwrap() as f32,
            )));
        }

        if c.chars().next().unwrap().is_ascii_digit() {
            let number = self.get_number();
            return Some(Token::Value(Value::Number(number.parse().unwrap())));
        }

        if c.chars().next().unwrap() == '.' {
            self.pointer += 1;
            let number = self.get_number();
            return Some(Token::Value(Value::Number(
                format!(".{}", number).parse().unwrap(),
            )));
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

