# Занятие 22. Элементы ФП и ООП в Rust

Примеры по презентации «ФП и ООП в Rust»: объектно-ориентированные идиомы через структуры и трейты, SOLID, функции высших порядков, рекурсия, ленивые итераторы и алгебраические типы данных.

## Структура

- `src/solid.rs` — расширение поведения новыми реализациями `PaymentProcessor` (OCP), blanket impl `Logger`, заменяемые `Shape` (LSP), узкие трейты `Workable`/`Eatable`/`Sleepable` (ISP) и `UserService<R: UserRepository>` с внедрением зависимости (DIP).
- `src/functional.rs` — `apply`, `compose`, функция `make_incrementer`, преобразование строк `Grid`, рекурсивный `factorial` и ленивый бесконечный итератор.
- `src/adt.rs` — рекурсивный `List<A>` через `Box`, типы-суммы и типы-произведения, pattern matching с диапазонами и guard-условиями, тотальный разбор `Color`.
- `examples/` — три самостоятельные демонстрации тех же разделов.

SRP показан разделением модулей и отдельным `UserService`: регистрация не смешивается с отправкой почты и логированием. Rust не использует наследование данных; поведение задаётся трейтами и композицией.

## Запуск

```bash
cargo run --example solid
cargo run --example functional
cargo run --example adt
```

В `functional` итоговая сетка равна `[[3, 0, 0], [3, 0, 0], [3, 0, 0]]`, а `take(5)` ограничивает бесконечный итератор квадратов значениями `[0, 1, 4, 9, 16]`.

## Проверка

```bash
cargo test
cargo clippy --all-targets -- -D warnings
```

## Playground-примеры из презентации

- [SRP: разделение ответственности](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=091b77600cf3bce6e90ff6dfba79e3ea).
- [OCP: платёжные процессоры и blanket impl](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=c2860e23d89a144e4b9ee0e682ba07ad).
- [LSP: прямоугольник и квадрат](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=891a70432cad12738ef8f08e3b7d427d).
- [ISP: человек и робот](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=865110775e5fbe0f3d00b7c351f8176a).
- [DIP: сервис и репозиторий](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=71bfb8a1bd652189f6349b91eefd9d79).
- [Функции высших порядков и Grid](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=0d88e137fb75207d4585da8a24ed46c2).
