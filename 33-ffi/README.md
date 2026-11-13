# Занятие 33. FFI

Воркспейс с примерами FFI (Foreign Function Interface) к занятию «Практика FFI».

## Что показано

| Крейт | Что демонстрирует |
|---|---|
| `my_lib` | Библиотека с `#[unsafe(no_mangle)] pub extern "C" fn add`, собирается в трёх видах: rlib (`lib`), `staticlib` и `cdylib` |
| `use_rust_lib` | Обычное использование Rust-крейта по пути (`my_lib::add`) |
| `use_static_lib` | Статическая линковка: `#[link(name = "my_lib", kind = "static")]`, `unsafe extern "C"` блок, `build.rs` с `rustc-link-search` |
| `use_dyn_lib` | Динамическая линковка на этапе компиляции: `kind = "dylib"` |
| `use_dyn_lib_rt` | Загрузка `.so` в рантайме через `libloading`, поиск символа `add` |
| `cjson-rs` | Связывание с реальной C-библиотекой: `cc` компилирует `third/cJSON.c`, `bindgen` генерирует биндинги по `third/cJSON.h` (`include!(concat!(env!("OUT_DIR"), ...))`) |

## Запуск

```bash
# сначала собрать my_lib (нужна и .a, и .so в target/debug)
cargo build -p my_lib
cargo run -p use_rust_lib
cargo run -p use_static_lib
cargo run -p use_dyn_lib
cargo run -p use_dyn_lib_rt
cargo run -p cjson-rs   # требуется clang для bindgen
```
