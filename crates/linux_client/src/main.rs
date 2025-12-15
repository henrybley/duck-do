mod config;
mod inbound;
mod outbound;

use crate::{config::Config, inbound::cli::CliServer, outbound::sqlite::Sqlite};
use client_core::task::impls::Service;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env()?;

    let sqlite = Sqlite::new(&config.database_url).await?;
    let task_service = Service::new(sqlite);

    let cli_server = CliServer::new(task_service).await?;

    cli_server.run().await
}
