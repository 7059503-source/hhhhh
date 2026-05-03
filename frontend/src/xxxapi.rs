use serde::{Deserialize, Serialize};
use leptos::*;
use wasm_bindgen::JsCast;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: Option<String>,
    pub data: Option<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
}

pub fn event_target_value(ev: &leptos::ev::Event) -> String {
    ev.target()
        .expect("target should exist")
        .dyn_into::<web_sys::HtmlInputElement>()
        .expect("target should be an input element")
        .value()
}

// Token管理函数
pub fn get_token() -> Option<String> {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(Some(token)) = storage.get_item("token") {
                return Some(token);
            }
        }
    }
    None
}

pub fn set_token(token: String) {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.set_item("token", &token);
        }
    }
}

pub fn clear_token() {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.remove_item("token");
        }
    }
}

// API函数
pub async fn api_login(req: &LoginRequest) -> Result<AuthResponse, String> {
    let client = reqwest::Client::new();
    let resp = client
        .post("http://localhost:3000/api/login")
        .json(&req)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    let api_resp: ApiResponse<AuthResponse> = resp.json().await.map_err(|e| e.to_string())?;
    if api_resp.success {
        api_resp.data.ok_or_else(|| "No data".to_string())
    } else {
        Err(api_resp.message.unwrap_or_else(|| "登录失败".to_string()))
    }
}

pub async fn api_register(req: &RegisterRequest) -> Result<AuthResponse, String> {
    let client = reqwest::Client::new();
    let resp = client
        .post("http://localhost:3000/api/register")
        .json(&req)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    let api_resp: ApiResponse<AuthResponse> = resp.json().await.map_err(|e| e.to_string())?;
    if api_resp.success {
        api_resp.data.ok_or_else(|| "No data".to_string())
    } else {
        Err(api_resp.message.unwrap_or_else(|| "注册失败".to_string()))
    }
}

pub async fn api_get_users() -> Result<Vec<User>, String> {
    let client = reqwest::Client::new();
    let token = get_token().ok_or("未登录".to_string())?;
    
    let resp = client
        .get("http://localhost:3000/api/users")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    let api_resp: ApiResponse<Vec<User>> = resp.json().await.map_err(|e| e.to_string())?;
    if api_resp.success {
        api_resp.data.ok_or_else(|| "No data".to_string())
    } else {
        Err(api_resp.message.unwrap_or_else(|| "获取用户列表失败".to_string()))
    }
}

pub async fn api_delete_user(user_id: i64) -> Result<(), String> {
    let client = reqwest::Client::new();
    let token = get_token().ok_or("未登录".to_string())?;
    
    let resp = client
        .delete(format!("http://localhost:3000/api/users/{}", user_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    let api_resp: ApiResponse<()> = resp.json().await.map_err(|e| e.to_string())?;
    if api_resp.success {
        Ok(())
    } else {
        Err(api_resp.message.unwrap_or_else(|| "删除用户失败".to_string()))
    }
}
