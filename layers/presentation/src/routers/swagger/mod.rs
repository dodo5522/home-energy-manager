use poem_openapi::{OpenApi, payload::PlainText};

pub struct Api;

#[OpenApi]
impl Api {
    /// Hello OpenAPIを返すAPI
    #[oai(path = "/", method = "get")]
    async fn hello(&self) -> PlainText<String> {
        PlainText("Hello, OpenAPI!".to_string())
    }
}
