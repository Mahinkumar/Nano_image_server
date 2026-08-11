


#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("empty path")]
    EmptyPath,
    #[error("unrecognized plugin: {0}")]
    UnknownPlugin(String),
    #[error("Missing parameter: {0}")]
    MissingParameter(String),
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),
}

