/// Заглушка (mock) для тестирования — хранит данные в памяти.
///
/// Используется вместо реального TableScan (который читает с диска).
/// При интеграции с настоящим TableScan — просто замените MockTableScan
/// на него: интерфейс трейта `Scan` одинаковый.
use std::collections::HashMap;

use crate::error::{DbError, Result};
use crate::scan::Scan;
use crate::value::Value;

pub struct MockTableScan {
    rows: Vec<HashMap<String, Value>>,
    // -1 означает «перед первой записью», до первого вызова next()
    current: i64,
}

impl MockTableScan {
    pub fn new(rows: Vec<HashMap<String, Value>>) -> Self {
        MockTableScan { rows, current: -1 }
    }

    fn get_current_value(&self, field_name: &str) -> Result<&Value> {
        if self.current < 0 || self.current as usize >= self.rows.len() {
            return Err(DbError::Other(
                "Итератор не указывает на запись. Вызовите next() перед чтением.".to_string(),
            ));
        }
        let row = &self.rows[self.current as usize];
        row.get(field_name)
            .ok_or_else(|| DbError::FieldNotFound(field_name.to_string()))
    }
}

impl Scan for MockTableScan {
    fn before_first(&mut self) {
        self.current = -1;
    }

    fn next(&mut self) -> bool {
        self.current += 1;
        (self.current as usize) < self.rows.len()
    }

    fn get_int(&self, field_name: &str) -> Result<i64> {
        match self.get_current_value(field_name)? {
            Value::Int(n) => Ok(*n),
            Value::Str(_) => Err(DbError::TypeMismatch {
                field: field_name.to_string(),
                expected: "Int".to_string(),
                got: "String".to_string(),
            }),
        }
    }

    fn get_string(&self, field_name: &str) -> Result<String> {
        match self.get_current_value(field_name)? {
            Value::Str(s) => Ok(s.clone()),
            Value::Int(_) => Err(DbError::TypeMismatch {
                field: field_name.to_string(),
                expected: "String".to_string(),
                got: "Int".to_string(),
            }),
        }
    }

    fn get_value(&self, field_name: &str) -> Result<Value> {
        self.get_current_value(field_name).cloned()
    }

    // has_field определяется по первой строке таблицы
    fn has_field(&self, field_name: &str) -> bool {
        self.rows.first().map_or(false, |row| row.contains_key(field_name))
    }

    fn close(&mut self) {}
}
