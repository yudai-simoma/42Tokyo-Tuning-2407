use crate::domains::tow_truck_service::TowTruckService;
use crate::errors::AppError;
use crate::repositories::order_repository::OrderRepositoryImpl;
use crate::repositories::tow_truck_repository::TowTruckRepositoryImpl;
use crate::{
    domains::dto::tow_truck::UpdateLocationRequestDto,
    repositories::map_repository::MapRepositoryImpl,
};
use actix_web::{web, HttpResponse};
use serde::Deserialize;

/// ページネーションされたレッカー車リストを取得するためのクエリパラメータ
#[derive(Deserialize, Debug)]
pub struct PaginatedTowTruckQuery {
    page: Option<i32>,
    page_size: Option<i32>,
    status: Option<String>,
    area: Option<i32>,
}

/// ページネーションされたレッカー車リストを取得するハンドラー関数
/// 
/// `service` - レッカー車サービスのインスタンス
/// `query` - ページネーションとフィルタリングのクエリパラメータ
/// 
/// 成功した場合、HTTP 200 OK レスポンスとレッカー車リストを返す
/// 失敗した場合、AppError を返す
/// 
/// ボトルネックになりうる箇所: データベースからの大量データ取得
/// - ページネーションとフィルタリングを適用することで、データベースからの取得負荷を軽減しています
pub async fn get_paginated_tow_trucks_handler(
    service: web::Data<
        TowTruckService<TowTruckRepositoryImpl, OrderRepositoryImpl, MapRepositoryImpl>,
    >,
    query: web::Query<PaginatedTowTruckQuery>,
) -> Result<HttpResponse, AppError> {
    let tow_trucks = service
        .get_all_tow_trucks(
            query.page.unwrap_or(0),
            query.page_size.unwrap_or(-1),
            query.status.clone(),
            query.area,
        )
        .await?;

    Ok(HttpResponse::Ok().json(tow_trucks))
}

/// レッカー車IDに基づいてレッカー車情報を取得するハンドラー関数
/// 
/// `service` - レッカー車サービスのインスタンス
/// `path` - レッカー車IDのパスパラメータ
/// 
/// 成功した場合、HTTP 200 OK レスポンスとレッカー車情報を返す
/// レッカー車が見つからない場合、HTTP 404 Not Found を返す
/// 失敗した場合、AppError を返す
pub async fn get_tow_truck_handler(
    service: web::Data<
        TowTruckService<TowTruckRepositoryImpl, OrderRepositoryImpl, MapRepositoryImpl>,
    >,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    match service.get_tow_truck_by_id(id).await {
        Ok(Some(tow_truck)) => Ok(HttpResponse::Ok().json(tow_truck)),
        Ok(None) => Ok(HttpResponse::NotFound().finish()),
        Err(err) => Err(err),
    }
}

/// レッカー車の位置を更新するハンドラー関数
/// 
/// `service` - レッカー車サービスのインスタンス
/// `req` - 位置更新リクエストのデータ
/// 
/// 成功した場合、HTTP 200 OK レスポンスを返す
/// 失敗した場合、AppError を返す
pub async fn update_location_handler(
    service: web::Data<
        TowTruckService<TowTruckRepositoryImpl, OrderRepositoryImpl, MapRepositoryImpl>,
    >,
    req: web::Json<UpdateLocationRequestDto>,
) -> Result<HttpResponse, AppError> {
    service
        .update_location(req.tow_truck_id, req.node_id)
        .await?;
    Ok(HttpResponse::Ok().finish())
}

/// 注文IDに基づいて最寄りの利用可能なレッカー車を取得するためのクエリパラメータ
#[derive(Deserialize, Debug)]
pub struct TowTruckQuery {
    order_id: i32,
}

/// 最寄りの利用可能なレッカー車を取得するハンドラー関数
/// 
/// `service` - レッカー車サービスのインスタンス
/// `query` - 注文IDを含むクエリパラメータ
/// 
/// 成功した場合、HTTP 200 OK レスポンスと最寄りのレッカー車情報を返す
/// レッカー車が見つからない場合、HTTP 404 Not Found を返す
/// 失敗した場合、AppError を返す
pub async fn get_nearest_available_tow_trucks_handler(
    service: web::Data<
        TowTruckService<TowTruckRepositoryImpl, OrderRepositoryImpl, MapRepositoryImpl>,
    >,
    query: web::Query<TowTruckQuery>,
) -> Result<HttpResponse, AppError> {
    match service
        .get_nearest_available_tow_trucks(query.order_id)
        .await
    {
        Ok(Some(tow_truck)) => Ok(HttpResponse::Ok().json(tow_truck)),
        Ok(None) => Ok(HttpResponse::NotFound().finish()),
        Err(err) => Err(err),
    }
}