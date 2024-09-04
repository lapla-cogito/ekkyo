#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct ConfigParseErr {
    #[from]
    src: anyhow::Error,
}

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct ConnectionErr {
    #[from]
    src: anyhow::Error,
}

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct ConvertMessageErr {
    #[from]
    src: anyhow::Error,
}

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct ConvertBytesErr {
    #[from]
    src: anyhow::Error,
}
