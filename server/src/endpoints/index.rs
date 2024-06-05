use poem::{handler, session::Session};
use tracing::trace;

#[handler]
pub fn index() -> String {
    let hello = format!("hello world!");
    trace!("{}", &hello);

    hello.to_owned()
}
