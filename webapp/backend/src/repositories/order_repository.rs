use crate::domains::order_service::OrderRepository;
use crate::errors::AppError;
use crate::models::order::{CompletedOrder, Order};
use chrono::{DateTime, Utc};
use sqlx::mysql::MySqlPool;

/// 注文リポジトリの実装構造体
#[derive(Debug)]
pub struct OrderRepositoryImpl {
    pool: MySqlPool,
}

impl OrderRepositoryImpl {
    /// 新しい `OrderRepositoryImpl` を作成する
    ///
    /// `pool` - MySQL の接続プール
    pub fn new(pool: MySqlPool) -> Self {
        OrderRepositoryImpl { pool }
    }
}

impl OrderRepository for OrderRepositoryImpl {
    /// 注文IDで注文を検索する
    ///
    /// `id` - 注文ID
    ///
    /// 成功した場合は `Order` を返し、失敗した場合は `AppError` を返す
    async fn find_order_by_id(&self, id: i32) -> Result<Order, AppError> {
        let order = sqlx::query_as::<_, Order>(
            "SELECT 
                *
            FROM
                orders 
            WHERE
                id = ?",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(order)
    }

    /// 注文のステータスを更新する
    ///
    /// `order_id` - 注文ID
    /// `status` - 新しいステータス
    ///
    /// 成功した場合は `()` を返し、失敗した場合は `AppError` を返す
    async fn update_order_status(&self, order_id: i32, status: &str) -> Result<(), AppError> {
        sqlx::query("UPDATE orders SET status = ? WHERE id = ?")
            .bind(status)
            .bind(order_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// ページネーションされた注文リストを取得する
    ///
    /// `page` - ページ番号
    /// `page_size` - 1ページあたりの注文数
    /// `sort_by` - ソートするフィールド
    /// `sort_order` - ソート順序（ASC または DESC）
    /// `status` - 注文のステータス
    /// `area` - エリアID
    ///
    /// 成功した場合は `Vec<Order>` を返し、失敗した場合は `AppError` を返す
    async fn get_paginated_orders(
        &self,
        page: i32,
        page_size: i32,
        sort_by: Option<String>,
        sort_order: Option<String>,
        status: Option<String>,
        area: Option<i32>,
    ) -> Result<Vec<Order>, AppError> {
        let offset = page * page_size;
        let order_clause = format!(
            "ORDER BY {} {}",
            match sort_by.as_deref() {
                Some("car_value") => "o.car_value",
                Some("status") => "o.status",
                Some("order_time") => "o.order_time",
                _ => "o.order_time",
            },
            match sort_order.as_deref() {
                Some("DESC") | Some("desc") => "DESC",
                _ => "ASC",
            }
        );

        let where_clause = match (status.clone(), area) {
            (Some(_), Some(_)) => "WHERE o.status = ? AND n.area_id = ?".to_string(),
            (None, Some(_)) => "WHERE n.area_id = ?".to_string(),
            (Some(_), None) => "WHERE o.status = ?".to_string(),
            _ => "".to_string(),
        };

        let sql = format!(
            "SELECT 
                o.id, 
                o.client_id, 
                o.dispatcher_id, 
                o.tow_truck_id, 
                o.status, 
                o.node_id, 
                o.car_value, 
                o.order_time, 
                o.completed_time
            FROM
                orders o
            JOIN
                nodes n
            ON 
                o.node_id = n.id
            {} 
            {} 
            LIMIT ? 
            OFFSET ?",
            where_clause, order_clause
        );

        let orders = match (status, area) {
            (Some(status), Some(area)) => {
                sqlx::query_as::<_, Order>(&sql)
                    .bind(status)
                    .bind(area)
                    .bind(page_size)
                    .bind(offset)
                    .fetch_all(&self.pool)
                    .await?
            }
            (None, Some(area)) => {
                sqlx::query_as::<_, Order>(&sql)
                    .bind(area)
                    .bind(page_size)
                    .bind(offset)
                    .fetch_all(&self.pool)
                    .await?
            }
            (Some(status), None) => {
                sqlx::query_as::<_, Order>(&sql)
                    .bind(status)
                    .bind(page_size)
                    .bind(offset)
                    .fetch_all(&self.pool)
                    .await?
            }
            _ => {
                sqlx::query_as::<_, Order>(&sql)
                    .bind(page_size)
                    .bind(offset)
                    .fetch_all(&self.pool)
                    .await?
            }
        };

        Ok(orders)
    }

    /// 新しい注文を作成する
    ///
    /// `client_id` - クライアントID
    /// `node_id` - ノードID
    /// `car_value` - 車の価値
    ///
    /// 成功した場合は `()` を返し、失敗した場合は `AppError` を返す
    async fn create_order(
        &self,
        client_id: i32,
        node_id: i32,
        car_value: f64,
    ) -> Result<(), AppError> {
        sqlx::query("INSERT INTO orders (client_id, node_id, status, car_value) VALUES (?, ?, 'pending', ?)")
            .bind(client_id)
            .bind(node_id)
            .bind(car_value)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// 注文のディスパッチ情報を更新する
    ///
    /// `id` - 注文ID
    /// `dispatcher_id` - ディスパッチャーID
    /// `tow_truck_id` - レッカー車ID
    ///
    /// 成功した場合は `()` を返し、失敗した場合は `AppError` を返す
    async fn update_order_dispatched(
        &self,
        id: i32,
        dispatcher_id: i32,
        tow_truck_id: i32,
    ) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE orders SET dispatcher_id = ?, tow_truck_id = ?, status = 'dispatched' WHERE id = ?",
        )
        .bind(dispatcher_id)
        .bind(tow_truck_id)
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// 完了した注文を作成する
    ///
    /// `order_id` - 注文ID
    /// `tow_truck_id` - レッカー車ID
    /// `completed_time` - 完了時間
    ///
    /// 成功した場合は `()` を返し、失敗した場合は `AppError` を返す
    async fn create_completed_order(
        &self,
        order_id: i32,
        tow_truck_id: i32,
        completed_time: DateTime<Utc>,
    ) -> Result<(), AppError> {
        sqlx::query("INSERT INTO completed_orders (order_id, tow_truck_id, completed_time) VALUES (?, ?, ?)")
            .bind(order_id)
            .bind(tow_truck_id)
            .bind(completed_time)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// 全ての完了した注文を取得する
    ///
    /// 成功した場合は `Vec<CompletedOrder>` を返し、失敗した場合は `AppError` を返す
    async fn get_all_completed_orders(&self) -> Result<Vec<CompletedOrder>, AppError> {
        let orders = sqlx::query_as::<_, CompletedOrder>(
            "SELECT co.id, co.order_id, co.tow_truck_id, co.order_time, co.completed_time, o.car_value
                    FROM completed_orders co
                    JOIN orders o ON co.order_id = o.id"
            )
            .fetch_all(&self.pool)
            .await?;

        Ok(orders)
    }
}