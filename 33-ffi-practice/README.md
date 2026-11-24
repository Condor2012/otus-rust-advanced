# Практика FFI — примеры к занятию 33

Воркспейс из шести crate'ов:

- `my_lib` — Rust-библиотека с `#[unsafe(no_mangle)] extern "C"` (lib + staticlib + cdylib);
- `use_rust_lib` — обычное использование как Rust-зависимости;
- `use_static_lib` / `use_dyn_lib` — линковка через `#[link]` со статической/динамической библиотекой;
- `use_dyn_lib_rt` — загрузка `.so` в рантайме через `libloading`;
- `cjson-rs` — сборка C-библиотеки cJSON через `cc` и генерация биндингов `bindgen` в `build.rs`.

```sh
cargo build -p my_lib
cargo test -p my_lib -p use_rust_lib -p use_dyn_lib_rt
cargo test -p use_static_lib -p use_dyn_lib
cargo clippy -p my_lib -p use_rust_lib -p use_dyn_lib_rt -p use_static_lib -p use_dyn_lib --all-targets -- -D warnings
cargo run -p use_rust_lib
cargo run -p use_dyn_lib_rt
```

`cjson-rs` использует `bindgen`, поэтому для его сборки нужен Clang с `libclang`:

```sh
# Debian/Ubuntu
sudo apt install clang libclang-dev
cargo test -p cjson-rs
```

При вынесенном `CARGO_TARGET_DIR` линковка учитывает этот каталог автоматически. Для запуска `use_dyn_lib` динамический загрузчик ОС также должен найти `libmy_lib.so`, например через `LD_LIBRARY_PATH` на Linux.
