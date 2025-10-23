use axum::{
    extract::{State, Query, Path},
    routing::{get, post},
    Json, Router,
};
use utoipa::IntoParams;
use serde::Deserialize;
use uuid::Uuid;
use serde_json::json;

use crate::{
    memory_store::{User, CreateUser, LoginUser, LoginResponse, UserResponse, MemoryStore},
    error::AppError,
    utils::{hash_password, verify_password},
    auth::JwtService,
};

#[derive(Clone)]
pub struct AppState {
    pub store: MemoryStore,
    pub jwt_secret: String,
}

#[derive(Deserialize, IntoParams)]
pub struct Pagination {
    pub page: Option<u32>,
    pub size: Option<u32>,
}

/// 用户路由注册
pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/users", get(get_users_paginated))
        .route("/users/:id", get(get_user).delete(delete_user))
        .route("/register", post(register_user))
        .route("/login", post(login_user))
}

/// 获取用户列表（分页）
#[utoipa::path(
    get,
    path = "/users",
    params(Pagination),
    responses(
        (status = 200, description = "获取用户列表成功", body = [User])
    )
)]
async fn get_users_paginated(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<serde_json::Value>, AppError> {
    let page = pagination.page.unwrap_or(1).max(1);
    let size = pagination.size.unwrap_or(10).min(100);

    let (users, total) = state.store.get_all_users(page, size).await;
    let pages = (total as f64 / size as f64).ceil() as u32;

    Ok(Json(json!({
        "users": users.into_iter().map(UserResponse::from).collect::<Vec<_>>(),
        "pagination": {
            "page": page,
            "size": size,
            "total": total,
            "pages": pages
        }
    })))
}

/// 根据ID获取用户
async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserResponse>, AppError> {
    let user = state.store.get_user_by_id(id).await
        .ok_or(AppError::NotFound)?;

    Ok(Json(UserResponse::from(user)))
}

/// 用户注册
#[utoipa::path(
    post,
    path = "/register",
    request_body = CreateUser,
    responses(
        (status = 201, description = "用户注册成功", body = UserResponse),
        (status = 400, description = "请求参数错误")
    )
)]
async fn register_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<UserResponse>, AppError> {
    // 哈希密码
    let password_hash = hash_password(&payload.password)?;

    let new_user = User {
        id: Uuid::new_v4(),
        username: payload.username,
        email: payload.email,
        password: password_hash,
        created_at: chrono::Utc::now(),
    };

    // 保存到内存存储
    let user = state.store.create_user(new_user).await
        .map_err(|e| AppError::BadRequest(e))?;

    Ok(Json(UserResponse::from(user)))
}

/// 用户登录
#[utoipa::path(
    post,
    path = "/login",
    request_body = LoginUser,
    responses(
        (status = 200, description = "登录成功", body = LoginResponse),
        (status = 401, description = "认证失败")
    )
)]
async fn login_user(
    State(state): State<AppState>,
    Json(payload): Json<LoginUser>,
) -> Result<Json<LoginResponse>, AppError> {
    let user = state.store.get_user_by_email(&payload.email).await
        .ok_or(AppError::AuthError("用户不存在".to_string()))?;

    // 验证密码
    if !verify_password(&payload.password, &user.password)? {
        return Err(AppError::AuthError("密码错误".to_string()));
    }

    // 生成JWT token
    let jwt_service = JwtService::new(state.jwt_secret.clone());
    let token = jwt_service.create_token(&user.id.to_string(), 3600)?;

    Ok(Json(LoginResponse {
        token,
        user: UserResponse::from(user),
    }))
}

/// 删除用户
async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    let deleted = state.store.delete_user(id).await;
    
    if !deleted {
        return Err(AppError::NotFound);
    }

    Ok(Json(json!({"status": "success", "message": "用户删除成功"})))
}