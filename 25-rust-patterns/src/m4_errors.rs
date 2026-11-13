//! Блок 4. Ошибки и модульность: thiserror/anyhow, границы API, sealed traits.
#![allow(dead_code, clippy::all)]

use std::fmt;

// ------------------------------------------------- 4.1 Ошибки без внешних крейтов
/// Наивно: String как тип ошибки. Обработать программно нельзя — только parse строки.
pub fn load_config_v1(raw: &str) -> Result<u16, String> {
    if raw.is_empty() {
        return Err("empty config".to_string());
    }
    raw.parse::<u16>().map_err(|e| format!("bad port: {}", e))
}

/// Идиоматично: enum ошибок + Display + Error + From. Вызывающий делает match.
#[derive(Debug)]
pub enum ConfigError {
    Empty,
    BadPort(std::num::ParseIntError),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Empty => write!(f, "configuration is empty"),
            ConfigError::BadPort(_) => write!(f, "port is not a valid number"),
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::Empty => None,
            ConfigError::BadPort(e) => Some(e),
        }
    }
}

impl From<std::num::ParseIntError> for ConfigError {
    fn from(e: std::num::ParseIntError) -> Self {
        ConfigError::BadPort(e)
    }
}

pub fn load_config_v2(raw: &str) -> Result<u16, ConfigError> {
    if raw.is_empty() {
        return Err(ConfigError::Empty);
    }
    Ok(raw.parse::<u16>()?) // From сработал автоматически
}

// ------------------------------------------------- 4.2 То же самое с thiserror
#[derive(Debug, thiserror::Error)]
pub enum PaymentError {
    #[error("payment {id} not found")]
    NotFound { id: u64 },

    #[error("insufficient funds: need {needed}, available {available}")]
    InsufficientFunds { needed: i64, available: i64 },

    #[error("provider request failed")]
    Provider(#[from] std::io::Error),
}

pub fn capture(id: u64, available: i64, needed: i64) -> Result<(), PaymentError> {
    if id == 0 {
        return Err(PaymentError::NotFound { id });
    }
    if available < needed {
        return Err(PaymentError::InsufficientFunds { needed, available });
    }
    Ok(())
}

// ------------------------------------------------- 4.3 anyhow на границе приложения
/// В приложении (bin) тип ошибки часто не важен — важен контекст.
pub fn run_app(raw_port: &str) -> anyhow::Result<u16> {
    use anyhow::Context;
    let port = load_config_v2(raw_port).context("не удалось прочитать конфигурацию клиента")?;
    capture(1, 1_000, 100).context("не удалось списать средства")?;
    Ok(port)
}

// ------------------------------------------------- 4.4 Совместимость API
/// #[non_exhaustive] позволяет добавлять варианты, не ломая чужой код:
/// внешний match обязан иметь `_ =>`.
#[non_exhaustive]
#[derive(Debug)]
pub enum WebhookEvent {
    PaymentCaptured { id: u64 },
    PaymentRefunded { id: u64 },
}

// ------------------------------------------------- 4.5 Sealed trait
mod sealed {
    pub trait Sealed { }
}

/// Трейт можно реализовать только внутри крейта: реализации не разъедутся,
/// а добавление метода не ломает пользователей.
pub trait Amount: sealed::Sealed {
    fn minor_units(&self) -> i64;
}

pub struct Rub(pub i64);
pub struct Usdt(pub i64);

impl sealed::Sealed for Rub {}
impl sealed::Sealed for Usdt {}

impl Amount for Rub {
    fn minor_units(&self) -> i64 {
        self.0 * 100
    }
}
impl Amount for Usdt {
    fn minor_units(&self) -> i64 {
        self.0 * 1_000_000
    }
}

// ------------------------------------------------- 4.6 Границы модуля
/// Наивно: всё pub, внутреннее устройство становится публичным контрактом.
pub mod bad_api {
    pub struct Ledger {
        pub entries: Vec<i64>, // любой может нарушить инвариант
        pub balance_cache: i64,
    }
}

/// Идиоматично: поля приватные, инвариант поддерживает сам тип,
/// наружу — минимальный набор методов.
pub mod good_api {
    pub struct Ledger {
        entries: Vec<i64>,
        balance: i64,
    }

    impl Ledger {
        pub fn new() -> Self {
            Self { entries: Vec::new(), balance: 0 }
        }
        pub fn push(&mut self, delta: i64) {
            self.entries.push(delta);
            self.balance += delta; // кэш не может разойтись с данными
        }
        pub fn balance(&self) -> i64 {
            self.balance
        }
        pub fn entries(&self) -> impl Iterator<Item = i64> + '_ {
            self.entries.iter().copied()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typed_errors() {
        assert!(matches!(load_config_v2(""), Err(ConfigError::Empty)));
        assert!(matches!(load_config_v2("x"), Err(ConfigError::BadPort(_))));
        assert_eq!(load_config_v2("8080").unwrap(), 8080);
    }

    #[test]
    fn thiserror_messages() {
        let e = capture(1, 10, 100).unwrap_err();
        assert_eq!(e.to_string(), "insufficient funds: need 100, available 10");
        assert!(matches!(capture(0, 0, 0), Err(PaymentError::NotFound { id: 0 })));
    }

    #[test]
    fn anyhow_context() {
        let err = run_app("nope").unwrap_err();
        assert!(err.to_string().contains("конфигурацию"));
        assert_eq!(run_app("8080").unwrap(), 8080);
    }

    #[test]
    fn ledger_invariant() {
        let mut l = good_api::Ledger::new();
        l.push(100);
        l.push(-30);
        assert_eq!(l.balance(), 70);
        assert_eq!(l.entries().count(), 2);
        assert_eq!(Usdt(2).minor_units(), 2_000_000);
    }
}
