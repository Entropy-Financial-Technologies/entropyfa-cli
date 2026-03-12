#[derive(Debug)]
pub enum ComputeError {
    ValidationError(Vec<String>),
    NotFound(String),
    InternalError(String),
}
