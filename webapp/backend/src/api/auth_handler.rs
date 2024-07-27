use crate::domains::auth_service::AuthService;
use crate::domains::dto::auth::{LoginRequestDto, LogoutRequestDto, RegisterRequestDto};
use crate::errors::AppError;
use crate::repositories::auth_repository::AuthRepositoryImpl;
use actix_web::{web, HttpResponse};

/// ユーザー登録を処理するハンドラー関数
/// 
/// `service` - ユーザー登録サービスのインスタンス
/// `req` - 登録リクエストのデータ
/// 
/// 成功した場合、HTTP 201 Created レスポンスを返す
/// 失敗した場合、AppError を返す
pub async fn register_handler(
    service: web::Data<AuthService<AuthRepositoryImpl>>,
    req: web::Json<RegisterRequestDto>,
) -> Result<HttpResponse, AppError> {
    match service
        .register_user(&req.username, &req.password, &req.role, req.area_id)
        .await
    {
        Ok(response) => Ok(HttpResponse::Created().json(response)),
        Err(err) => Err(err),
    }
}

/// ユーザーログインを処理するハンドラー関数
/// 
/// `service` - ユーザーログインサービスのインスタンス
/// `req` - ログインリクエストのデータ
/// 
/// 成功した場合、HTTP 200 OK レスポンスを返す
/// 失敗した場合、AppError を返す
pub async fn login_handler(
    service: web::Data<AuthService<AuthRepositoryImpl>>,
    req: web::Json<LoginRequestDto>,
) -> Result<HttpResponse, AppError> {
    match service.login_user(&req.username, &req.password).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(err) => Err(err),
    }
}

/// ユーザーログアウトを処理するハンドラー関数
/// 
/// `service` - ユーザーログアウトサービスのインスタンス
/// `req` - ログアウトリクエストのデータ
/// 
/// 成功・失敗に関わらず、HTTP 200 OK レスポンスを返す
pub async fn logout_handler(
    service: web::Data<AuthService<AuthRepositoryImpl>>,
    req: web::Json<LogoutRequestDto>,
) -> Result<HttpResponse, AppError> {
    match service.logout_user(&req.session_token).await {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Ok(HttpResponse::Ok().finish()),
    }
}

/// ユーザープロフィール画像を取得するハンドラー関数
/// 
/// `service` - プロフィール画像取得サービスのインスタンス
/// `path` - ユーザーIDのパスパラメータ
/// 
/// 成功した場合、HTTP 200 OK レスポンスと画像データを返す
/// 失敗した場合、AppError を返す
/// 
/// ボトルネックになりうる箇所: 画像のリサイズ処理
/// - 画像のリサイズ処理は計算リソースを多く消費する可能性があるため、非同期処理として実装されている
pub async fn user_profile_image_handler(
    service: web::Data<AuthService<AuthRepositoryImpl>>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();
    let profile_image_byte = service.get_resized_profile_image_byte(user_id).await?;
    Ok(HttpResponse::Ok()
        .content_type("image/png")
        .body(profile_image_byte))
}