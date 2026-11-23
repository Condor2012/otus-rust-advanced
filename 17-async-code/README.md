# Занятие 17. Асинхронный код

Материалы по презентации «Асинхронный код»: кооперативная многозадачность, `Future`, `async`/`await`, ручной `poll`, учебный executor и Tokio.

## Структура

- `src/timer.rs` — `Pend3TimesFuture`, который трижды возвращает `Pending`, и `TimerFuture` с готовностью по `Instant`.
- `src/executor.rs` — однопоточный executor: хранит `Pin<Box<dyn Future<Output = ()>>>`, опрашивает задачи и удаляет завершённые.
- `examples/manual_poll.rs` — ручной четырёхкратный `poll` асинхронного блока с noop-waker.
- `examples/executor.rs` — три асинхронных клиента, каждый последовательно читает запрос, отправляет ответ и читает второй запрос.
- `examples/tokio.rs` — `tokio::join!` ждёт все задержки, а `tokio::select!` завершается после первой и отменяет остальные.

Учебный executor использует активный повторный опрос и noop-waker, как в исходнике занятия. Он предназначен для демонстрации устройства `Future`, а не для production: реальный исполнитель должен реагировать на `Waker` без постоянного опроса спящих задач.

## Запуск без зависимостей

```bash
cargo run --example manual_poll
cargo run --example executor
```

`manual_poll` выводит `Hello`, четыре строки `Poll 0..3 times` и `Hello again`. Пример executor выполняет три клиентские задачи конкурентно на одном потоке; порядок их сообщений может отличаться.

## Пример Tokio

Tokio подключён как optional-зависимость, поэтому базовые учебные примеры не требуют внешних пакетов:

```bash
cargo run --example tokio --features tokio
```

При задержках 100, 200 и 300 мс `join!` занимает около 300 мс, а `select!` — около 100 мс.

## Проверка

```bash
cargo test --all-features
cargo clippy --all-targets --all-features -- -D warnings
```

## Исходники из презентации

- [Ручной poll в Compiler Explorer](https://godbolt.org/z/K4f79zPKn).
- [Учебный executor в Compiler Explorer](https://godbolt.org/z/G597M4vnG).
- [Реализация `futures::executor::block_on`](https://docs.rs/futures-executor/0.3.32/src/futures_executor/local_pool.rs.html#80).
- [Примеры Tokio](https://docs.rs/tokio/latest/tokio/index.html#examples).
