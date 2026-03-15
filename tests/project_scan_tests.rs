/// Интеграционные тесты для оператора `ProjectScan`.
///
/// Проверяют корректность проекции полей, обработку ошибок,
/// работу с пустой таблицей и цепочку операторов.
use std::collections::HashMap;

use project_scan::{DbError, MockTableScan, ProjectScan, Scan, Value};

/// Вспомогательная функция: создаёт тестовую таблицу студентов
/// с четырьмя полями: имя, возраст, группа, средний_балл.
fn make_students_table() -> MockTableScan {
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
    ];
    MockTableScan::new(rows)
}

/// Тест: проекция подмножества полей (2 из 4).
/// Эквивалент SQL: SELECT имя, группа FROM студенты
#[test]
fn test_projection_subset() {
    let table = make_students_table();
    let mut scan = ProjectScan::new(
        Box::new(table),
        vec!["имя".to_string(), "группа".to_string()],
    );

    // Первая запись
    assert!(scan.next());
    assert_eq!(scan.get_string("имя").unwrap(), "Алиса");
    assert_eq!(scan.get_string("группа").unwrap(), "ИВТ-21");

    // Вторая запись
    assert!(scan.next());
    assert_eq!(scan.get_string("имя").unwrap(), "Борис");
    assert_eq!(scan.get_string("группа").unwrap(), "ИВТ-22");

    // Третья запись
    assert!(scan.next());
    assert_eq!(scan.get_string("имя").unwrap(), "Вера");
    assert_eq!(scan.get_string("группа").unwrap(), "ИВТ-21");

    // Записи кончились
    assert!(!scan.next());

    scan.close();
}

/// Тест: `has_field` возвращает `false` для непроецированных полей.
#[test]
fn test_has_field_hidden() {
    let table = make_students_table();
    let scan = ProjectScan::new(
        Box::new(table),
        vec!["имя".to_string(), "группа".to_string()],
    );

    // Проецированные поля — доступны
    assert!(scan.has_field("имя"));
    assert!(scan.has_field("группа"));

    // Непроецированные поля — недоступны
    assert!(!scan.has_field("возраст"));
    assert!(!scan.has_field("средний_балл"));
    assert!(!scan.has_field("несуществующее"));
}

/// Тест: `get_int` на непроецированном поле возвращает `FieldNotFound`.
#[test]
fn test_get_int_field_not_found() {
    let table = make_students_table();
    let mut scan = ProjectScan::new(
        Box::new(table),
        vec!["имя".to_string(), "группа".to_string()],
    );

    scan.next();

    let result = scan.get_int("возраст");
    assert!(result.is_err());
    match result.unwrap_err() {
        DbError::FieldNotFound(name) => assert_eq!(name, "возраст"),
        other => panic!("Ожидалась ошибка FieldNotFound, получена: {other}"),
    }
}

/// Тест: `get_string` на непроецированном поле возвращает `FieldNotFound`.
#[test]
fn test_get_string_field_not_found() {
    let table = make_students_table();
    let mut scan = ProjectScan::new(
        Box::new(table),
        vec!["возраст".to_string()],
    );

    scan.next();

    let result = scan.get_string("имя");
    assert!(result.is_err());
    match result.unwrap_err() {
        DbError::FieldNotFound(name) => assert_eq!(name, "имя"),
        other => panic!("Ожидалась ошибка FieldNotFound, получена: {other}"),
    }
}

/// Тест: пустая таблица (0 записей). `next()` сразу возвращает `false`.
#[test]
fn test_empty_table() {
    let table = MockTableScan::new(vec![]);
    let mut scan = ProjectScan::new(
        Box::new(table),
        vec!["имя".to_string()],
    );

    assert!(!scan.next());
    scan.close();
}

/// Тест: проекция одного поля.
#[test]
fn test_single_field_projection() {
    let table = make_students_table();
    let mut scan = ProjectScan::new(
        Box::new(table),
        vec!["средний_балл".to_string()],
    );

    let mut scores = vec![];
    while scan.next() {
        scores.push(scan.get_int("средний_балл").unwrap());
    }
    assert_eq!(scores, vec![95, 82, 91]);

    // Остальные поля недоступны
    assert!(!scan.has_field("имя"));
    assert!(!scan.has_field("возраст"));
    assert!(!scan.has_field("группа"));

    scan.close();
}

/// Тест: проекция всех полей (эквивалент SELECT *).
#[test]
fn test_all_fields_projection() {
    let table = make_students_table();
    let mut scan = ProjectScan::new(
        Box::new(table),
        vec![
            "имя".to_string(),
            "возраст".to_string(),
            "группа".to_string(),
            "средний_балл".to_string(),
        ],
    );

    assert!(scan.next());
    assert_eq!(scan.get_string("имя").unwrap(), "Алиса");
    assert_eq!(scan.get_int("возраст").unwrap(), 20);
    assert_eq!(scan.get_string("группа").unwrap(), "ИВТ-21");
    assert_eq!(scan.get_int("средний_балл").unwrap(), 95);

    // Все поля доступны
    assert!(scan.has_field("имя"));
    assert!(scan.has_field("возраст"));
    assert!(scan.has_field("группа"));
    assert!(scan.has_field("средний_балл"));

    scan.close();
}

/// Тест: цепочка `ProjectScan` поверх другого `ProjectScan`.
/// Сначала проецируем 3 поля, затем из них — только 1.
#[test]
fn test_chained_projections() {
    let table = make_students_table();

    // Первый уровень: SELECT имя, возраст, группа
    let first = ProjectScan::new(
        Box::new(table),
        vec![
            "имя".to_string(),
            "возраст".to_string(),
            "группа".to_string(),
        ],
    );

    // Второй уровень: SELECT имя (из результата первого)
    let mut second = ProjectScan::new(
        Box::new(first),
        vec!["имя".to_string()],
    );

    // Поле «имя» доступно, остальные — нет
    assert!(second.has_field("имя"));
    assert!(!second.has_field("возраст"));
    assert!(!second.has_field("группа"));
    assert!(!second.has_field("средний_балл"));

    // Итерируемся и проверяем значения
    let mut names = vec![];
    while second.next() {
        names.push(second.get_string("имя").unwrap());
    }
    assert_eq!(names, vec!["Алиса", "Борис", "Вера"]);

    // Непроецированное поле из первого уровня — ошибка
    second.before_first();
    second.next();
    let result = second.get_int("возраст");
    assert!(matches!(result, Err(DbError::FieldNotFound(_))));

    second.close();
}

/// Тест: `before_first()` сбрасывает итератор, позволяя пройтись заново.
#[test]
fn test_before_first_reset() {
    let table = make_students_table();
    let mut scan = ProjectScan::new(
        Box::new(table),
        vec!["имя".to_string()],
    );

    // Первый проход
    let mut count = 0;
    while scan.next() {
        count += 1;
    }
    assert_eq!(count, 3);

    // Сброс и второй проход
    scan.before_first();
    let mut count = 0;
    while scan.next() {
        count += 1;
    }
    assert_eq!(count, 3);

    scan.close();
}

/// Тест: `get_value` возвращает корректные значения `Value`.
#[test]
fn test_get_value() {
    let table = make_students_table();
    let mut scan = ProjectScan::new(
        Box::new(table),
        vec!["имя".to_string(), "возраст".to_string()],
    );

    scan.next();

    assert_eq!(
        scan.get_value("имя").unwrap(),
        Value::Str("Алиса".to_string())
    );
    assert_eq!(scan.get_value("возраст").unwrap(), Value::Int(20));

    // Непроецированное поле
    assert!(scan.get_value("группа").is_err());

    scan.close();
}
