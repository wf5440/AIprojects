mod error;
mod auth;
mod utils;
mod routes;
mod memory_store;

use axum::{Router, routing::get};
use tracing_subscriber;
// use utoipa_swagger_ui::SwaggerFile;
// use utoipa::OpenApi;

use crate::memory_store::MemoryStore;
use crate::routes::user::{user_routes, AppState};

// #[derive(OpenApi)]
// #[openapi(
//     paths(
//         routes::user::get_users_paginated,
//         routes::user::register_user,
//         routes::user::login_user
//     ),
//     components(schemas(memory_store::User, memory_store::CreateUser))
// )]
// struct ApiDoc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();
    
    // 创建内存存储
    let memory_store = MemoryStore::new();
    
    // 应用状态
    let app_state = AppState { 
        store: memory_store,
        jwt_secret: "your-super-secret-jwt-key".to_string(),
    };
    
    // 健康检查端点
    async fn health_check() -> &'static str {
        "OK"
    }
    
    // 构建路由
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/", get(|| async { "Rust Backend API" }))
        .merge(user_routes())
        .merge(SwaggerFile::new("/swagger-ui", ApiDoc::openapi()))
        .with_state(app_state);
    let addr = format!("0.0.0.0:{}", settings.port.unwrap_or(3000)); // 使用3000作为备用端口
    tracing::info!("服务器启动在 http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}