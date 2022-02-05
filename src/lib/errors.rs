
#[derive(Debug, thiserror::Error)]
pub enum GogError {
    #[error(r#"Error while running "go init"#)]
    GogInitError,
    #[error("Command or Arg not found")]
    GogNotFound,
    #[error("Open Browser Error")]
    GogBrowserError(#[from] std::io::Error)
}