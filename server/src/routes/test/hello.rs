use poem::handler;
use tracing::trace;

#[handler]
pub fn hello() -> String {
    let hello = "hello world!".to_string();
    trace!("{}", &hello);

    hello.to_owned()
}
