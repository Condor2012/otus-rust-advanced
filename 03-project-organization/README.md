# Занятие 3. Организация проекта

Пример Cargo workspace из двух крейтов и демонстрация feature-флагов.

## Структура

- `engine` — библиотека с броском кубика d20 (`roll20`). Фича `real_rand`
  включает настоящий `rand`, без неё бросок всегда возвращает 20 (заглушка).
- `game` — бинарный крейт, использующий `engine`. Содержит:
  - `main` — одиночная атака;
  - `bin/double_attack.rs` — двойная атака;
  - `bin/tripple_attack.rs` — тройная атака.

## Как работают фичи

```toml
# game/Cargo.toml
[features]
game_real_rand = ["engine/real_rand"]   # пробрасывает фичу в engine
default = ["game_real_rand"]
```

Запуск с реальным рандомом (по умолчанию):

```bash
cargo run
```

Запуск с заглушкой (всегда 20 урона):

```bash
cargo run --no-default-features
```
