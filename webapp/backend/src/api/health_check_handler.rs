use crate::errors::AppError;
use actix_web::HttpResponse;
use serde::Serialize;

/// ヘルスチェックレスポンスの構造体
/// 
/// `Serialize` トレイトを導出することで、この構造体をJSONに自動的にシリアライズできるようにしています。
#[derive(Serialize)]
struct HealthCheckResponse {
    status: String,
}

/// ヘルスチェックを処理するハンドラー関数
/// 
/// このエンドポイントは、サービスが正常に動作しているかを確認するために使用されます。
/// 常に "OK" ステータスを持つ `HealthCheckResponse` を返します。
///
/// 戻り値:
/// - 成功時: HTTP 200 OK レスポンスと JSON 形式の `HealthCheckResponse`
/// - 失敗時: `AppError`（ただし、この関数では失敗することはありません）
pub async fn health_check_handler() -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(HealthCheckResponse {
        status: "OK".to_string(),
    }))
}