use sqlx::MySqlPool;

use crate::{
    domains::map_service::MapRepository,
    models::graph::{Edge, Node},
};

/// マップリポジトリの実装構造体
#[derive(Debug)]
pub struct MapRepositoryImpl {
    pool: MySqlPool,
}

impl MapRepositoryImpl {
    /// 新しい `MapRepositoryImpl` を作成する
    ///
    /// `pool` - MySQL の接続プール
    pub fn new(pool: MySqlPool) -> Self {
        MapRepositoryImpl { pool }
    }
}

impl MapRepository for MapRepositoryImpl {
    /// 全てのノードを取得する
    ///
    /// `area_id` - オプションのエリアID。指定された場合、そのエリアのノードのみを取得する
    ///
    /// 成功した場合は `Vec<Node>` を返し、失敗した場合は `sqlx::Error` を返す
    async fn get_all_nodes(&self, area_id: Option<i32>) -> Result<Vec<Node>, sqlx::Error> {
        // エリアIDに基づいてWHERE句を作成
        let where_clause = match area_id {
            Some(_) => "WHERE area_id = ?",
            None => "",
        };

        // SQLクエリを作成
        let sql = format!(
            "SELECT
                * 
            FROM
                nodes
            {}
            ORDER BY
                id",
            where_clause
        );

        // SQLクエリを実行し、結果を取得
        let nodes = match area_id {
            Some(area_id) => {
                sqlx::query_as::<_, Node>(&sql)
                    .bind(area_id)
                    .fetch_all(&self.pool)
                    .await?
            }
            None => {
                sqlx::query_as::<_, Node>(&sql)
                    .fetch_all(&self.pool)
                    .await?
            }
        };

        Ok(nodes)
    }

    /// 全てのエッジを取得する
    ///
    /// `area_id` - オプションのエリアID。指定された場合、そのエリアのエッジのみを取得する
    ///
    /// 成功した場合は `Vec<Edge>` を返し、失敗した場合は `sqlx::Error` を返す
    async fn get_all_edges(&self, area_id: Option<i32>) -> Result<Vec<Edge>, sqlx::Error> {
        // エリアIDに基づいてWHERE句を作成
        let where_clause = match area_id {
            Some(_) => "JOIN nodes n ON e.node_a_id = n.id WHERE n.area_id = ?",
            None => "",
        };

        // SQLクエリを作成
        let sql = format!(
            "SELECT
                e.node_a_id,
                e.node_b_id,
                e.weight
            FROM
                edges e
            {}",
            where_clause
        );

        // SQLクエリを実行し、結果を取得
        let edges = match area_id {
            Some(area_id) => {
                sqlx::query_as::<_, Edge>(&sql)
                    .bind(area_id)
                    .fetch_all(&self.pool)
                    .await?
            }
            None => {
                sqlx::query_as::<_, Edge>(&sql)
                    .fetch_all(&self.pool)
                    .await?
            }
        };

        Ok(edges)
    }

    /// ノードIDに基づいてエリアIDを取得する
    ///
    /// `node_id` - ノードID
    ///
    /// 成功した場合は `i32` を返し、失敗した場合は `sqlx::Error` を返す
    async fn get_area_id_by_node_id(&self, node_id: i32) -> Result<i32, sqlx::Error> {
        let area_id = sqlx::query_scalar("SELECT area_id FROM nodes WHERE id = ?")
            .bind(node_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(area_id)
    }

    /// エッジを更新する
    ///
    /// `node_a_id` - ノードAのID
    /// `node_b_id` - ノードBのID
    /// `weight` - 新しい重み
    ///
    /// 成功した場合は `()` を返し、失敗した場合は `sqlx::Error` を返す
    async fn update_edge(
        &self,
        node_a_id: i32,
        node_b_id: i32,
        weight: i32,
    ) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("UPDATE edges SET weight = ? WHERE node_a_id = ? AND node_b_id = ?")
            .bind(weight)
            .bind(node_a_id)
            .bind(node_b_id)
            .execute(&mut tx)
            .await?;
    
        // sqlx::query("UPDATE edges SET weight = ? WHERE node_a_id = ? AND node_b_id = ?")
        //     .bind(weight)
        //     .bind(node_b_id)
        //     .bind(node_a_id)
        //     .execute(&mut tx)
        //     .await?;
    
        tx.commit().await?;
    
        Ok(())
    }
}