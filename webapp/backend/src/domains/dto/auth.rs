use serde::{Deserialize, Serialize};

// 入力データ構造

/// ユーザー登録リクエストのデータ構造
#[derive(Deserialize, Debug)]
pub struct RegisterRequestDto {
    pub username: String,
    pub password: String,
    pub role: String,
    pub area_id: Option<i32>,
}

/// ユーザーログインリクエストのデータ構造
#[derive(Deserialize, Debug)]
pub struct LoginRequestDto {
    pub username: String,
    pub password: String,
}

/// ユーザーログアウトリクエストのデータ構造
#[derive(Deserialize)]
pub struct LogoutRequestDto {
    pub session_token: String,
}

// 出力データ構造

/// ユーザーログインレスポンスのデータ構造
#[derive(Serialize)]
pub struct LoginResponseDto {
    pub user_id: i32,
    pub username: String,
    pub session_token: String,
    pub role: String,
    pub dispatcher_id: Option<i32>,
    pub area_id: Option<i32>,
}