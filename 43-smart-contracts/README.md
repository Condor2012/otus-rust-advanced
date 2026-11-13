# Смарт-контракт «token» (Internet Computer) — пример к занятию 43

Учебный токен-контракт для IC: mint/transfer/balance_of на `ic-cdk` 0.20
с генерацией Candid-интерфейса.

## Структура

```
Cargo.toml       крейт token (cdylib), зависимости: candid 0.10, ic-cdk 0.20
src/lib.rs       init/mint/transfer/balance_of, экспорт Candid
src/storage.rs   IcStorage: thread_local-хранилище балансов и владельца
src/wallet.rs    трейт BalanceStorage + обобщённый Wallets<S>
src/token.did    Candid-интерфейс (service : (nat) -> {...})
dfx.json         описание канистра для dfx
```

## Сборка и деплой

```sh
dfx start --background
dfx deploy token
```

После деплоя можно вызвать:

```sh
dfx canister call token balance_of '(null)'
dfx canister call token transfer '(principal "...", 100)'
```
