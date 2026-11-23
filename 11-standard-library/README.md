# Занятие 11. Стандартная библиотека

Материалы по презентации «Стандартная библиотека»: умные указатели, внутренняя мутабельность, коллекции и итераторы. Используется только стандартная библиотека Rust, внешних зависимостей нет.

## Структура

- `src/sorted_vec.rs` — практика со `SortedVec`: `Deref`, автоматическое разыменование и исправление клонирования через `#[derive(Clone)]`.
- `src/tree.rs` — дерево `Rc<Node>`; сильные ссылки ведут к детям, а ссылка на родителя заменена на `Weak<Node>`, поэтому цикла владения нет и `Drop` срабатывает.
- `src/collections.rs` — `HashMap::entry`, `HashSet`, `BTreeSet` и `FromIterator`.
- `src/words.rs` — собственный `Iterator` с заимствованными элементами и реализация `IntoIterator`.
- `examples/smart_pointers.rs` — `Box`, `Deref`, `Cell`, `RefCell`, `Rc<RefCell<T>>` и практика `SortedVec`.
- `examples/collections.rs` — `Vec`, `VecDeque`, `HashMap`, множества и `BinaryHeap`.
- `examples/iterators.rs` — ленивые адаптеры, потребители, бесконечный диапазон с `take`, собственный итератор и `IntoIterator`.
- `examples/tree.rs` — практика с устранением цикла `Rc` и вывод деструкторов.

## Запуск

```bash
cargo run --example smart_pointers
cargo run --example collections
cargo run --example iterators
cargo run --example tree
```

В примере `iterators` сначала печатается сообщение о создании ленивого итератора, а затем последовательность `112031`: вывод исходного значения и его остатка чередуются во время потребления итератора.

При завершении `tree` сначала срабатывает деструктор корня `1`, а затем — деструкторы детей `3` и `5`. Это подтверждает, что слабая родительская ссылка не удерживает дерево в памяти.

## Проверка

```bash
cargo test
cargo clippy --all-targets -- -D warnings
```

Тесты проверяют сортировку и клонирование `SortedVec`, отсутствие цикла владения в дереве, подсчёт символов через Entry API, построение множеств через `collect`, собственный `Iterator` и `IntoIterator`.

## Практика из презентации

- [Исходный `SortedVec`](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=af05899253f6f5b7522649bc79ba4c5b) и [решение](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=c8153c9b41d49ad5c1eb3cb6e5a4ba5c).
- [Дерево с циклом `Rc`](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=55e7f6aab605013f1475df9795c1d66b) и [решение через `Weak`](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=726073d98f4534a2e1c2d992cafe05a1).
