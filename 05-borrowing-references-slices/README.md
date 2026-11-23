# Занятие 5. Заимствование, ссылки и слайсы

Несколько независимых демонстраций по презентации занятия. Общий код находится в библиотеке и покрыт unit-тестами, а запускаемые программы расположены в `examples/`.

## Примеры

- `borrowing` — разделяемые `&T` и уникальные `&mut T` ссылки, `swap` и правило Aliasing XOR Mutability.
- `slices` — диапазоны слайсов, `&[i32]` и преобразование `&str` в `String`.
- `methods` — получатели `self`, `&self`, `&mut self` и `self: &Rc<Self>`.
- `lifetimes` — `longest`, `first`, lifetime elision и структура с заимствованной ссылкой.
- `storage` — решение практики со слайда 50. Исходный код получен из указанного в презентации Rust Playground gist; исправление — явные времена жизни в `push(&mut self, value: &'a T)` и `get(&self, index: usize) -> Option<&'a T>`.

## Запуск

```bash
cargo run --example borrowing
cargo run --example slices
cargo run --example methods
cargo run --example lifetimes
cargo run --example storage
cargo test
```

Практика `storage` выводит значение первого элемента:

```text
10
```

Ссылки в `Storage` продолжают ссылаться на `x` и `y`, а не на сам контейнер, поэтому полученная ссылка остаётся действительной после уничтожения `Storage`. Компилятор не позволит уничтожить исходные значения раньше этой ссылки.
