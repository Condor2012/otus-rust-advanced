# Занятие 12. Практика Rust-разработки #1

Мини-блокчейн рантайм в стиле Substrate: модульная state-machine
из «паллет» (pallets) с диспатчем вызовов и исполнением блоков экстринсиков.

## Структура

- `src/system.rs` — системный pallet: номер блока, nonce аккаунтов
  (ассоциированные типы через трейт `Config`, арифметика через `num`).
- `src/balances.rs` — pallet балансов: `set_balance`, `transfer`
  с проверками переполнения (`CheckedAdd`/`CheckedSub`), unit-тесты.
- `src/proof_of_existance.rs` — proof-of-existence: `create_claim`/`revoke_claim`.
- `src/support.rs` — инфраструктура: `Block`, `Header`, `Extrinsic`,
  трейт `Dispatch`, `DispatchResult`.
- `src/macros/mod.rs` — декларативные макросы (`extrinsic!`, `block!`,
  `transfer_call!`, `poe_call!`, `extrinsics!`, `account!`, `setup_account!`).
- `src/lib.rs` — `Runtime` + `TransactionService`: очередь транзакций,
  собирающая блок из пяти переводов (`listener`).
- `src/main.rs` — демонстрация: два блока с переводами между аккаунтами.

## Запуск

```bash
cargo run
cargo test
```
