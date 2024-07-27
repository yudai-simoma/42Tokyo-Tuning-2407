use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand::Rng;

use crate::errors::AppError;

/// セッショントークンを生成する関数
///
/// ランダムな30文字の英数字からなるセッショントークンを生成します。
///
/// 戻り値:
/// - 生成されたセッショントークンの文字列
pub fn generate_session_token() -> String {
    let mut rng = rand::thread_rng();
    let token: String = (0..30)
        .map(|_| {
            let idx = rng.gen_range(0..62);
            let chars = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
            chars[idx] as char
        })
        .collect();
    token
}

/// パスワードをハッシュ化する関数
///
/// `password` - ハッシュ化するパスワードの文字列
///
/// 成功した場合はハッシュ化されたパスワードの文字列を返し、失敗した場合は `AppError` を返します。
///
/// 戻り値:
/// - `Result<String, AppError>`: 成功時はハッシュ化されたパスワード、失敗時はエラー
pub fn hash_password(password: &str) -> Result<String, AppError> {
    let password_bytes = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    match argon2.hash_password(password_bytes, &salt) {
        Ok(hashed_password_bytes) => Ok(hashed_password_bytes.to_string()),
        Err(_) => Err(AppError::InternalServerError),
    }
}

/// パスワードを検証する関数
///
/// `hashed_password` - ハッシュ化されたパスワードの文字列
/// `input_password` - 入力されたパスワードの文字列
///
/// 成功した場合は `true` を返し、失敗した場合は `false` を返します。
///
/// 戻り値:
/// - `Result<bool, AppError>`: 成功時は `true`、失敗時は `false`
pub fn verify_password(hashed_password: &str, input_password: &str) -> Result<bool, AppError> {
    let input_password_bytes = input_password.as_bytes();
    let parsed_hash = match PasswordHash::new(hashed_password) {
        Ok(hash) => hash,
        Err(_) => return Err(AppError::InternalServerError),
    };
    match Argon2::default().verify_password(input_password_bytes, &parsed_hash) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}