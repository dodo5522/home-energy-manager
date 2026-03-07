use layer_presentation::routers;
use std::env::var;

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

/// Run the application server.
async fn run() -> anyhow::Result<()> {
    let bind_addr = var("BIND_ADDR")?;
    let bind_port = var("BIND_PORT")?;
    let address = format!("{bind_addr}:{bind_port}");
    let allowed_origins: Vec<String> = var("ALLOWED_ORIGINS")?
        .split(",")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.into())
        .collect();

    tracing_subscriber::fmt::init();

    // run our app with hyper, listening globally on the port
    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, routers::route(allowed_origins)).await?;

    Ok(())
}
