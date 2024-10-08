use axum::{body::Body, extract::Request, response::Response};
use std::task::{Context, Poll};
use tower::{Layer, Service};

type BoxFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;

#[derive(Clone)]
pub struct TracingLayer;

impl<S> Layer<S> for TracingLayer {
    type Service = TracingMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        TracingMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct TracingMiddleware<S> {
    inner: S,
}

impl<S> Service<Request> for TracingMiddleware<S>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    // `BoxFuture` is a type alias for `Pin<Box<dyn Future + Send + 'a>>`
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let future = self.inner.call(request);
        Box::pin(async move {
            #[cfg(feature = "log")]
            let log_subscription = dxp_logging::get_subscription();
            let response: Response = future.await?;
            #[cfg(feature = "log")]
            drop(log_subscription);
            Ok(response)
        })
    }
}
