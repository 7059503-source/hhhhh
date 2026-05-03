use crate::auth::{create_token, verify_token};
use crate::db::DbPool;
use crate::models::{ApiResponse, AuthResponse, LoginRequest, RegisterRequest, UserResponse};
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    Json,
};

pub async fn register(
    State(pool): State<DbPool>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, Json<ApiResponse<()>>> {
    let password_hash = match bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST) {
        Ok(hash) => hash,
        Err(_) => {
            return Err(Json(ApiResponse::<()>::error("Failed to hash password".to_string())))
        }
    };

    let result = sqlx::query(
        "INSERT INTO users (username, email, password_hash) VALUES (?, ?, ?)",
    )
    .bind(&payload.username)
    .bind(&payload.email)
    .bind(&password_hash)
    .execute(&*pool)
    .await;

    match result {
        Ok(res) => {
            let user_id = res.last_insert_rowid();
            let token = match create_token(user_id, &payload.username) {
                Ok(t) => t,
                Err(_) => {
                    return Err(Json(ApiResponse::<()>::error("Failed to create token".to_string())))
                }
            };

            Ok(Json(ApiResponse::success(AuthResponse {
                token,
                user: UserResponse {
                    id: user_id,
                    username: payload.username,
                    email: payload.email,
                },
            })))
        }
        Err(e) => {
            let msg = if e.to_string().contains("UNIQUE constraint failed: users.username") {
                "Username already exists"
            } else if e.to_string().contains("UNIQUE constraint failed: users.email") {
                "Email already exists"
            } else {
                "Registration failed"
            };
            Err(Json(ApiResponse::<()>::error(msg.to_string())))
        }
    }
}

pub async fn login(
    State(pool): State<DbPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, Json<ApiResponse<()>>> {
    let user_result: Result<Option<(i64, String, String, String)>, _> = sqlx::query_as(
        "SELECT id, username, email, password_hash FROM users WHERE username = ?",
    )
    .bind(&payload.username)
    .fetch_optional(&*pool)
    .await;

    match user_result {
        Ok(Some((id, username, email, password_hash))) => {
            if bcrypt::verify(&payload.password, &password_hash).unwrap_or(false) {
                let token = match create_token(id, &username) {
                    Ok(t) => t,
                    Err(_) => {
                        return Err(Json(ApiResponse::<()>::error("Failed to create token".to_string())))
                    }
                };

                let user_response = UserResponse {
                    id,
                    username: username.clone(),
                    email,
                };

                Ok(Json(ApiResponse::success(AuthResponse {
                    token,
                    user: user_response,
                })))
            } else {
                Err(Json(ApiResponse::<()>::error("Invalid password".to_string())))
            }
        }
        Ok(None) => Err(Json(ApiResponse::<()>::error("User not found".to_string()))),
        Err(_) => Err(Json(ApiResponse::<()>::error("Login failed".to_string()))),
    }
}

pub async fn get_user(
    State(pool): State<DbPool>,
    Path(user_id): Path<i64>,
    headers: HeaderMap,
) -> Result<Json<ApiResponse<UserResponse>>, Json<ApiResponse<()>>> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok());

    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => &header[7..],
        _ => return Err(Json(ApiResponse::<()>::error("Missing or invalid authorization header".to_string()))),
    };

    if let Err(_) = verify_token(token) {
        return Err(Json(ApiResponse::<()>::error("Invalid or expired token".to_string())));
    }

    let user_result: Result<Option<(i64, String, String)>, _> = sqlx::query_as(
        "SELECT id, username, email FROM users WHERE id = ?",
    )
    .bind(user_id)
    .fetch_optional(&*pool)
    .await;

    match user_result {
        Ok(Some((id, username, email))) => Ok(Json(ApiResponse::success(UserResponse {
            id,
            username,
            email,
        }))),
        Ok(None) => Err(Json(ApiResponse::<()>::error("User not found".to_string()))),
        Err(_) => Err(Json(ApiResponse::<()>::error("Failed to fetch user".to_string()))),
    }
}

pub async fn get_all_users(
    State(pool): State<DbPool>,
    headers: HeaderMap,
) -> Result<Json<ApiResponse<Vec<UserResponse>>>, Json<ApiResponse<()>>> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok());

    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => &header[7..],
        _ => return Err(Json(ApiResponse::<()>::error("Missing or invalid authorization header".to_string()))),
    };

    if let Err(_) = verify_token(token) {
        return Err(Json(ApiResponse::<()>::error("Invalid or expired token".to_string())));
    }

    let users_result: Result<Vec<(i64, String, String)>, _> = sqlx::query_as(
        "SELECT id, username, email FROM users",
    )
    .fetch_all(&*pool)
    .await;

    match users_result {
        Ok(users) => {
            let user_responses: Vec<UserResponse> = users
                .into_iter()
                .map(|(id, username, email)| UserResponse {
                    id,
                    username,
                    email,
                })
                .collect();
            Ok(Json(ApiResponse::success(user_responses)))
        }
        Err(_) => Err(Json(ApiResponse::<()>::error("Failed to fetch users".to_string()))),
    }
}

pub async fn delete_user(
    State(pool): State<DbPool>,
    Path(user_id): Path<i64>,
    headers: HeaderMap,
) -> Result<Json<ApiResponse<String>>, Json<ApiResponse<()>>> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok());

    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => &header[7..],
        _ => return Err(Json(ApiResponse::<()>::error("Missing or invalid authorization header".to_string()))),
    };

    if let Err(_) = verify_token(token) {
        return Err(Json(ApiResponse::<()>::error("Invalid or expired token".to_string())));
    }

    let result = sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(user_id)
        .execute(&*pool)
        .await;

    match result {
        Ok(res) => {
            if res.rows_affected() > 0 {
                Ok(Json(ApiResponse::success("User deleted successfully".to_string())))
            } else {
                Err(Json(ApiResponse::<()>::error("User not found".to_string())))
            }
        }
        Err(_) => Err(Json(ApiResponse::<()>::error("Failed to delete user".to_string()))),
    }
}

pub async fn get_current_user(
    State(pool): State<DbPool>,
    headers: HeaderMap,
) -> Result<Json<ApiResponse<UserResponse>>, Json<ApiResponse<()>>> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok());

    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => &header[7..],
        _ => return Err(Json(ApiResponse::<()>::error("Missing or invalid authorization header".to_string()))),
    };

    let claims = match verify_token(token) {
        Ok(c) => c,
        Err(_) => return Err(Json(ApiResponse::<()>::error("Invalid or expired token".to_string()))),
    };

    let user_result: Result<Option<(i64, String, String)>, _> = sqlx::query_as(
        "SELECT id, username, email FROM users WHERE id = ?",
    )
    .bind(claims.sub)
    .fetch_optional(&*pool)
    .await;

    match user_result {
        Ok(Some((id, username, email))) => Ok(Json(ApiResponse::success(UserResponse {
            id,
            username,
            email,
        }))),
        Ok(None) => Err(Json(ApiResponse::<()>::error("User not found".to_string()))),
        Err(_) => Err(Json(ApiResponse::<()>::error("Failed to fetch user".to_string()))),
    }
}
