use serde::{Deserialize, Serialize};

// 入力データ構造

/// 位置更新リクエストのデータ構造
#[derive(Deserialize, Debug)]
pub struct UpdateLocationRequestDto {
    pub tow_truck_id: i32,
    pub node_id: i32,
}

// 出力データ構造

/// レッカー車のデータ構造
#[derive(Serialize, Clone)]
pub struct TowTruckDto {
    pub id: i32,
    pub driver_user_id: i32,
    pub driver_username: Option<String>,
    pub status: String,
    pub node_id: i32,
    pub area_id: i32,
}

impl TowTruckDto {
    /// TowTruck エンティティから TowTruckDto を生成する関数
    pub fn from_entity(entity: crate::models::tow_truck::TowTruck) -> Self {
        TowTruckDto {
            id: entity.id,
            driver_user_id: entity.driver_id,
            driver_username: entity.driver_username,
            status: entity.status,
            node_id: entity.node_id,
            area_id: entity.area_id,
        }
    }
}