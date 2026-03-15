/// Оператор ProjectScan для учебной СУБД.
///
/// Публичный интерфейс для интеграции с другими компонентами:
///   - `Scan`          — трейт, который должны реализовывать все операторы
///   - `ProjectScan`   — оператор проекции (SELECT col1, col2 FROM ...)
///   - `Value`         — тип значения поля (Int / Str)
///   - `DbError`       — тип ошибки, общий для всех операторов
///   - `MockTableScan` — заглушка; замените на реальный TableScan при интеграции
pub mod error;
pub mod value;
pub mod scan;
pub mod project_scan;
pub mod mock_scan;

pub use error::{DbError, Result};
pub use mock_scan::MockTableScan;
pub use project_scan::ProjectScan;
pub use scan::Scan;
pub use value::Value;
