# Занятие 43. Примеры смарт-контрактов

Токен на Internet Computer (ICP): Rust-канистер на `ic-cdk` с mint/transfer/balance_of.

## Что показано

- Разделение логики и хранилища: трейт `BalanceStorage` (credit/debit/balance_of) + обобщённый `Wallets<S>`, хранилище `IcStorage` на `thread_local!` (`RefCell<HashMap<Principal, Nat>>`) — канистеры однопоточны, `thread_local` — идиоматичное «глобальное» состояние.
- Макросы `ic-cdk`: `#[init]`, `#[update]`, `#[query]`, `ic_cdk::export_candid!()`.
- `Principal` как идентификатор пользователя, `msg_caller()` — кто вызвал метод.
- Авторизация mint по owner, дебет со проверкой баланса (`InsufficientBalance`).
- Интерфейс в Candid (`token.did`), деплой через `dfx.json` (`crate-type = ["cdylib"]`).

## Запуск (локальный реплика)

```bash
dfx start --background
dfx deploy
dfx canister call token balance_of '(null)'
```
