//! Блок 3. Структурные паттерны: Builder, Typestate, RAII-guard, enum vs dyn, дженерики.
#![allow(dead_code, clippy::all)]

use std::marker::PhantomData;

// ------------------------------------------------------------- 3.1 Builder
/// Наивно: конструктор на семь аргументов. Что означает `true, false, 3`?
pub fn new_client_v1(
    endpoint: String,
    timeout_ms: u64,
    retries: u8,
    verify_tls: bool,
    follow_redirects: bool,
    user_agent: Option<String>,
) -> HttpClient {
    HttpClient { endpoint, timeout_ms, retries, verify_tls, follow_redirects, user_agent }
}

#[derive(Debug, Clone)]
pub struct HttpClient {
    pub endpoint: String,
    pub timeout_ms: u64,
    pub retries: u8,
    pub verify_tls: bool,
    pub follow_redirects: bool,
    pub user_agent: Option<String>,
}

/// Идиоматично: builder — обязательное в `new`, необязательное — цепочкой.
pub struct HttpClientBuilder {
    endpoint: String,
    timeout_ms: u64,
    retries: u8,
    verify_tls: bool,
    follow_redirects: bool,
    user_agent: Option<String>,
}

impl HttpClient {
    pub fn builder(endpoint: impl Into<String>) -> HttpClientBuilder {
        HttpClientBuilder {
            endpoint: endpoint.into(),
            timeout_ms: 5_000,
            retries: 3,
            verify_tls: true,
            follow_redirects: false,
            user_agent: None,
        }
    }
}

impl HttpClientBuilder {
    pub fn timeout_ms(mut self, v: u64) -> Self {
        self.timeout_ms = v;
        self
    }
    pub fn retries(mut self, v: u8) -> Self {
        self.retries = v;
        self
    }
    pub fn verify_tls(mut self, v: bool) -> Self {
        self.verify_tls = v;
        self
    }
    pub fn user_agent(mut self, v: impl Into<String>) -> Self {
        self.user_agent = Some(v.into());
        self
    }
    pub fn build(self) -> HttpClient {
        HttpClient {
            endpoint: self.endpoint,
            timeout_ms: self.timeout_ms,
            retries: self.retries,
            verify_tls: self.verify_tls,
            follow_redirects: self.follow_redirects,
            user_agent: self.user_agent,
        }
    }
}

// ------------------------------------------------------------- 3.2 Typestate
/// Наивно: состояние — поле, корректность порядка вызовов проверяется в рантайме.
pub struct PaymentDyn {
    id: u64,
    state: PaymentState,
}

#[derive(PartialEq, Debug)]
pub enum PaymentState {
    Draft,
    Authorized,
    Captured,
}

impl PaymentDyn {
    pub fn new(id: u64) -> Self {
        Self { id, state: PaymentState::Draft }
    }
    pub fn authorize(&mut self) -> Result<(), &'static str> {
        if self.state != PaymentState::Draft {
            return Err("wrong state");
        }
        self.state = PaymentState::Authorized;
        Ok(())
    }
    pub fn capture(&mut self) -> Result<(), &'static str> {
        if self.state != PaymentState::Authorized {
            return Err("wrong state"); // узнаем только на проде
        }
        self.state = PaymentState::Captured;
        Ok(())
    }
}

/// Идиоматично: состояние — параметр типа. Неверный порядок не компилируется.
pub struct Draft;
pub struct Authorized;
pub struct Captured;

pub struct PaymentT<S> {
    id: u64,
    _state: PhantomData<S>,
}

impl PaymentT<Draft> {
    pub fn new(id: u64) -> Self {
        Self { id, _state: PhantomData }
    }
    pub fn authorize(self) -> PaymentT<Authorized> {
        PaymentT { id: self.id, _state: PhantomData }
    }
}

impl PaymentT<Authorized> {
    pub fn capture(self) -> PaymentT<Captured> {
        PaymentT { id: self.id, _state: PhantomData }
    }
    pub fn void(self) -> PaymentT<Draft> {
        PaymentT { id: self.id, _state: PhantomData }
    }
}

impl<S> PaymentT<S> {
    pub fn id(&self) -> u64 {
        self.id
    }
}
// PaymentT::<Draft>::new(1).capture(); // ошибка компиляции: метода нет

// ------------------------------------------------------------- 3.3 RAII-guard
pub struct Metrics {
    pub open_transactions: i32,
    pub log: Vec<String>,
}

