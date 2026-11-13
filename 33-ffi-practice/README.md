# Практика FFI — примеры к занятию 33

Воркспейс из шести crate'ов:

- `my_lib` — Rust-библиотека с `#[unsafe(no_mangle)] extern "C"` (lib + staticlib + cdylib);
- `use_rust_lib` — обычное использование как Rust-зависимости;
- `use_static_lib` / `use_dyn_lib` — линковка через `#[link]` со статической/динамической библиотекой;
- `use_dyn_lib_rt` — загрузка `.so` в рантайме через `libloading`;
- `cjson-rs` — сборка C-библиотеки cJSON через `cc` и генерация биндингов `bindgen` в `build.rs`.

```sh
cargo build
```
