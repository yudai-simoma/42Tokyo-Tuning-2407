use crate::{
    domains::order_service::OrderService,
    errors::AppError,
    repositories::{
        auth_repository::AuthRepositoryImpl, map_repository::MapRepositoryImpl,
        order_repository::OrderRepositoryImpl, tow_truck_repository::TowTruckRepositoryImpl,
    },
};
use actix_web::{web, HttpResponse};

/// 完了した注文を取得するハンドラー関数
/// 
/// `service` - 注文サービスのインスタンス
/// 
/// 成功した場合、HTTP 200 OK レスポンスと完了した注文のリストを返す
/// 失敗した場合、AppError を返す
/// 
/// ボトルネックになりうる箇所: データベースからの大量データ取得
/// - 完了した注文のリストを取得する処理は、データベースへのアクセスを伴うため、非同期処理として実装されている
pub async fn result_handler(
    service: web::Data<
        OrderService<
            OrderRepositoryImpl,
            TowTruckRepositoryImpl,
            AuthRepositoryImpl,
            MapRepositoryImpl,
        >,
    >,
) -> Result<HttpResponse, AppError> {
    match service.get_completed_orders().await {
        Ok(completed_orders) => Ok(HttpResponse::Ok().json(completed_orders)),
        Err(err) => Err(err),
    }
}