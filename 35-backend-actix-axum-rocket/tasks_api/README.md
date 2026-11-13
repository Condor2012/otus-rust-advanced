# tasks_api — демо к занятию «Rust. Backend #1»

axum 0.8 + sqlx 0.9 + PostgreSQL 16. CRUD по задачам.

```bash
docker compose up -d                 # PostgreSQL
cargo install sqlx-cli --no-default-features --features postgres,rustls
sqlx migrate run                     # создать таблицу (нужна для query! на этапе компиляции)
cargo run                            # http://localhost:3000
```

Без БД при сборке: `SQLX_OFFLINE=true cargo build` (используется кэш `.sqlx/`, обновлять: `cargo sqlx prepare`).

```bash
curl -X POST localhost:3000/tasks -H 'content-type: application/json' -d '{"title":"Прочитать доку axum"}'
curl localhost:3000/tasks?done=false
curl -X PATCH localhost:3000/tasks/1 -H 'content-type: application/json' -d '{"done":true}'
curl -X DELETE localhost:3000/tasks/1
```
