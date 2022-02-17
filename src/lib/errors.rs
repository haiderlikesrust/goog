

#[derive(Debug, thiserror::Error)]
pub enum GoogError {
    #[error(r#"Error while running "go init"#)]
    GoogInitError,
    #[error("Command or Arg not found")]
    GoogNotFound,
    #[error("Open Browser Error")]
    GoogBrowserError(#[from] std::io::Error),
    #[error("Missing a required Arg")]
    GoogArgNotSupplied
}
