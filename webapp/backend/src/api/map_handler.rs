use crate::{
    domains::{dto::map::UpdateEdgeRequestDto, map_service::MapService},
    errors::AppError,
    repositories::map_repository::MapRepositoryImpl,
};
use actix_web::{web, HttpResponse};

/// エッジ更新リクエストを処理するハンドラー関数
/// 
/// `service` - エッジ更新サービスのインスタンス
/// `req` - エッジ更新リクエストのデータ
/// 
/// 成功した場合、HTTP 200 OK レスポンスを返す
/// 失敗した場合、AppError を返す
/// 
/// ボトルネックになりうる箇所: データベース操作
/// - エッジの更新処理はデータベースへのアクセスを伴うため、非同期処理として実装されている
pub async fn update_edge_handler(
    service: web::Data<MapService<MapRepositoryImpl>>,
    req: web::Json<UpdateEdgeRequestDto>,
) -> Result<HttpResponse, AppError> {
    match service
        .update_edge(req.node_a_id, req.node_b_id, req.weight)
        .await
    {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(err) => Err(err),
    }
}