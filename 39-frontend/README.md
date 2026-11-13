# Rust Frontend — демо к занятию

WebAssembly, wasm-bindgen, Yew, Tauri. Проверено на Rust 1.97 (Linux x86_64):
wasm-bindgen 0.2.129, web-sys 0.3.106, Yew 0.23, trunk 0.22, Tauri 2.12, tauri-cli 2.12.

```
raw-wasm/    голый wasm без wasm-bindgen: 145 байт, только числа
wasm-lib/    wasm-bindgen: функции, классы, исключения, web-sys, serde, тесты внутри wasm
yew-app/     Yew SPA: use_state, свойства, use_reducer, use_effect_with, SSR-тест, trunk
tauri-app/   Tauri 2 + фронтенд на Yew: команды, State, события, capabilities, тест IPC
screenshots/ как это выглядит после запуска
```

## Подготовка

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.129 --locked   # версия = версии крейта!
cargo install trunk --locked
cargo install tauri-cli --version "^2" --locked
# binaryen (wasm-opt) свежий, не ниже ~v116: npm i -g binaryen  или  с GitHub
# Linux для Tauri:
sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev librsvg2-dev libayatana-appindicator3-dev
```

## 1. raw-wasm

```sh
cd raw-wasm
rustc --target wasm32-unknown-unknown --crate-type cdylib -C opt-level=z -C panic=abort -C strip=symbols add.rs -o add.wasm
node run.mjs
```

## 2. wasm-lib

```sh
cd wasm-lib
cargo test                                              # обычные тесты (x86_64)
cargo test --release --target wasm32-unknown-unknown    # тесты внутри wasm (runner в .cargo/config.toml)
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/wasm_lib.wasm --target nodejs --out-dir pkg-node
wasm-bindgen target/wasm32-unknown-unknown/release/wasm_lib.wasm --target web --out-dir www/pkg
node node/run.js
cd www && python3 -m http.server 8080    # открыть http://localhost:8080
```

Эквивалент одной командой: `wasm-pack build --target web`.

Ловушка из демо: в `sum_primes` было `let mut j = i * i;` — в wasm32 `usize` 32-битный,
и при `i > 65535` произведение переполняется. Обычные тесты на x86_64 этого не видят,
тест `tests/wasm.rs` под wasm — видит. Сейчас там `i.saturating_mul(i)`.

## 3. yew-app

```sh
cd yew-app
cargo test            # SSR-рендер в строку + тест редьюсера, без браузера
trunk serve           # http://127.0.0.1:8080, пересборка при изменениях
trunk build --release # dist/: ~148 КБ wasm (63 КБ gzip)
```

## 4. tauri-app

```sh
cd tauri-app
cargo tauri dev                       # trunk serve + окно приложения
cargo tauri build                     # релизная сборка и пакеты (deb, AppImage, msi, dmg ...)
trunk build && cargo test -p tauri-demo   # тест команд через mock IPC (нужен собранный dist/)
```

При запуске окно сразу вызывает `greet` и `system_info`, форма сохраняет заметки в `State`
(пустая — ошибка из Rust), кнопка «Обратный отсчёт» получает события `tick` 5 → 0.
