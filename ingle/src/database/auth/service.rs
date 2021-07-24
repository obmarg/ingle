//! Authentication service.

use std::sync::Arc;
use std::task::{Context, Poll};
use tonic::codegen::http::{header::AUTHORIZATION, HeaderValue, Request};
use tonic::metadata::MetadataValue;
use tower_service::Service;

use super::Credentials;

#[derive(Debug, Clone)]
pub struct AuthService<S> {
    inner: S,
    credentials: Option<Credentials>,
}

impl<S> AuthService<S> {
    #[inline]
    pub fn new(inner: S, credentials: Option<Credentials>) -> Self {
        Self { inner, credentials }
    }
}

impl<S, Body, Response> Service<Request<Body>> for AuthService<S>
where
    S: Service<Request<Body>, Response = Response>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    #[inline]
    fn call(&mut self, mut request: Request<Body>) -> Self::Future {
        match &self.credentials {
            Some(Credentials::ServiceAccount(token)) => {
                let jwt = token.jwt();
                request.headers_mut().insert(
                    AUTHORIZATION,
                    HeaderValue::from_str(&format!("Bearer {}", jwt)).unwrap(),
                );
            }
            Some(Credentials::AuthToken(token)) => {
                request.headers_mut().insert(
                    AUTHORIZATION,
                    HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
                );
            }
            Some(Credentials::EmulatorOwner) => {
                request.headers_mut().insert(
                    AUTHORIZATION,
                    HeaderValue::from_str("Bearer owner").unwrap(),
                );
            }
            _ => {}
        }

        self.inner.call(request)
    }
}
