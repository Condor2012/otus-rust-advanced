# Занятие 35. Backend #1

Три реализации одного и того же REST API (доски задач) на разных веб-фреймворках Rust — для сравнения подходов:

| Crate | Фреймворк | База данных |
|---|---|---|
| `actix_ex` | actix-web 4 | MongoDB |
| `axum_ex` | axum 0.7 | MySQL (sqlx) |
| `rocket_ex` | rocket 0.5 | MySQL (sqlx) |

API одинаковое у всех: CRUD досок (`/board`) и задач (`/board/{id}/task`), middleware со счётчиком обращений, обработка ошибок через `thiserror`.

## Настройка

```bash
cp .env.example .env
```

Поднимите нужную базу (см. readme внутри каждого crate) и при необходимости поправьте `.env`.

## Запуск

```bash
cargo run -p actix_ex
# или
cargo run -p axum_ex
# или
cargo run -p rocket_ex
```

## Проверка API

Готовые curl-скрипты лежат в `requests/`:

```bash
sh requests/create_board.sh "Моя доска"
sh requests/list_boards.sh
```
