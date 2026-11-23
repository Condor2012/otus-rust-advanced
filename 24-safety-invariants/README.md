# Занятие 24. Инварианты безопасности

Примеры по презентации «Инварианты безопасности»: контракты `unsafe`, локализация небезопасных операций и проектирование API, в котором некорректные состояния невозможно выразить.

## Структура

- `src/safety.rs` — `unsafe fn` с явным контрактом чётности, безопасные обёртки с проверкой условия и локализованный `get_unchecked` после проверки индекса.
- `src/contracts.rs` — пример из Compiler Explorer: `NonZero<i32>` в `Param1` и `Param2`, а также typestate-билдер. Метод `with_param2` доступен только после `with_param1`, поэтому нарушить порядок инициализации через публичный API нельзя.
- `src/states.rs` — состояние персонажа через `enum`: каждый вариант хранит только относящиеся к нему поля, вместо набора независимых `bool` и `f32`.
- `examples/` — три самостоятельные демонстрации.

Внешний пример содержал показательную опечатку: `set_param2` присваивал значение полю `param1`. В typestate-версии оба параметра сохраняются в собственные поля; `Configured` можно получить только после полной настройки.

Небезопасные операции находятся внутри небольших функций и сопровождаются комментариями `SAFETY`. Примеры не выполняют код с нарушенными инвариантами: вызывать UB для демонстрации ошибки не требуется.

## Запуск

```bash
cargo run --example unsafe_contracts
cargo run --example typestate
cargo run --example character_states
```

## Проверка

```bash
cargo test
cargo clippy --all-targets -- -D warnings
```

## Материалы

- [Исходный пример в Compiler Explorer](https://godbolt.org/z/fbq8ddK34).
- [Behavior considered undefined — Rust Reference](https://doc.rust-lang.org/reference/behavior-considered-undefined.html).
- [`Vec::set_len`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.set_len), [`Vec::binary_search`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.binary_search), [`slice::get_unchecked`](https://doc.rust-lang.org/std/primitive.slice.html#method.get_unchecked).
- [`Ord`](https://doc.rust-lang.org/std/cmp/trait.Ord.html) — пример контракта безопасного трейта, нарушение которого может испортить логику алгоритмов, но не должно приводить к UB.
- [Two Kinds of Invariants](https://www.ralfj.de/blog/2018/08/22/two-kinds-of-invariants.html) и [Tightness](https://www.ecorax.net/tightness/).
- [Design by contract](https://en.wikipedia.org/wiki/Design_by_contract) и [Principle of least astonishment](https://en.wikipedia.org/wiki/Principle_of_least_astonishment).
