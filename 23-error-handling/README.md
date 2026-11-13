# Занятие 23. Обработка ошибок

Демо-пример к занятию «Обработка ошибок».

## Что показано

- Собственный `enum Error` (`DivisionByZero`, `NegativeNumber`) с ручной реализацией `ToString` — прообраз будущих `std::error::Error` + `Display`.
- Разбор устройства `Option<T>` и `Result<T, E>` (закомментированные «скелеты» enum'ов).
- Возврат `Result<String, Error>` из функции `do_some_work` и обработка через `if let Ok(..)` (варианты с `match` и `unwrap_or` — в комментариях).
- Функция `get_user_id` -> `Option<String>`: поиск в `HashMap` и особый случай `"admin"`.

## Запуск

```bash
cargo run
```
