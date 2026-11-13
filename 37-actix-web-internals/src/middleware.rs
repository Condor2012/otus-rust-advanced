//! Два способа написать middleware.
use std::{
    future::{Ready, ready},
    rc::Rc,
    sync::atomic::{AtomicU64, Ordering},
    time::Instant,
};

use actix_web::{
    Error,
    body::MessageBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    http::header::{HeaderName, HeaderValue},
    middleware::Next,
};
use futures_util::future::LocalBoxFuture;

// ---------- 1. middleware::from_fn — async-функция ----------
pub async fn timing(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let started = Instant::now();
    let mut res = next.call(req).await?;
    let us = started.elapsed().as_micros();
    res.headers_mut().insert(
        HeaderName::from_static("x-response-time-us"),
        HeaderValue::from(us as u64),
    );
    Ok(res)
}

// ---------- 2. Transform + Service — «как устроено внутри» ----------
static NEXT_ID: AtomicU64 = AtomicU64::new(1);

/// Фабрика middleware: вызывается по разу на каждый воркер.
pub struct RequestId;

impl<S, B> Transform<S, ServiceRequest> for RequestId
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RequestIdMw<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestIdMw { service: Rc::new(service) }))
    }
}

/// Сам сервис-обёртка. Rc, а не Arc: живёт внутри одного воркера.
pub struct RequestIdMw<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for RequestIdMw<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    // poll_ready просто делегируем внутреннему сервису.
    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        let svc = Rc::clone(&self.service);
        Box::pin(async move {
            let mut res = svc.call(req).await?;
            res.headers_mut()
                .insert(HeaderName::from_static("x-request-id"), HeaderValue::from(id));
            Ok(res)
        })
    }
}
