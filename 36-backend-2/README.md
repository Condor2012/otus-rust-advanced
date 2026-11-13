# backend2-demo — демо к занятию «Rust. Backend #2»

Одна предметная область (задачи) — четыре способа к ней обратиться.

| Крейт | Что внутри | Запуск |
|---|---|---|
| `grpc` | tonic 0.14: unary + серверный стрим | `cargo run -p tasks-grpc --bin grpc-server`, затем `--bin grpc-client` |
| `graphql` | async-graphql 7.2 + axum: query, mutation, subscription | `cargo run -p tasks-graphql` → http://localhost:8000 (GraphiQL) |
| `ws` | axum WebSocket-чат на broadcast + клиент на tokio-tungstenite | `cargo run -p tasks-ws`, затем в двух терминалах `cargo run -p tasks-ws --bin ws-client alice` |
| `bot` | teloxide 0.17: команды, общее состояние, диалог | `TELOXIDE_TOKEN=... cargo run -p tasks-bot` (токен у @BotFather) |

protoc ставить не нужно: `grpc/build.rs` берёт бинарник из `protoc-bin-vendored`.

## GraphQL: примеры

```graphql
mutation { addTask(title: "Изучить async-graphql") { id title done } }
query    { tasks(done: false) { id title } }
subscription { taskAdded { id title } }   # во вкладке GraphiQL, параллельно с mutation
```

```bash
curl localhost:8000 -H 'content-type: application/json' \
  -d '{"query":"{ tasks { id title done } }"}'
```

## Тесты бота

`cargo test -p tasks-bot` — разбор команд и хранилище без обращения к Telegram.
