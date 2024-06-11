//https://github.com/poem-web/poem/discussions/280

use minijinja::context;
use minijinja::value::Value;
use minijinja::Environment as Minijinja;
use poem::{http::StatusCode, Endpoint, IntoResponse, Middleware, Request, Response, Result};
use tracing::error;

pub struct ErrorMiddleware {
    #[cfg(not(feature = "hot-reload"))]
    pub templates: &'static Minijinja<'static>,
    #[cfg(feature = "hot-reload")]
    pub templates: &'static arc_swap::ArcSwap<Minijinja<'static>>,
}

impl<E: Endpoint> Middleware<E> for ErrorMiddleware {
    type Output = ErrorMiddlewareImpl<E>;

    fn transform(&self, ep: E) -> Self::Output {
        ErrorMiddlewareImpl {
            ep,
            templates: self.templates,
        }
    }
}

pub struct ErrorMiddlewareImpl<E> {
    ep: E,
    #[cfg(not(feature = "hot-reload"))]
    pub templates: &'static Minijinja<'static>,
    #[cfg(feature = "hot-reload")]
    pub templates: &'static arc_swap::ArcSwap<Minijinja<'static>>,
}

impl<E: Endpoint> ErrorMiddlewareImpl<E> {
    #[cfg(not(feature = "hot-reload"))]
    fn get_templates(&self) -> &'static Minijinja<'static> {
        self.templates
    }
    #[cfg(feature = "hot-reload")]
    fn get_templates(&self) -> arc_swap::Guard<std::sync::Arc<Minijinja<'static>>> {
        self.templates.load()
    }
    fn render_template_response(
        &self,
        ctx: Value,
        template_path: &'static str,
        status_code: StatusCode,
        fallback_msg: &'static str,
    ) -> Result<Response> {
        let templates = self.get_templates();

        let template = match templates.get_template(template_path) {
            Ok(res) => res,
            Err(err) => {
                error!("could not get template {template_path} {err}");
                return Ok(Response::builder().status(status_code).body(fallback_msg));
            }
        };

        let template_rendered = match template.render(&ctx) {
            Ok(res) => res,
            Err(err) => {
                error!("could not render template {template_path} {err}");
                return Ok(Response::builder().status(status_code).body(fallback_msg));
            }
        };
        Ok(Response::builder()
            .status(status_code)
            .body(template_rendered))
    }
}

impl<E: Endpoint> Endpoint for ErrorMiddlewareImpl<E> {
    type Output = Response;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        let uri = req.uri().clone();
        let resp = self.ep.call(req).await;
        // let method = req.method().clone();
        // let start = Instant::now();
        // let elapsed = start.elapsed();
        match resp {
            Ok(resp) => Ok(resp.into_response()),
            Err(e) => {
                if e.is::<poem::error::NotFoundError>() {
                    let uri_path = &uri.path();
                    let ctx = context! { uri => &uri_path};
                    let template_path = "error/404.jinja";
                    let status_code = StatusCode::NOT_FOUND;
                    let fallback_msg = "not found";
                    return self.render_template_response(
                        ctx,
                        template_path,
                        status_code,
                        fallback_msg,
                    );
                }
                if e.is::<poem::error::MethodNotAllowedError>() {
                    //TODO: create template
                    return Ok(Response::builder()
                        .status(StatusCode::METHOD_NOT_ALLOWED)
                        .body("not allowed"));
                }

                Ok(e.into_response())
            }
        }
    }
}
