use std::fmt;

use poem::{error::ResponseError, http::StatusCode, Response};
use tracing::{event, Level};

#[derive(Debug, thiserror::Error)]
pub enum ContextualError {
    InnerError(#[from] anyhow::Error),
}

//https://old.reddit.com/r/rust/comments/viax3s/in_rust_why_can_we_not_define_an_impl_for_a_type/
pub trait CtxtErrExt<T> {
    fn map_ctxt_err(self) -> Result<T, ContextualError>;
}

impl<T> CtxtErrExt<T> for Result<T, anyhow::Error> {
    fn map_ctxt_err(self) -> Result<T, ContextualError> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(ContextualError::from(e)),
        }
    }
}

impl fmt::Display for ContextualError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ContextualError::InnerError(error) => write!(f, "{}", error),
        }
    }
}

impl ResponseError for ContextualError {
    fn status(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn as_response(&self) -> poem::Response
    where
        Self: std::error::Error + Send + Sync + 'static,
    {
        match self {
            //https://github.com/Keats/tera/blob/master/src/renderer/tests/errors.rs#L12
            ContextualError::InnerError(error) => {
                let errs = error
                    .chain()
                    .map(|e| format!("{}", e))
                    .collect::<Vec<String>>();

                let body = format!("{:?}", error) + "\n" + &errs.join("\n");
                let log = format!("{:?}", error) + ";" + &errs.join(";");

                event!(Level::ERROR, "{log}");
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(body)
            }
        }
    }
}
