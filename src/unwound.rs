pub fn get_unwound_error(err: Box<dyn Any + Send>) -> anyhow::Error {
    if err.is::<String>() {
        if let Ok(err) = err.downcast::<String>() {
            anyhow::anyhow!("Unhandled Error: {:?}", err)
        } else {
            anyhow::anyhow!("Unhandled Error!")
        }
    } else if err.is::<&str>() {
        if let Ok(err) = err.downcast::<&str>() {
            anyhow::anyhow!("Unhandled Error: {:?}", err)
        } else {
            anyhow::anyhow!("Unhandled Error!")
        }
    } else {
        anyhow::anyhow!("Unhandled Error: {:?}", err)
    }
}
