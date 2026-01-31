use layer_presentation::routers;
use poem::{EndpointExt, Server, listener::TcpListener, middleware::Tracing};
use poem_openapi::OpenApiService;
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

    let service_title = "Energy generation recorder";
    let service_version = "1.0.0";
    let bind_addr = var("BIND_ADDR")?;
    let bind_port = var("BIND_PORT")?;
    let address = format!("{bind_addr}:{bind_port}");

    tracing_subscriber::fmt::init();

    let api_server =
        OpenApiService::new(routers::Api, service_title, service_version).server(&address);
    let swagger = api_server.swagger_ui();
    let redoc = api_server.redoc();
    let json = api_server.spec_endpoint();

    // run our app with hyper, listening globally on the port
    let router = routers::get();
    Server::new(TcpListener::bind(&address))
        .run(
            router
                .nest("/redoc", redoc)
                .nest("/swagger", swagger)
                .nest("/openapi.json", json)
                .with(Tracing),
        )
        .await?;

    Ok(())
}
