use std::{fs, io, path::PathBuf};

use axum::{
    http::{header, HeaderMap, HeaderValue, Response},
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use clap::Args;
use serde_json::Value;

use crate::{
    error::CliError,
    executable::{Executable, ExecutionContext},
};

#[derive(Debug, Args)]
pub struct ServeCmd {
    pub schema: PathBuf,
    #[arg(short, long, default_value_t = 3000)]
    pub port: u16,
}

impl Executable for ServeCmd {
    async fn execute(&self, ctx: &ExecutionContext) -> Result<(), CliError> {
        let dir_string = self.schema.to_string_lossy().to_string();
        let spec = std::fs::read(&self.schema)?;
        let json_spec: Value = serde_json::from_slice(&spec)?;
        let port = self.port;
        let swagger = crate::consts::SWAGGER_UI;

        // build our application with a single route
        let app = Router::new()
            .route(
                "/",
                get(async || -> Html<String> {
                    Html(String::from_utf8(swagger.to_vec()).unwrap())
                }),
            )
            .route("/api", get(async || -> Json<Value> { Json(json_spec) }))
            .nest(
                "/dist",
                Router::new()
                    .route("/swagger-ui.css", get(serve_swagger_css))
                    .route("/swagger-ui-bundle.js", get(serve_swagger_bundle))
                    .route("/swagger-ui-es-bundle.js", get(serve_swagger_es_bundle))
                    .route(
                        "/swagger-ui-standalone-preset.js",
                        get(serve_swagger_standalone),
                    ),
            );

        // run our app with hyper, listening globally on port 3000
        let listener = tokio::net::TcpListener::bind(&format!("0.0.0.0:{port}"))
            .await
            .unwrap();

        println!();
        println!("# ------------------------------------------------------------ #");
        println!("# Serving the OpenAPI spec at:\t\t http://localhost:{port} #");
        println!("# ------------------------------------------------------------ #");

        axum::serve(listener, app).await.unwrap();
        Ok(())
    }
}

async fn serve_swagger_css() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());
    (headers, crate::consts::SWAGGER_CSS)
}

async fn serve_swagger_bundle() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/javascript".parse().unwrap());
    (headers, crate::consts::SWAGGER_BUNDLE)
}

async fn serve_swagger_es_bundle() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/javascript".parse().unwrap());
    (headers, crate::consts::SWAGGER_ES_BUNDLE)
}

async fn serve_swagger_standalone() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/javascript".parse().unwrap());
    (headers, crate::consts::SWAGGER_STANDALONE)
}
