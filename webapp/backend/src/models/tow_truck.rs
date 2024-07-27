use sqlx::FromRow;

/// レッカー車を表す構造体
#[derive(FromRow, Clone, Debug)]
pub struct TowTruck {
    pub id: i32,
    pub driver_id: i32,
    pub driver_username: Option<String>,
    pub status: String,
    pub area_id: i32,
    pub node_id: i32,
}