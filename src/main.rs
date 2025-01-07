mod app_state;
mod cli;
mod logging;
mod server;
mod settings;
mod storage;

use cli::Cli;
use logging::setup_logging;

#[tokio::main]
async fn main() {
    let cli = Cli::initialize();
    let settings = cli.load_settings().unwrap();
    setup_logging(&settings.log_level.parse().expect("Log level not parsing")).unwrap();

    cli.handle(settings).await.unwrap();
}
