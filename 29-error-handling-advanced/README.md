# Занятие 29. Обработка ошибок. Углублённое погружение

Рабочий пример иерархии ошибок и дополнительной классификации ошибок для логирования и повторных попыток.

- `src/errors.rs` — `thiserror`, `#[non_exhaustive]`, преобразования через `From`, методы `retryable` и `severity_level`.
- `src/main.rs` — сервис пользователей, `?` в `main` и panic-hook с сохранением стандартного обработчика.

## Запуск и проверка

```bash
cargo run
cargo test
cargo clippy --all-targets -- -D warnings
```
