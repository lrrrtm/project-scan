/// Оператор проекции — реализует выбор столбцов (аналог SELECT col1, col2 FROM ...).
///
/// Оборачивает любой нижележащий оператор (`Box<dyn Scan>`) и ограничивает
/// набор доступных полей списком `field_list`. Навигация и чтение данных
/// делегируются нижележащему оператору; при запросе поля вне `field_list`
/// возвращается `FieldNotFound`.
use crate::error::{DbError, Result};
use crate::scan::Scan;
use crate::value::Value;

pub struct ProjectScan {
    // Box<dyn Scan> позволяет хранить любой оператор: TableScan, SelectScan,
    // другой ProjectScan и т.д. — конкретный тип определяется в рантайме.
    base_scan: Box<dyn Scan>,
    field_list: Vec<String>,
}

impl ProjectScan {
    pub fn new(base_scan: Box<dyn Scan>, field_list: Vec<String>) -> Self {
        ProjectScan { base_scan, field_list }
    }

    fn check_field(&self, field_name: &str) -> Result<()> {
        if self.field_list.iter().any(|f| f == field_name) {
            Ok(())
        } else {
            Err(DbError::FieldNotFound(field_name.to_string()))
        }
    }
}

impl Scan for ProjectScan {
    fn before_first(&mut self) {
        self.base_scan.before_first();
    }

    fn next(&mut self) -> bool {
        self.base_scan.next()
    }

    fn get_int(&self, field_name: &str) -> Result<i64> {
        self.check_field(field_name)?;
        self.base_scan.get_int(field_name)
    }

    fn get_string(&self, field_name: &str) -> Result<String> {
        self.check_field(field_name)?;
        self.base_scan.get_string(field_name)
    }

    fn get_value(&self, field_name: &str) -> Result<Value> {
        self.check_field(field_name)?;
        self.base_scan.get_value(field_name)
    }

    fn has_field(&self, field_name: &str) -> bool {
        self.field_list.iter().any(|f| f == field_name)
    }

    fn close(&mut self) {
        self.base_scan.close();
    }
}
