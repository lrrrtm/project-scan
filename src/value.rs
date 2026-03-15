use std::fmt;

/// Значение поля записи. Поддерживаемые типы: целое число и строка.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Str(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{n}"),
            Value::Str(s) => write!(f, "{s}"),
        }
    }
}
