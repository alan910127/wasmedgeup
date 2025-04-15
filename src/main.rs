use clap::Parser;
use wasmedgeup::cli::Cli;
use wasmedgeup::cli::CommandExecutor;
use wasmedgeup::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    let ctx = cli.context(Default::default());

    if let Some(command) = cli.commands {
        command.execute(ctx).await?;
    };

    Ok(())
}
