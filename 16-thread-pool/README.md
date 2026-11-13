# Занятие 16. Пишем ThreadPool

Реализация пула потоков и actor-фреймворка на чистом Rust (кроме `rayon` для сравнения в примере `find`).

## Что внутри

- **`ThreadPool`** (`src/lib.rs`) — пул потоков на `std::sync::mpsc`. Задачи отправляются через канал, потоки забирают их через `Mutex<Receiver>`. Корректно дожидается завершения потоков при `Drop`.
- **`Actor` + `System`** (`src/actor.rs`) — минимальный actor-фреймворк: каждый актор живёт в собственном потоке и обрабатывает сообщения из канала. Возврат `None` из `process_message` останавливает актора.

## Примеры

```bash
cargo run --example barrier
cargo run --example find
cargo run --example phantom_data
cargo run --example ping_pong
cargo run --example send_sync
cargo run --example weak_strong
```

| Пример | Что демонстрирует |
|---|---|
| `ping_pong` | Обмен сообщениями между акторами |
| `barrier` | Синхронизация потоков |
| `send_sync` | Трейты `Send`/`Sync` на практике |
| `weak_strong` | `Rc`/`Arc`, слабые и сильные ссылки |
| `phantom_data` | `PhantomData` и маркерные типы |
| `find` | Параллельный поиск: однопоточный, на потоках, на пуле, на rayon |
