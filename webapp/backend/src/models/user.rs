use sqlx::FromRow;

/// ユーザーを表す構造体
#[derive(FromRow, Clone, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub profile_image: String,
    pub role: String,
}

/// セッションを表す構造体
#[derive(FromRow, Clone, Debug)]
pub struct Session {
    pub id: i32,
    pub user_id: i32,
    pub session_token: String,
    pub is_valid: bool,
}

/// ドライバーを表す構造体
#[derive(FromRow, Clone, Debug)]
pub struct Driver {
    pub id: i32,
    pub user_id: i32,
    pub session_token: String,
    pub is_valid: bool,
}

/// ディスパッチャーを表す構造体
#[derive(FromRow, Clone, Debug)]
pub struct Dispatcher {
    pub id: i32,
    pub user_id: i32,
    pub area_id: i32,
}