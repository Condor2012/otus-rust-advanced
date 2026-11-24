# Занятие 30. Утилитарные крейты

Примеры по презентации «Утилитарные библиотеки»: работа с битами и байтами, математика, обработка данных и параллельные вычисления.

## Структура

- `src/bits.rs` — `bytemuck` для POD-типа `Point`, битовые флаги `bitflags` и inline-хранение `SmallVec`.
- `src/math.rs` — повороты векторов через `nalgebra` и `glam`, преобразование в пространство камеры и воспроизводимая генерация случайных значений через `StdRng`.
- `src/data.rs` — `serde`/`serde_json`, бинарная сериализация `bincode`, парсер строки запроса на `nom` и аргументы командной строки через derive API `clap`.
- `src/parallel.rs` — `rayon::join`, параллельные итераторы, `crossbeam::atomic::AtomicCell`, `Parker`/`Unparker`, `Backoff` и `WaitGroup`.
- `examples/` — четыре самостоятельные демонстрации, соответствующие разделам презентации.

`bitvec` дополнительно показан в `bits_and_bytes`: битовый вектор хранит последовательность битов компактнее `Vec<bool>` и предоставляет привычные срезы. `rusty_pool` упомянут на слайдах, но отдельный пример не добавлен: основные приёмы пула и параллельного запуска уже показаны через Rayon, без дополнительной асинхронной зависимости.

## Запуск

```bash
cargo run --example bits_and_bytes
cargo run --example math
cargo run --example data
cargo run --example concurrency
```

Случайные значения получаются из генератора с фиксированным seed, поэтому вывод повторяем. В отличие от примеров со слайдов, демонстрации не используют искусственные задержки `sleep`: результат параллельных операций проверяется непосредственно.

## Проверка

```bash
cargo test
cargo clippy --all-targets -- -D warnings
```

## Документация библиотек

- Биты и байты: [bytemuck](https://docs.rs/bytemuck), [bitflags](https://docs.rs/bitflags), [bitvec](https://docs.rs/bitvec), [smallvec](https://docs.rs/smallvec).
- Математика: [nalgebra](https://docs.rs/nalgebra), [glam](https://docs.rs/glam), [rand](https://docs.rs/rand).
- Данные: [serde](https://docs.rs/serde), [bincode](https://docs.rs/bincode), [nom](https://docs.rs/nom), [clap](https://docs.rs/clap).
- Многопоточность: [rayon](https://docs.rs/rayon), [crossbeam](https://docs.rs/crossbeam), [rusty_pool](https://docs.rs/rusty_pool).