/// Наивно: парные вызовы begin/end. Любой ранний return или `?` ломает баланс.
pub fn with_transaction_v1(m: &mut Metrics, fail: bool) -> Result<(), &'static str> {
    m.open_transactions += 1;
    m.log.push("begin".into());
    if fail {
        return Err("boom"); // end никогда не вызовется
    }
    m.open_transactions -= 1;
    m.log.push("end".into());
    Ok(())
}

/// Идиоматично: освобождение ресурса привязано к области видимости через Drop.
pub struct TxGuard<'a> {
    metrics: &'a mut Metrics,
}

impl<'a> TxGuard<'a> {
    pub fn begin(metrics: &'a mut Metrics) -> Self {
        metrics.open_transactions += 1;
        metrics.log.push("begin".into());
        Self { metrics }
    }
}

impl<'a> Drop for TxGuard<'a> {
    fn drop(&mut self) {
        self.metrics.open_transactions -= 1;
        self.metrics.log.push("end".into());
    }
}

pub fn with_transaction_v2(m: &mut Metrics, fail: bool) -> Result<(), &'static str> {
    let _tx = TxGuard::begin(m);
    if fail {
        return Err("boom"); // Drop всё равно закроет транзакцию
    }
    Ok(())
}

// --------------------------------------------- 3.4 enum vs trait object
pub trait Provider {
    fn name(&self) -> &'static str;
    fn fee_minor_units(&self, amount: i64) -> i64;
}

pub struct Acme;
pub struct Globex;

impl Provider for Acme {
    fn name(&self) -> &'static str {
        "acme"
    }
    fn fee_minor_units(&self, amount: i64) -> i64 {
        amount / 100
    }
}

impl Provider for Globex {
    fn name(&self) -> &'static str {
        "globex"
    }
    fn fee_minor_units(&self, amount: i64) -> i64 {
        amount / 50 + 10
    }
}

/// Закрытое множество вариантов — enum: match исчерпывающий, нет аллокаций,
/// компилятор напомнит о каждом месте при добавлении варианта.
pub enum ProviderEnum {
    Acme,
    Globex,
}

impl ProviderEnum {
    pub fn fee_minor_units(&self, amount: i64) -> i64 {
        match self {
            ProviderEnum::Acme => amount / 100,
            ProviderEnum::Globex => amount / 50 + 10,
        }
    }
}

/// Открытое множество (плагины, реализации извне) — трейт-объекты.
pub fn total_fee(providers: &[Box<dyn Provider>], amount: i64) -> i64 {
    providers.iter().map(|p| p.fee_minor_units(amount)).sum()
}

/// Статическая диспетчеризация: одна реализация на вызов, инлайн, нет vtable.
pub fn fee_static<P: Provider>(provider: &P, amount: i64) -> i64 {
    provider.fee_minor_units(amount)
}

// --------------------------------------------- 3.5 Обобщение аргументов
/// Наивно: заставляем вызывающего сделать String, даже если у него &str.
pub fn label_v1(name: String) -> String {
    format!("[{}]", name)
}

/// Идиоматично: impl Into<String> — одна аллокация и только когда нужна.
pub fn label_v2(name: impl AsRef<str>) -> String {
    format!("[{}]", name.as_ref())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_reads_better() {
        let c = HttpClient::builder("https://api.example.com")
            .timeout_ms(30_000)
            .retries(5)
            .user_agent("lesson/1.0")
            .build();
        assert_eq!(c.timeout_ms, 30_000);
        assert!(c.verify_tls);
    }

    #[test]
    fn typestate_flow() {
        let p = PaymentT::new(7).authorize().capture();
        assert_eq!(p.id(), 7);

        let mut d = PaymentDyn::new(7);
        assert_eq!(d.capture(), Err("wrong state")); // рантайм вместо компиляции
        assert!(d.authorize().is_ok());
    }

    #[test]
    fn guard_closes_on_early_return() {
        let mut m = Metrics { open_transactions: 0, log: vec![] };
        let _ = with_transaction_v1(&mut m, true);
        assert_eq!(m.open_transactions, 1); // утечка

        let mut m2 = Metrics { open_transactions: 0, log: vec![] };
        let _ = with_transaction_v2(&mut m2, true);
        assert_eq!(m2.open_transactions, 0); // баланс сохранён
    }

    #[test]
    fn dispatch() {
        let dynamic: Vec<Box<dyn Provider>> = vec![Box::new(Acme), Box::new(Globex)];
        assert_eq!(total_fee(&dynamic, 10_000), 100 + 210);
        assert_eq!(fee_static(&Acme, 10_000), 100);
        assert_eq!(ProviderEnum::Globex.fee_minor_units(10_000), 210);
        assert_eq!(label_v2("x"), "[x]");
    }
}
