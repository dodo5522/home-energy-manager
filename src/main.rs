use poem::{EndpointExt, Server, listener::TcpListener, middleware::Tracing};
use std::env::var;

use layer_presentation::{di, routers};

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

/// Run the application server.
async fn run() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let bind_addr = var("BIND_ADDR")?;
    let bind_port = var("BIND_PORT")?;

    tracing_subscriber::fmt::init();

    // run our app with hyper, listening globally on the port
    let router = routers::get();
    Server::new(TcpListener::bind(format!("{bind_addr}:{bind_port}")))
        .run(router.with(Tracing))
        .await?;

    Ok(())
}
