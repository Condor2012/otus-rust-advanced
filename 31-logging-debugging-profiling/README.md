# Занятие 31. Логи, отладка, профилирование

Примеры по презентации «Логирование, отладка, профилирование и бенчмарки»: уровни логирования, структурированные события, `dbg!`, отладчики, CPU-профилирование и Criterion.

## Структура

- `examples/log.rs` — фасад `log` и реализация `env_logger`: `trace`, `debug`, `info`, `warn`, `error`.
- `examples/tracing.rs` — `tracing` и `tracing-subscriber`, инструментирование функции через `#[instrument]` и структурированные поля событий.
- `examples/debug.rs` — сравнение `println!` и `dbg!`; второй макрос выводит файл, строку и выражение в stderr и возвращает значение.
- `examples/workload.rs` — небольшая нагрузка для запуска под профилировщиком; `black_box` мешает компилятору удалить вычисление.
- `benches/workload.rs` — воспроизводимый бенчмарк Criterion для `sum_to`.

## Запуск

```bash
RUST_LOG=trace cargo run --example log
RUST_LOG=info cargo run --example log
cargo run --example tracing
cargo run --example debug
cargo run --release --example workload
cargo bench
```

При `RUST_LOG=info` сообщения уровней `trace` и `debug` отфильтровываются. Подписчик `tracing` в примере ограничен уровнем `INFO`.

## Отладка

Сборка в профиле `dev` сохраняет отладочную информацию:

```bash
cargo build --example debug
rust-gdb /tmp/otus-rust-advanced-target/debug/examples/debug
```

На локальном компьютере путь обычно будет `target/debug/examples/debug`. Основные команды GDB/LLDB:

```text
break main
run
next
print values
```

Для LLDB команда чтения значения также записывается как `print values` или сокращённо `p values`.

## Профилирование

В Linux после установки `perf` и `flamegraph`:

```bash
cargo build --release --example workload
perf record --call-graph dwarf target/release/examples/workload
perf report
perf script | stackcollapse-perf.pl | flamegraph.pl > flamegraph.svg
```

В зависимости от установки FlameGraph последний шаг может выглядеть как `perf script | inferno-collapse-perf | inferno-flamegraph > flamegraph.svg`. В macOS вместо `perf` используется `xctrace` из Xcode. Правила ОС могут ограничивать доступ к счётчикам производительности.

## Бенчмарки

Встроенный `#[bench]` требует nightly и не используется в crate. Criterion работает на стабильном Rust, сохраняет статистику и HTML-отчёты. Для только проверки сборки бенчмарка:

```bash
cargo bench --no-run
```

## Материалы

- [log](https://docs.rs/log), [env_logger](https://docs.rs/env_logger).
- [tracing](https://docs.rs/tracing), [tracing-subscriber](https://docs.rs/tracing-subscriber).
- [Criterion.rs](https://docs.rs/criterion).
- [The Rust Performance Book](https://nnethercote.github.io/perf-book/).
