use anyhow::Context as _;

#[derive(Debug)]
pub struct Connection {
    connection: tokio::net::TcpStream,
}

impl Connection {
    pub async fn connect(
        config: &crate::config::Config,
    ) -> Result<Self, crate::error::ConnectionErr> {
        let connection = match config.mode {
            crate::config::Mode::Active => Self::connect_remote(config).await,
            crate::config::Mode::Passive => Self::accept_remote(config).await,
        }?;

        Ok(Self { connection })
    }

    async fn connect_remote(
        config: &crate::config::Config,
    ) -> anyhow::Result<tokio::net::TcpStream> {
        tracing::info!("connecting to remote peer {0}:{1}", config.remote_ip, 179);
        tokio::net::TcpStream::connect((config.remote_ip, 179))
            .await
            .context(format!(
                "failed to connect to remote peer {0}:{1}",
                config.remote_ip, 179
            ))
    }

    async fn accept_remote(
        config: &crate::config::Config,
    ) -> anyhow::Result<tokio::net::TcpStream> {
        let listener = tokio::net::TcpListener::bind((config.local_ip, 179))
            .await
            .context(format!(
                "failed to bind to local peer {0}:{1}",
                config.local_ip, 179
            ))?;

        let (connection, _) = listener
            .accept()
            .await
            .context("failed to accept connection")?;
        Ok(connection)
    }
}
