# Rust-паттерны — примеры к занятию 25

Четыре блока: от базовых идиом до проектирования API. Каждый пример — пара
«наивно vs идиоматично» с тестами.

- `src/m1_basics.rs` — Option/Result и `?`, итераторы, заимствование, From/TryFrom, Default;
- `src/m2_anti.rs` — анти-паттерны: stringly-typed API, unwrap/clone-driven development, преждевременный `Rc<RefCell>`, Deref-полиморфизм, god-struct, лишние аллокации;
- `src/m3_structural.rs` — Builder, Typestate, RAII-guard, enum vs dyn vs дженерики, обобщение аргументов;
- `src/m4_errors.rs` — thiserror/anyhow, `#[non_exhaustive]`, sealed traits, инкапсуляция инвариантов.

```sh
cargo test
```
