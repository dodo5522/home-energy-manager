use layer_infra_db::DatabaseConnector;
use layer_presentation::route;
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
    let db_connector = DatabaseConnector::new(
        var("DB_OPERATOR_NAME")?,
        var("DB_OPERATOR_PASSWORD")?,
        var("DB_HOST")?,
        var("DB_PORT")?,
        var("DB_NAME")?,
    );
    let bind_addr = var("BIND_ADDR")?;
    let bind_port = var("BIND_PORT")?;
    let address = format!("{bind_addr}:{bind_port}");

    #[allow(unused_mut)]
    let mut allowed_origins: Vec<String> = var("ALLOWED_ORIGINS")?
        .split(",")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.into())
        .collect();

    #[cfg(feature = "allow-localhost-access")]
    {
        eprintln!("debug feature enabled");
        allowed_origins.push("http://localhost:3000".to_string());
        allowed_origins.push("http://0.0.0.0:3000".to_string());
    }

    tracing_subscriber::fmt::init();

    // run our app with hyper, listening globally on the port
    let listener = tokio::net::TcpListener::bind(address).await?;
    let connection = db_connector.get_connection().await?;
    let router = route(allowed_origins, connection)?;
    axum::serve(listener, router).await?;

    Ok(())
}
