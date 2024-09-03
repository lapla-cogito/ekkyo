#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct ConfigParseErr {
    #[from]
    src: anyhow::Error,
}
