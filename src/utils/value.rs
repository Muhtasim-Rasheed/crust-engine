use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::utils::Callable;

#[derive(Clone, PartialEq)]
pub enum Value {
    Null,
    Number(f32),
    String(String),
    Boolean(bool),
    List(Rc<RefCell<Vec<Rc<RefCell<Value>>>>>),
    Object(Rc<RefCell<HashMap<String, Rc<RefCell<Value>>>>>),
    Closure(Rc<Callable>),
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::List(l) => {
                let string = l
                    .borrow()
                    .iter()
                    .map(|item| item.borrow().to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "[{}]", string)
            }
            Value::Object(o) => {
                let mut string = String::new();
                for (key, value) in o.borrow().iter() {
                    string.push_str(&format!("{}: {}, ", key, value.borrow().to_string()));
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
                        string.push('(');
                        for (i, arg) in f.args.iter().enumerate() {
                            string.push_str(&arg.to_string());
                            if i < f.args.len() - 1 {
                                string.push_str(", ");
                            }
                        }
                        string.push_str(") ");
                        string.push_str(f.returns.to_string().as_str());
                        string.push_str(" { ... }");
                    }
                }
                write!(f, "{}", string)
            }
        }
    }
}

impl Value {
    pub fn list(items: Vec<Value>) -> Self {
        Value::List(Rc::new(RefCell::new(
            items.into_iter().map(|v| Rc::new(RefCell::new(v))).collect(),
        )))
    }

    pub fn object(pairs: HashMap<String, Value>) -> Self {
        Value::Object(Rc::new(RefCell::new(
            pairs
                .into_iter()
                .map(|(k, v)| (k, Rc::new(RefCell::new(v))))
                .collect(),
        )))
    }

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
                let string = l
                    .borrow()
                    .iter()
                    .map(|item| item.borrow().to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("[{}]", string)
            }
            Value::Object(o) => {
                format!(
                    "{{ {} }}",
                    o.borrow()
                        .iter()
                        .map(|(k, v)| format!("{}: {}", k, v.borrow().to_string()))
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
                    format!("({}) -> {}", args, f.returns)
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
            Value::List(l) => !l.borrow().is_empty(),
            Value::Object(o) => !o.borrow().is_empty(),
            Value::Closure(c) => match **c {
                Callable::Builtin(_) => true,
                Callable::Function(ref f) => !f.body.is_empty(),
            },
        }
    }

    pub fn to_list(&self) -> Vec<Rc<RefCell<Value>>> {
        match self {
            Value::Null => vec![],
            Value::List(l) => l.borrow().clone(),
            Value::String(s) => s
                .chars()
                .map(|c| Rc::new(RefCell::new(Value::String(c.to_string()))))
                .collect(),

            Value::Object(o) => o
                .borrow()
                .iter()
                .map(|(k, v)| {
                    Rc::new(RefCell::new(Value::List(Rc::new(RefCell::new(vec![
                        Rc::new(RefCell::new(Value::String(k.clone()))),
                        v.clone(),
                    ])))))
                })
                .collect(),
            _ => vec![Rc::new(RefCell::new(self.clone()))],
        }
    }

    pub fn to_object(&self) -> HashMap<String, Rc<RefCell<Value>>> {
        match self {
            Value::Null => HashMap::new(),
            Value::Object(o) => o.borrow().clone(),
            Value::List(l) => l
                .borrow()
                .iter()
                .enumerate()
                .map(|(i, v)| (i.to_string(), v.clone()))
                .collect(),
            _ => HashMap::new(),
        }
    }
}
