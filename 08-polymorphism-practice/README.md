# Занятие 8. Статический и динамический полиморфизм. Практика

Практические примеры из обеих лекций: замыкания и функции высшего порядка, Copy/Clone, система событий и trait upcasting.

## Примеры

- `closures` — решение задания `apply` из Rust Playground, обобщённый `map`, повторное использование копируемого `Fn`-замыкания, возврат `impl Fn` и `Box<dyn Fn>`.
- `events` — `EventKind: Copy`, `Event: Clone`, обработка через `FnOnce`, накопление состояния через `FnMut` и хранение обработчиков `Box<dyn Fn(Event)>` в `EventBus`.
- `upcasting` — нативное преобразование `&dyn Pet` в `&dyn Animal`, доступное начиная с Rust 1.86.

Статический dispatch используется в обобщённых `apply`, `map`, `handle_once` и `process_stream`. Динамический dispatch нужен `EventBus` для хранения разных замыканий в одной коллекции и функции `returns_closure` для возврата замыканий разных типов.

## Запуск

```bash
cargo run --example closures
cargo run --example events
cargo run --example upcasting
cargo test
```

Тесты включают все три проверки исходного задания `apply`, различие Copy/Clone, накопление состояния обработчиком и доставку события через трейт-объект.
