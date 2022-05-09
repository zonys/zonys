pub fn try_catch<O, E, S, F>(function: S, on_failure: F) -> Result<O, E>
where
    S: FnOnce() -> Result<O, E>,
    F: FnOnce(&E),
{
    match function() {
        Ok(o) => Ok(o),
        Err(e) => {
            on_failure(&e);
            Err(e)
        }
    }
}
