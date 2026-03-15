/// Общий интерфейс для всех операторов сканирования (Volcano / Iterator model).
///
/// Порядок вызова:
/// 1. `before_first()` — сброс к началу
/// 2. Цикл: `next()` → если `true`, читаем поля через `get_*`
/// 3. `close()` — освобождение ресурсов
///
/// Этот трейт должны реализовывать все операторы: TableScan, SelectScan,
/// ProjectScan и т.д. Благодаря единому интерфейсу операторы можно
/// свободно комбинировать в дерево выполнения запроса.
use crate::error::Result;
use crate::value::Value;

pub trait Scan {
    /// Сбрасывает итератор перед первой записью.
    fn before_first(&mut self);

    /// Переходит к следующей записи. Возвращает `false`, если записи кончились.
    fn next(&mut self) -> bool;

    /// Возвращает целочисленное значение поля текущей записи.
    /// Ошибки: `FieldNotFound`, `TypeMismatch`.
    fn get_int(&self, field_name: &str) -> Result<i64>;

    /// Возвращает строковое значение поля текущей записи.
    /// Ошибки: `FieldNotFound`, `TypeMismatch`.
    fn get_string(&self, field_name: &str) -> Result<String>;

    /// Возвращает значение поля как `Value` (тип-независимый геттер).
    /// Ошибки: `FieldNotFound`.
    fn get_value(&self, field_name: &str) -> Result<Value>;

    /// Возвращает `true`, если поле доступно через данный оператор.
    /// У ProjectScan — только поля из списка проекции.
    fn has_field(&self, field_name: &str) -> bool;

    /// Освобождает ресурсы оператора.
    fn close(&mut self);
}
