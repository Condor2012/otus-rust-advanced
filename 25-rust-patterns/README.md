# Занятие 25. Rust-паттерны

Примеры к занятию «Rust-паттерны»: четыре модуля с парами «наивно vs идиоматично» и тестами.

## Структура

| Модуль | Содержание |
|---|---|
| `src/m1_basics.rs` | Базовые идиомы: Option/Result и `?`, итераторы, заимствование (`&str` vs `String`), `From`/`TryFrom`, `Default` + struct update syntax |
| `src/m2_anti.rs` | Анти-паттерны и их замены: stringly-typed API → newtype, unwrap-driven development, clone-driven development, преждевременный `Rc<RefCell<T>>`, Deref-полиморфизм, God-struct, лишние аллокации |
| `src/m3_structural.rs` | Структурные паттерны: Builder, Typestate (PhantomData), RAII-guard (Drop), enum vs trait object vs дженерики, `impl AsRef<str>`/`impl Into<String>` |
| `src/m4_errors.rs` | Ошибки и модульность: ручной `Display` + `Error` + `From`, `thiserror`, `anyhow` с контекстом на границе приложения, `#[non_exhaustive]`, sealed traits, инкапсуляция инвариантов |

Каждая пара `fn_xxx_v1` / `fn_xxx_v2` показывает неидиоматичный и идиоматичный вариант одной и той же задачи, с комментариями, чем плох первый.

## Запуск

```bash
cargo test   # тесты у каждого блока
```
