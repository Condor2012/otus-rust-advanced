//! Блок 1. Базовые идиомы: Option/Result, ?, итераторы, заимствование, From/Into, Default.
#![allow(dead_code, clippy::all)]

use std::collections::HashMap;

// ---------------------------------------------------------------- 1.1 Option
#[derive(Clone)]
pub struct User {
    pub id: u64,
    pub email: Option<String>,
}

/// Наивно: клон, unwrap, индексация, "пустая строка" как признак ошибки.
pub fn email_domain_v1(user: &User) -> String {
    if user.email.is_some() {
        let email = user.email.clone().unwrap();
        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() == 2 {
            parts[1].to_string()
        } else {
            String::new()
        }
    } else {
        String::new()
    }
}

/// Идиоматично: отсутствие значения выражено типом, нет аллокаций и паник.
pub fn email_domain_v2(user: &User) -> Option<&str> {
    let email = user.email.as_deref()?;
    let (_, domain) = email.split_once('@')?;
    Some(domain)
}

// ---------------------------------------------------------------- 1.2 Result и ?
#[derive(Debug, PartialEq)]
pub enum AmountError {
    NotANumber,
    Zero,
    TooLarge { max: u64 },
}

/// Наивно: ошибки строками, ручной match, ранние return вперемешку.
pub fn parse_amount_v1(raw: &str) -> Result<u64, String> {
    let trimmed = raw.trim();
    let parsed = trimmed.parse::<u64>();
    match parsed {
        Ok(value) => {
            if value == 0 {
                return Err("amount is zero".to_string());
            }
            if value > 1_000_000 {
                return Err(format!("amount {} is too large", value));
            }
            Ok(value)
        }
        Err(_) => Err("not a number".to_string()),
    }
}

/// Идиоматично: типизированная ошибка, `?`, поток без вложенности.
pub fn parse_amount_v2(raw: &str) -> Result<u64, AmountError> {
    let value: u64 = raw.trim().parse().map_err(|_| AmountError::NotANumber)?;
    match value {
        0 => Err(AmountError::Zero),
        v if v > 1_000_000 => Err(AmountError::TooLarge { max: 1_000_000 }),
        v => Ok(v),
    }
}

// ---------------------------------------------------------------- 1.3 Итераторы
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Pending,
    Paid,
    Refunded,
}

#[derive(Clone)]
pub struct Payment {
    pub amount: u64,
    pub status: Status,
    pub provider: String,
}

/// Наивно: индексы, ручной аккумулятор, &Vec<T> в сигнатуре.
pub fn paid_total_v1(payments: &Vec<Payment>) -> u64 {
    let mut total = 0;
    for i in 0..payments.len() {
        if payments[i].status == Status::Paid {
            total = total + payments[i].amount;
        }
    }
    total
}

/// Идиоматично: срез вместо &Vec, цепочка итераторов, намерение видно сразу.
pub fn paid_total_v2(payments: &[Payment]) -> u64 {
    payments
        .iter()
        .filter(|p| p.status == Status::Paid)
        .map(|p| p.amount)
        .sum()
}

/// Наивно: ручное накопление в HashMap с двойным поиском.
pub fn by_provider_v1(payments: &[Payment]) -> HashMap<String, u64> {
    let mut map: HashMap<String, u64> = HashMap::new();
    for p in payments {
        if map.contains_key(&p.provider) {
            let current = map.get(&p.provider).unwrap().clone();
            map.insert(p.provider.clone(), current + p.amount);
        } else {
            map.insert(p.provider.clone(), p.amount);
        }
    }
    map
}

/// Идиоматично: entry API — один поиск, без clone значения.
pub fn by_provider_v2(payments: &[Payment]) -> HashMap<&str, u64> {
    let mut map = HashMap::new();
    for p in payments {
        *map.entry(p.provider.as_str()).or_insert(0) += p.amount;
    }
    map
}

