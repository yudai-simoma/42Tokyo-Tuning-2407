use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::order::CompletedOrder;

// 入力データ構造

/// クライアント注文リクエストのデータ構造
#[derive(Deserialize, Debug)]
pub struct ClientOrderRequestDto {
    pub client_id: i32,
    pub node_id: i32,
    pub car_value: f64,
}

/// ディスパッチャー注文リクエストのデータ構造
#[derive(Deserialize, Debug)]
pub struct DispatcherOrderRequestDto {
    pub order_id: i32,
    pub dispatcher_id: i32,
    pub tow_truck_id: i32,
    pub order_time: DateTime<Utc>,
}

/// 注文ステータス更新リクエストのデータ構造
#[derive(Deserialize, Debug)]
pub struct UpdateOrderStatusRequestDto {
    pub order_id: i32,
    pub status: String,
}

// 出力データ構造

/// 注文のデータ構造
#[derive(Serialize, Debug)]
pub struct OrderDto {
    pub id: i32,
    pub client_id: i32,
    pub client_username: Option<String>,
    pub dispatcher_id: Option<i32>,
    pub dispatcher_user_id: Option<i32>,
    pub dispatcher_username: Option<String>,
    pub tow_truck_id: Option<i32>,
    pub driver_user_id: Option<i32>,
    pub driver_username: Option<String>,
    pub status: String,
    pub node_id: i32,
    pub area_id: i32,
    pub car_value: f64,
    pub order_time: DateTime<Utc>,
    pub completed_time: Option<DateTime<Utc>>,
}

/// 完了した注文のデータ構造
#[derive(Serialize, Debug)]
pub struct CompletedOrderDto {
    pub id: i32,
    pub order_id: i32,
    pub tow_truck_id: i32,
    pub order_time: Option<DateTime<Utc>>,
    pub completed_time: DateTime<Utc>,
    pub car_value: f64,
}

impl CompletedOrderDto {
    /// CompletedOrder エンティティから CompletedOrderDto を生成する関数
    pub fn from_entity(entity: CompletedOrder) -> Self {
        CompletedOrderDto {
            id: entity.id,
            order_id: entity.order_id,
            tow_truck_id: entity.tow_truck_id,
            car_value: entity.car_value,
            order_time: entity.order_time,
            completed_time: entity.completed_time,
        }
    }
}