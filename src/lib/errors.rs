
#[derive(Debug, thiserror::Error)]
pub enum GogError {
    #[error(r#"Error while running "go init"#)]
    GogInitError,
    #[error("Error while parsing config file")]
    ConfigFileParseError(#[from] serde_json::Error),
    #[error("Command or Arg not found")]
    GogNotFound,
    #[error("Open Browser Error")]
    GogBrowserError(#[from] std::io::Error)
}