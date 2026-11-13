# actix-web под капотом — демо к занятию

actix-web 4.17 · actix-http 3.19 · actix-server 2.10 · actix-ws 0.4 · Rust 1.97 (edition 2024).

## Структура

```
src/main.rs         HttpServer + App: воркеры, таймауты, h2c, тесты (#![forbid(unsafe_code)])
src/routes.rs       обработчики: состояние, блокирующий код, тело, стриминг, WebSocket
src/extract.rs      собственный экстрактор ClientId (FromRequest)
src/errors.rs       ApiError + ResponseError (thiserror)
src/middleware.rs   middleware двумя способами: from_fn и Transform/Service
src/lib.rs          #![deny(clippy::undocumented_unsafe_blocks)]
src/unsafe_demo.rs  unsafe как в actix-http: align_to для WS-маски, HeaderValue без проверки
examples/ws_client.rs    WebSocket-клиент на awc
examples/slow_client.rs  «медленный клиент» → 408
examples/mask_bench.rs   замер трёх реализаций маски
```

## Запуск

```sh
cargo test                          # 7 тестов
WORKERS=2 cargo run                 # сервер на 127.0.0.1:8080 (HTTP/1.1 + h2c)
```

## Маршруты

| Маршрут | Что показывает |
|---|---|
| `GET /hits` | round-robin по воркерам; `Data<T>` (общий) vs `Rc<Cell>` (на воркер) |
| `GET /fast` | быстрый ответ + имя потока |
| `GET /block-bad` | `std::thread::sleep` прямо в воркере — блокирует его |
| `GET /block-good` | то же через `web::block` (пул `spawn_blocking`) |
| `POST /notes` | экстрактор `ClientId` + `Json<Note>` с лимитом 1 KiB → 400/413 |
| `POST /upload` | потоковое чтение `Payload`, свой лимит 1 MiB → 413 |
| `GET /stream` | chunked-ответ, 5 «тиков» по 500 мс |
| `GET /proto` | версия протокола: HTTP/1.1 или HTTP/2.0 |
| `GET /ws` | WebSocket-эхо с heartbeat (actix-ws) |

## Команды для проверки

```sh
for i in 1 2 3 4; do curl -s localhost:8080/hits; echo; done
curl -si localhost:8080/fast
curl -sv localhost:8080/fast localhost:8080/fast 2>&1 | grep -E "Connected|Re-using"
curl -s localhost:8080/proto; curl -s --http2-prior-knowledge localhost:8080/proto
curl -s -X POST localhost:8080/notes -H 'content-type: application/json' -d '{"text":"hi"}'
curl -s -X POST localhost:8080/notes -H 'x-client-id: alice' -H 'content-type: application/json' -d '{"text":"hi"}'
head -c 2000000 /dev/urandom | curl -s -X POST localhost:8080/upload --data-binary @-
curl -N localhost:8080/stream
cargo run --example slow_client
cargo run --example ws_client
cargo run --release --example mask_bench
```

Блокировка воркера нагляднее с одним воркером: `WORKERS=1 cargo run`, затем
`curl localhost:8080/block-bad & sleep 0.3; curl -w '%{time_total}\n' localhost:8080/fast`.

Остановка: `kill -TERM <pid>` — мягкая (дожидается активных запросов), Ctrl+C (SIGINT) — принудительная.

Miri (нужен nightly): `rustup +nightly component add miri && cargo +nightly miri test --lib`.
