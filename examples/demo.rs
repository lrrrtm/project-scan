/// Пример использования `ProjectScan`.
///
/// Создаём таблицу студентов с четырьмя полями, оборачиваем её
/// в `ProjectScan` с полями ["имя", "группа"] и выводим результат.
/// Аналог SQL: SELECT имя, группа FROM студенты
use std::collections::HashMap;

use project_scan::{MockTableScan, ProjectScan, Scan, Value};

fn main() {
    // Создаём данные таблицы «студенты»
    let rows = vec![
        HashMap::from([
            ("имя".to_string(), Value::Str("Алиса".to_string())),
            ("возраст".to_string(), Value::Int(20)),
            ("группа".to_string(), Value::Str("ИВТ-21".to_string())),
            ("средний_балл".to_string(), Value::Int(95)),
        ]),
        HashMap::from([
            ("имя".to_string(), Value::Str("Борис".to_string())),
            ("возраст".to_string(), Value::Int(21)),
            ("группа".to_string(), Value::Str("ИВТ-22".to_string())),
            ("средний_балл".to_string(), Value::Int(82)),
        ]),
        HashMap::from([
            ("имя".to_string(), Value::Str("Вера".to_string())),
            ("возраст".to_string(), Value::Int(19)),
            ("группа".to_string(), Value::Str("ИВТ-21".to_string())),
            ("средний_балл".to_string(), Value::Int(91)),
        ]),
        HashMap::from([
            ("имя".to_string(), Value::Str("Григорий".to_string())),
            ("возраст".to_string(), Value::Int(22)),
            ("группа".to_string(), Value::Str("ИВТ-22".to_string())),
            ("средний_балл".to_string(), Value::Int(78)),
        ]),
    ];

    // Создаём мок-таблицу
    let table = MockTableScan::new(rows);

    // Оборачиваем в ProjectScan — оставляем только «имя» и «группа»
    // Эквивалент SQL: SELECT имя, группа FROM студенты
    let mut scan = ProjectScan::new(
        Box::new(table),
        vec!["имя".to_string(), "группа".to_string()],
    );

    println!("=== Результат проекции: SELECT имя, группа FROM студенты ===");
    println!("{:<15} {:<10}", "Имя", "Группа");
    println!("{}", "-".repeat(25));

    // Итерируемся по записям и выводим результат
    while scan.next() {
        let name = scan.get_string("имя").unwrap();
        let group = scan.get_string("группа").unwrap();
        println!("{:<15} {:<10}", name, group);
    }

    println!();

    // Демонстрация: поле «возраст» недоступно через проекцию
    println!("Поле 'возраст' доступно? {}", scan.has_field("возраст"));
    println!("Поле 'имя' доступно? {}", scan.has_field("имя"));

    // Освобождаем ресурсы
    scan.close();
}
