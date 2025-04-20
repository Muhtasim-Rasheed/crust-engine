#[derive(Clone)]
pub enum Value {
    Null,
    Number(f32),
    String(String),
    Boolean(bool),
    List(Vec<Value>),
}

impl Value {
    pub fn to_number(&self) -> f32 {
        match self {
            Value::Number(n) => *n,
            Value::String(s) => s.parse().unwrap_or(0.0),
            Value::Boolean(b) => if *b { 1.0 } else { 0.0 },
            _ => 0.0,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Value::Null => "null".to_string(),
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.clone(),
            Value::Boolean(b) => b.to_string(),
            Value::List(l) => {
                let mut string = String::new();
                for (i, item) in l.iter().enumerate() {
                    string.push_str(&item.to_string());
                    if i < l.len() - 1 {
                        string.push_str(", ");
                    }
                }
                string
            }
        }
    }

    pub fn to_boolean(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            _ => false,
        }
    }

    pub fn to_list(&self) -> Vec<Value> {
        match self {
            Value::List(l) => l.clone(),
            Value::String(s) => s.chars()
                .map(|c| Value::String(c.to_string()))
                .collect(),
            _ => vec![self.clone()],
        }
    }

    pub fn change_by(&mut self, value: Value) {
        match (self, value) {
            (Value::Null, Value::Null) => {}
            (Value::Number(n), Value::Number(v)) => *n += v,
            (Value::String(s), Value::String(v)) => s.push_str(&v),
            (Value::Boolean(b), Value::Boolean(v)) => *b = *b || v,
            (Value::List(l), Value::List(v)) => l.extend(v),
            _ => panic!("Incompatible types for change_by"),
        }
    }
}
