use crate::{
    errors::AppError,
    models::graph::{Edge, Node},
};

/// マップリポジトリのトレイト
pub trait MapRepository {
    /// 全てのノードを取得する
    /// 
    /// `area_id` - オプションのエリアID。指定された場合、そのエリアのノードのみを取得する
    /// 
    /// 戻り値: ノードのベクターまたはSQLエラー
    async fn get_all_nodes(&self, area_id: Option<i32>) -> Result<Vec<Node>, sqlx::Error>;

    /// 全てのエッジを取得する
    /// 
    /// `area_id` - オプションのエリアID。指定された場合、そのエリアのエッジのみを取得する
    /// 
    /// 戻り値: エッジのベクターまたはSQLエラー
    async fn get_all_edges(&self, area_id: Option<i32>) -> Result<Vec<Edge>, sqlx::Error>;

    /// ノードIDに基づいてエリアIDを取得する
    /// 
    /// `node_id` - ノードID
    /// 
    /// 戻り値: エリアIDまたはSQLエラー
    async fn get_area_id_by_node_id(&self, node_id: i32) -> Result<i32, sqlx::Error>;

    /// エッジを更新する
    /// 
    /// `node_a_id` - ノードAのID
    /// `node_b_id` - ノードBのID
    /// `weight` - 新しい重み
    /// 
    /// 戻り値: 成功した場合は空のResult、失敗した場合はSQLエラー
    async fn update_edge(
        &self,
        node_a_id: i32,
        node_b_id: i32,
        weight: i32,
    ) -> Result<(), sqlx::Error>;
}

/// マップサービスの構造体
#[derive(Debug)]
pub struct MapService<T: MapRepository + std::fmt::Debug> {
    repository: T,
}

impl<T: MapRepository + std::fmt::Debug> MapService<T> {
    /// 新しいマップサービスを作成する
    pub fn new(repository: T) -> Self {
        MapService { repository }
    }

    /// エッジを更新する
    /// 
    /// `node_a_id` - ノードAのID
    /// `node_b_id` - ノードBのID
    /// `weight` - 新しい重み
    /// 
    /// 戻り値: 成功した場合は空のResult、失敗した場合はAppError
    pub async fn update_edge(
        &self,
        node_a_id: i32,
        node_b_id: i32,
        weight: i32,
    ) -> Result<(), AppError> {
        self.repository
            .update_edge(node_a_id, node_b_id, weight)
            .await?;

        Ok(())
    }
}