//! Блок 2. Анти-паттерны и их идиоматичные замены.
#![allow(dead_code, clippy::all)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// ------------------------------------------- 2.1 Stringly-typed API -> newtype
/// Анти-паттерн: три строки подряд, компилятор не заметит перестановку аргументов.
pub fn transfer_v1(from: String, to: String, currency: String, amount: i64) -> Result<(), String> {
    if amount <= 0 {
        return Err("bad amount".into());
    }
    let _ = (from, to, currency);
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AccountId(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Currency {
    Rub,
    Usd,
    Usdt,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Money {
    pub minor_units: i64,
    pub currency: Currency,
}

#[derive(Debug, PartialEq)]
pub enum TransferError {
    NonPositiveAmount,
    SameAccount,
}

impl AccountId {
    /// Единственная дверь в тип — валидация не обходится.
    pub fn new(raw: &str) -> Option<Self> {
        (!raw.is_empty() && raw.len() <= 64).then(|| AccountId(raw.to_string()))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Идиоматично: перепутать `from` и `to` всё ещё можно, но перепутать
/// счёт с валютой или с суммой — уже нет; инварианты живут в типах.
pub fn transfer_v2(from: &AccountId, to: &AccountId, amount: Money) -> Result<(), TransferError> {
    if amount.minor_units <= 0 {
        return Err(TransferError::NonPositiveAmount);
    }
    if from == to {
        return Err(TransferError::SameAccount);
    }
    Ok(())
}

// ------------------------------------------------- 2.2 unwrap-driven development
pub struct Settings {
    values: HashMap<String, String>,
}

impl Settings {
    pub fn new() -> Self {
        Self { values: HashMap::new() }
    }

    /// Анти-паттерн: библиотека решает за приложение, что программе пора умереть.
    pub fn port_v1(&self) -> u16 {
        self.values.get("port").unwrap().parse().unwrap()
    }

    /// Идиоматично: вернуть Result и дать вызывающему выбрать реакцию.
    pub fn port_v2(&self) -> Result<u16, SettingsError> {
        let raw = self
            .values
            .get("port")
            .ok_or(SettingsError::Missing("port"))?;
        raw.parse().map_err(|_| SettingsError::Invalid("port"))
    }

    /// Разумный компромисс, когда значение необязательно.
    pub fn port_or_default(&self) -> u16 {
        self.values
            .get("port")
            .and_then(|v| v.parse().ok())
            .unwrap_or(8080)
    }
}

#[derive(Debug, PartialEq)]
pub enum SettingsError {
    Missing(&'static str),
    Invalid(&'static str),
}

// ------------------------------------------------- 2.3 Clone-driven development
#[derive(Clone)]
pub struct Order {
    pub id: u64,
    pub lines: Vec<String>,
}

/// Анти-паттерн: clone, чтобы «успокоить» borrow checker.
pub fn count_lines_v1(orders: Vec<Order>) -> usize {
    let copy = orders.clone();
    let mut n = 0;
    for o in copy {
        n += o.lines.clone().len();
    }
    n
}

/// Идиоматично: одно заимствование, ноль аллокаций.
pub fn count_lines_v2(orders: &[Order]) -> usize {
    orders.iter().map(|o| o.lines.len()).sum()
}

// ------------------------------------- 2.4 Преждевременный Rc<RefCell<T>>
pub struct NodeShared {
    pub name: String,
    pub children: Vec<Rc<RefCell<NodeShared>>>,
}

/// Анти-паттерн: разделяемая изменяемость там, где хватает владения.
/// Цена — рантайм-проверки, паника при повторном borrow_mut, отказ от Send.
pub fn rename_all_v1(node: &Rc<RefCell<NodeShared>>, suffix: &str) {
    let mut n = node.borrow_mut();
    n.name.push_str(suffix);
    for c in &n.children {
        rename_all_v1(c, suffix);
    }
}

pub struct Node {
    pub name: String,
    pub children: Vec<Node>,
}

/// Идиоматично: дерево владеет детьми, изменяемость — через &mut.
pub fn rename_all_v2(node: &mut Node, suffix: &str) {
    node.name.push_str(suffix);
    for c in &mut node.children {
        rename_all_v2(c, suffix);
    }
}

// ------------------------------------------------- 2.5 Deref-полиморфизм
pub struct Engine {
    pub power: u32,
}

impl Engine {
    pub fn start(&self) -> &'static str {
        "engine started"
    }
}

pub struct Car {
    engine: Engine,
}

/// Анти-паттерн: Deref используют, чтобы «унаследовать» методы Engine.
/// Методы протекают в Car, автодополнение врёт, отношения не видно в типах.
impl std::ops::Deref for Car {
    type Target = Engine;
    fn deref(&self) -> &Engine {
        &self.engine
    }
}

pub struct CarGood {
    engine: Engine,
}

/// Идиоматично: явная делегация (или трейт, если нужен полиморфизм).
impl CarGood {
    pub fn start(&self) -> &'static str {
        self.engine.start()
    }
    pub fn engine(&self) -> &Engine {
        &self.engine
    }
}

// ------------------------------------------------- 2.6 God-struct / «менеджер»
/// Анти-паттерн: один тип знает про всё.
pub struct PaymentManager {
    pub db_url: String,
    pub http_timeout: u64,
    pub retry_count: u8,
    pub audit_log: Vec<String>,
    pub cache: HashMap<String, String>,
}

impl PaymentManager {
    pub fn do_everything(&mut self, _raw_request: &str) -> Result<(), String> {
        // валидация + поход в БД + HTTP + запись в лог + инвалидация кэша...
        Ok(())
    }
}

/// Идиоматично: роли разделены трейтами, зависимости видны в сигнатурах.
pub trait AccountRepo {
    fn balance(&self, id: &AccountId) -> Option<Money>;
}

pub trait Ledger {
    fn record(&mut self, from: &AccountId, to: &AccountId, amount: Money);
}

pub struct TransferService<R, L> {
    repo: R,
    ledger: L,
}

impl<R: AccountRepo, L: Ledger> TransferService<R, L> {
    pub fn new(repo: R, ledger: L) -> Self {
        Self { repo, ledger }
    }

    pub fn execute(
        &mut self,
        from: &AccountId,
        to: &AccountId,
        amount: Money,
    ) -> Result<(), TransferError> {
        transfer_v2(from, to, amount)?;
        let balance = self.repo.balance(from).unwrap_or(Money {
            minor_units: 0,
            currency: amount.currency,
        });
        if balance.minor_units < amount.minor_units {
            return Err(TransferError::NonPositiveAmount);
        }
        self.ledger.record(from, to, amount);
        Ok(())
    }
}

// ------------------------------------------------- 2.7 Лишние аллокации
/// Анти-паттерн: строка собирается заново на каждой итерации.
pub fn join_v1(items: &[String]) -> String {
    let mut out = String::new();
    for (i, s) in items.iter().enumerate() {
        if i > 0 {
            out = format!("{}, {}", out, s);
        } else {
            out = format!("{}", s);
        }
    }
    out
}

/// Идиоматично: один проход, один буфер.
pub fn join_v2(items: &[String]) -> String {
    items.join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn newtype_transfer() {
        let a = AccountId::new("acc-1").unwrap();
        let b = AccountId::new("acc-2").unwrap();
        let m = Money { minor_units: 100, currency: Currency::Usdt };
        assert_eq!(transfer_v2(&a, &b, m), Ok(()));
        assert_eq!(transfer_v2(&a, &a, m), Err(TransferError::SameAccount));
        assert!(AccountId::new("").is_none());
    }

    #[test]
    fn settings() {
        let s = Settings::new();
        assert_eq!(s.port_v2(), Err(SettingsError::Missing("port")));
        assert_eq!(s.port_or_default(), 8080);
    }

    #[test]
    fn counting_and_joining() {
        let orders = vec![
            Order { id: 1, lines: vec!["a".into(), "b".into()] },
            Order { id: 2, lines: vec!["c".into()] },
        ];
        assert_eq!(count_lines_v2(&orders), 3);
        assert_eq!(count_lines_v1(orders.clone()), 3);
        assert_eq!(join_v1(&["x".to_string(), "y".to_string()]), join_v2(&["x".to_string(), "y".to_string()]));
    }

    #[test]
    fn tree_rename() {
        let mut root = Node {
            name: "root".into(),
            children: vec![Node { name: "child".into(), children: vec![] }],
        };
        rename_all_v2(&mut root, "!");
        assert_eq!(root.name, "root!");
        assert_eq!(root.children[0].name, "child!");
    }

    struct MemRepo(Money);
    impl AccountRepo for MemRepo {
        fn balance(&self, _id: &AccountId) -> Option<Money> {
            Some(self.0)
        }
    }
    struct VecLedger(Vec<String>);
    impl Ledger for VecLedger {
        fn record(&mut self, from: &AccountId, to: &AccountId, amount: Money) {
            self.0.push(format!("{}->{}:{}", from.as_str(), to.as_str(), amount.minor_units));
        }
    }

    #[test]
    fn service_is_testable_without_db() {
        let mut svc = TransferService::new(
            MemRepo(Money { minor_units: 1_000, currency: Currency::Usdt }),
            VecLedger(Vec::new()),
        );
        let a = AccountId::new("a").unwrap();
        let b = AccountId::new("b").unwrap();
        assert!(svc
            .execute(&a, &b, Money { minor_units: 500, currency: Currency::Usdt })
            .is_ok());
    }
}
