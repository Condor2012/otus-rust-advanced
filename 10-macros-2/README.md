# Занятие 10. Макросы. Часть 2

Процедурные макросы на `syn` + `quote`: крейт обязан быть
`[lib] proc-macro = true` и компилироваться в отдельном crate.

## Структура

- `my_macros` — proc-macro крейт, три вида макросов:
  - `say_hello!` — **функциональный** (`#[proc_macro]`): печатает
    `Hello, <выражение>!`, принимает любое `syn::Expr`;
  - `#[log_call]` — **атрибутный** (`#[proc_macro_attribute]`): оборачивает
    тело функции логами вызова/выхода, сохраняя возвращаемое значение;
  - `#[derive(SimpleDebug)]` — **derive** (`#[proc_macro_derive]`): генерирует
    `impl Debug` для всех трёх видов структур — с именованными полями
    (`debug_struct`), кортежных (`debug_tuple`) и юнит-структур.
- `use_my_macros` — бинарный крейт-потребитель. В `main` раскомментируйте
  `say_hello_example()` и `my_function()`, чтобы увидеть первые два макроса.

## Запуск

```bash
cd use_my_macros
cargo run
```
