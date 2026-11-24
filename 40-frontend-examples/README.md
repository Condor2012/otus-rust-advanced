# Фреймворки фронтенда на Rust — примеры к занятию

Три демо-проекта: Dioxus (fullstack), Leptos (CSR), Leptos + Tauri 2.

```
first-proj/   Dioxus 0.6 fullstack: роутер, server functions, tailwind (dx serve)
second-proj/  Leptos 0.8 CSR: роутер, компоненты, trunk (trunk serve)
tauri-proj/   Leptos-фронтенд в десктопном приложении Tauri 2 (cargo tauri dev)
```

Не включены в репозиторий (собираются/генерируются локально): `target/`, `dist/`,
`src-tauri/gen/`, `src-tauri/icons/`, `Cargo.lock`, бинарные ассеты иконок.

## Проверка

```bash
cd first-proj
cargo test --no-default-features
cargo clippy --no-default-features --all-targets -- -D warnings

cd ../second-proj
cargo test
cargo clippy --all-targets -- -D warnings

# Только Leptos-интерфейс, без системных библиотек Tauri:
cd ../tauri-proj
cargo check -p tauri-proj-ui
```

Для полноценного запуска нужны Dioxus CLI, Trunk и Tauri CLI. Сборка
`tauri-proj` на Debian/Ubuntu дополнительно требует системные пакеты:

```bash
sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev librsvg2-dev libayatana-appindicator3-dev
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```
