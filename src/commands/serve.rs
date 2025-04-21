use std::{fs, io, path::PathBuf, sync::Arc, vec};

use axum::{
    body::Bytes, extract::State, http::{header, HeaderMap, HeaderValue, Response, StatusCode}, response::{Html, IntoResponse}, routing::get, Json, Router
};
use clap::Args;
use serde_json::Value;
use tokio::sync::RwLock;

use crate::{
    error::CliError,
    executable::{Executable, ExecutionContext},
};

#[derive(Debug, Args)]
pub struct ServeCmd {
    pub schema: PathBuf,
    #[arg(short, long, default_value_t = 3000)]
    pub port: u16,
    /// Temporarily attach a wasm binary to this session
    #[arg(short, long)]
    pub wasm: Option<PathBuf>,
}


#[derive(Clone, Debug)]
pub struct RouterData {
    pub wasm_bytes: Option<Vec<u8>>
}

impl Executable for ServeCmd {
    async fn execute(&self, ctx: &ExecutionContext) -> Result<(), CliError> {
        let dir_string = self.schema.to_string_lossy().to_string();
        let spec = std::fs::read(&self.schema)?;
        let json_spec: Value = serde_json::from_slice(&spec)?;

        
        let port = self.port;
        let swagger = crate::consts::SWAGGER_UI;
        
        // Wasm binary interpretation
        let wasm_bytes = if let Some(wasm_path) = &self.wasm {
            std::fs::read(wasm_path).ok()
        } else {
            if let Some(val) = json_spec.get("x-wasm") {
                z85::decode(val.as_str().unwrap()).ok()
            } else {
                None
            }
        };

        let state = RouterData { wasm_bytes };

        let app = Router::new()
        .route(
            "/",
            get(async || -> Html<String> {
                Html(String::from_utf8(swagger.to_vec()).unwrap())
            }),
        )
        .route("/api", get(async || -> Json<Value> { Json(json_spec) }))
        .route("/wasm", get(serve_wasm))
        .nest(
            "/dist",
            Router::new()
            .route("/swagger-ui.css", get(serve_swagger_css))
            .route("/swagger-ui-bundle.js", get(serve_swagger_bundle))
            .route("/swagger-ui-es-bundle.js", get(serve_swagger_es_bundle))
            .route("/swagger-ui.js", get(serve_swagger_js))
            .route(
                "/swagger-ui-standalone-preset.js",
                get(serve_swagger_standalone),
            ),
        )
        .with_state(state);

        // run our app with hyper, listening globally on port 3000
        let listener = tokio::net::TcpListener::bind(&format!("0.0.0.0:{port}"))
            .await
            .unwrap();

        println!();
        println!("#=------------------------------#-----------------------------=#");
        println!("# Serving the OpenAPI spec at:\t#\t http://localhost:{port} #");
        println!("#=------------------------------#-----------------------------=#");

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

async fn serve_swagger_js() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/javascript".parse().unwrap());
    (headers, crate::consts::SWAGGER_JS)

}

async fn serve_wasm(State(state): State<RouterData>) -> impl IntoResponse {
    let lock = &state;
    let error_msg = b"Wasm bytecode not provided.".to_vec();
    let body = lock.wasm_bytes.as_ref().unwrap_or(&error_msg).as_slice().to_owned();
    (
        if lock.wasm_bytes.is_none() {
            StatusCode::NOT_FOUND
        }
        else {
            StatusCode::OK
        },
        Bytes::from(body)
    )
}