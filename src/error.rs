use thiserror::Error;

/// Типы ошибок, возникающих при работе операторов сканирования.
#[derive(Debug, Error)]
pub enum DbError {
    /// Запрошено поле, которого нет в данном операторе (например, вне списка проекции).
    #[error("Поле не найдено: {0}")]
    FieldNotFound(String),

    /// Вызван get_int() для строкового поля или get_string() для числового.
    #[error("Несоответствие типов для поля '{field}': ожидалось {expected}, получено {got}")]
    TypeMismatch { field: String, expected: String, got: String },

    #[error("Ошибка ввода-вывода: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Ошибка: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, DbError>;
