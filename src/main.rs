use layer_infra_db::get_connection;
use layer_presentation::get_router;
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
    dotenvy::dotenv()?;

    let bind_addr = var("BIND_ADDR")?;
    let bind_port = var("BIND_PORT")?;
    let user = var("DB_OPERATOR_NAME")?;
    let password = var("DB_OPERATOR_PASSWORD")?;

    tracing_subscriber::fmt::init();

    // connect db
    let db = get_connection(&user, &password).await?;
    println!("{:?}", db);

    // run our app with hyper, listening globally on the port
    let router = get_router();
    let listener = tokio::net::TcpListener::bind(format!("{bind_addr}:{bind_port}")).await?;
    axum::serve(listener, router).await?;

    Ok(())
}