/// Возврат итератора вместо Vec: вызывающий сам решает, нужна ли аллокация.
pub fn large_payments<'a>(payments: &'a [Payment], min: u64) -> impl Iterator<Item = &'a Payment> {
    payments.iter().filter(move |p| p.amount >= min)
}

// ---------------------------------------------------------------- 1.4 Заимствование
/// Наивно: функция забирает владение, хотя только читает.
pub fn mask_card_v1(number: String) -> String {
    let tail = &number[number.len() - 4..];
    format!("**** **** **** {}", tail)
}

/// Идиоматично: &str принимает и String, и литерал, и срез.
pub fn mask_card_v2(number: &str) -> String {
    let tail = number.get(number.len().saturating_sub(4)..).unwrap_or(number);
    format!("**** **** **** {}", tail)
}

// ---------------------------------------------------------------- 1.5 From / TryFrom
#[derive(Debug, Clone, PartialEq)]
pub struct Cents(pub i64);

#[derive(Debug, Clone, PartialEq)]
pub struct Rubles(pub i64);

/// Наивно: свободная функция-конвертер, которую надо помнить по имени.
pub fn rubles_to_cents(r: &Rubles) -> Cents {
    Cents(r.0 * 100)
}

/// Идиоматично: From даёт `.into()`, работает в дженериках и в `?`.
impl From<Rubles> for Cents {
    fn from(r: Rubles) -> Self {
        Cents(r.0 * 100)
    }
}

/// Валидация — это TryFrom, а не конструктор, который иногда паникует.
pub struct Iso4217(String);

impl TryFrom<&str> for Iso4217 {
    type Error = AmountError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() == 3 && value.chars().all(|c| c.is_ascii_uppercase()) {
            Ok(Iso4217(value.to_string()))
        } else {
            Err(AmountError::NotANumber)
        }
    }
}

// ---------------------------------------------------------------- 1.6 Default
#[derive(Debug)]
pub struct ClientConfig {
    pub endpoint: String,
    pub timeout_ms: u64,
    pub retries: u8,
    pub verify_tls: bool,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            endpoint: "https://api.example.com".into(),
            timeout_ms: 5_000,
            retries: 3,
            verify_tls: true,
        }
    }
}

/// Struct update syntax: меняем одно поле, остальное — из Default.
pub fn slow_client() -> ClientConfig {
    ClientConfig {
        timeout_ms: 30_000,
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_versions_agree() {
        let u = User { id: 1, email: Some("a@b.com".into()) };
        assert_eq!(email_domain_v1(&u), "b.com");
        assert_eq!(email_domain_v2(&u), Some("b.com"));
    }

    #[test]
    fn amount_parsing() {
        assert_eq!(parse_amount_v2("  42 "), Ok(42));
        assert_eq!(parse_amount_v2("0"), Err(AmountError::Zero));
        assert_eq!(parse_amount_v2("x"), Err(AmountError::NotANumber));
    }

    #[test]
    fn totals_agree() {
        let ps = vec![
            Payment { amount: 100, status: Status::Paid, provider: "acme".into() },
            Payment { amount: 50, status: Status::Pending, provider: "acme".into() },
            Payment { amount: 70, status: Status::Paid, provider: "globex".into() },
        ];
        assert_eq!(paid_total_v1(&ps), 170);
        assert_eq!(paid_total_v2(&ps), 170);
        assert_eq!(by_provider_v2(&ps)["acme"], 150);
        assert_eq!(large_payments(&ps, 70).count(), 2);
    }

    #[test]
    fn conversions() {
        let c: Cents = Rubles(5).into();
        assert_eq!(c, Cents(500));
        assert!(Iso4217::try_from("USD").is_ok());
        assert!(Iso4217::try_from("usd").is_err());
    }

    #[test]
    fn masking() {
        assert_eq!(mask_card_v2("4111111111111111"), "**** **** **** 1111");
        assert_eq!(slow_client().retries, 3);
    }
}
