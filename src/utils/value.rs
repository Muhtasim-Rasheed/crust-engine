use std::collections::HashMap;

use crate::utils::Callable;

#[derive(Clone, PartialEq)]
pub enum Value {
    Null,
    Number(f32),
    String(String),
    Boolean(bool),
    List(Vec<Value>),
    Object(HashMap<String, Value>),
    Closure(Box<Callable>),
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::List(l) => {
                let mut string = String::new();
                for (i, item) in l.iter().enumerate() {
                    string.push_str(&item.to_string());
                    if i < l.len() - 1 {
                        string.push_str(", ");
                    }
                }
                write!(f, "[{}]", string)
            }
            Value::Object(o) => {
                let mut string = String::new();
                for (key, value) in o.iter() {
                    string.push_str(&format!("{}: {}, ", key, value.to_string()));
                }
                if !string.is_empty() {
                    string.pop();
                    string.pop();
                }
                write!(f, "{{ {} }}", string)
            }
            Value::Closure(c) => {
                let mut string = String::new();
                match **c {
                    Callable::Builtin(_) => {
                        string.push_str("{ builtin }");
                    }
                    Callable::Function(ref f) => {
                        string.push_str("(");
                        for (i, arg) in f.args.iter().enumerate() {
                            string.push_str(&arg.to_string());
                            if i < f.args.len() - 1 {
                                string.push_str(", ");
                            }
                        }
                        string.push_str(") ");
                        string.push_str(&f.returns.to_string().as_str());
                        string.push_str(" { ... }");
                    }
                }
                write!(f, "{}", string)
            }
        }
    }
}

impl Value {
    pub fn to_number(&self) -> f32 {
        match self {
            Value::Number(n) => *n,
            Value::String(s) => s.parse().unwrap_or(0.0),
            Value::Boolean(b) => {
                if *b {
                    1.0
                } else {
                    0.0
                }
            }
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
                string.push_str("[");
                for (i, item) in l.iter().enumerate() {
                    string.push_str(&item.to_string());
                    if i < l.len() - 1 {
                        string.push_str(", ");
                    }
                }
                string.push_str("]");
                string
            }
            Value::Object(o) => {
                format!(
                    "{{ {} }}",
                    o.iter()
                        .map(|(k, v)| format!("{}: {}", k, v.to_string()))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Value::Closure(c) => match **c {
                Callable::Builtin(_) => "(..) ? -> { builtin }".to_string(),
                Callable::Function(ref f) => {
                    let args = f
                        .args
                        .iter()
                        .map(|arg| arg.to_string())
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("({}) -> {}", args, f.returns.to_string())
                }
            },
        }
    }

    pub fn to_boolean(&self) -> bool {
        match self {
            Value::Null => false,
            Value::Boolean(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::List(l) => !l.is_empty(),
            Value::Object(o) => !o.is_empty(),
            Value::Closure(c) => match **c {
                Callable::Builtin(_) => true,
                Callable::Function(ref f) => !f.body.is_empty(),
            },
        }
    }

    pub fn to_list(&self) -> Vec<Value> {
        match self {
            Value::Null => vec![],
            Value::List(l) => l.clone(),
            Value::String(s) => s.chars().map(|c| Value::String(c.to_string())).collect(),
            Value::Object(o) => o
                .iter()
                .map(|(k, v)| Value::List(vec![Value::String(k.clone()), v.clone()]))
                .collect(),
            _ => vec![self.clone()],
        }
    }

    pub fn to_object(&self) -> HashMap<String, Value> {
        match self {
            Value::Null => HashMap::new(),
            Value::Object(o) => o.clone(),
            Value::List(l) => l
                .iter()
                .enumerate()
                .map(|(i, v)| (i.to_string(), v.clone()))
                .collect(),
            _ => HashMap::new(),
        }
    }
}
