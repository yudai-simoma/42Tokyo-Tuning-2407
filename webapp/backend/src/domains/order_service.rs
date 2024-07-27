use chrono::{DateTime, Utc};

use super::{
    auth_service::AuthRepository,
    dto::order::{CompletedOrderDto, OrderDto},
    map_service::MapRepository,
    tow_truck_service::TowTruckRepository,
};

use crate::{
    errors::AppError,
    models::order::{CompletedOrder, Order},
};

/// 注文リポジトリのトレイト
pub trait OrderRepository {
    /// 注文IDに基づいて注文を取得する
    async fn find_order_by_id(&self, id: i32) -> Result<Order, AppError>;

    /// 注文のステータスを更新する
    async fn update_order_status(&self, order_id: i32, status: &str) -> Result<(), AppError>;

    /// ページネーションされた注文リストを取得する
    async fn get_paginated_orders(
        &self,
        page: i32,
        page_size: i32,
        sort_by: Option<String>,
        sort_order: Option<String>,
        status: Option<String>,
        area: Option<i32>,
    ) -> Result<Vec<Order>, AppError>;

    /// 新しい注文を作成する
    async fn create_order(
        &self,
        customer_id: i32,
        node_id: i32,
        car_value: f64,
    ) -> Result<(), AppError>;

    /// 注文のディスパッチ情報を更新する
    async fn update_order_dispatched(
        &self,
        id: i32,
        dispatcher_id: i32,
        tow_truck_id: i32,
    ) -> Result<(), AppError>;

    /// 完了した注文を作成する
    async fn create_completed_order(
        &self,
        order_id: i32,
        tow_truck_id: i32,
        completed_time: DateTime<Utc>,
    ) -> Result<(), AppError>;

    /// 全ての完了した注文を取得する
    async fn get_all_completed_orders(&self) -> Result<Vec<CompletedOrder>, AppError>;
}

/// 注文サービスの構造体
#[derive(Debug)]
pub struct OrderService<
    T: OrderRepository + std::fmt::Debug,
    U: TowTruckRepository + std::fmt::Debug,
    V: AuthRepository + std::fmt::Debug,
    W: MapRepository + std::fmt::Debug,
> {
    order_repository: T,
    tow_truck_repository: U,
    auth_repository: V,
    map_repository: W,
}

impl<
    T: OrderRepository + std::fmt::Debug,
    U: TowTruckRepository + std::fmt::Debug,
    V: AuthRepository + std::fmt::Debug,
    W: MapRepository + std::fmt::Debug,
> OrderService<T, U, V, W>
{
    /// 新しい注文サービスを作成する
    pub fn new(
        order_repository: T,
        tow_truck_repository: U,
        auth_repository: V,
        map_repository: W,
    ) -> Self {
        OrderService {
            order_repository,
            tow_truck_repository,
            auth_repository,
            map_repository,
        }
    }

    /// 注文のステータスを更新する
    pub async fn update_order_status(&self, order_id: i32, status: &str) -> Result<(), AppError> {
        self.order_repository
            .update_order_status(order_id, status)
            .await?;
        Ok(())
    }

    /// 注文IDに基づいて注文情報を取得する
    pub async fn get_order_by_id(&self, id: i32) -> Result<OrderDto, AppError> {
        let order = self.order_repository.find_order_by_id(id).await?;

        // クライアントのユーザー名を取得
        let client_username = self
            .auth_repository
            .find_user_by_id(order.client_id)
            .await?
            .ok_or(AppError::NotFound)?
            .username;

        // ディスパッチャー情報を取得
        let dispatcher = match order.dispatcher_id {
            Some(dispatcher_id) => self
                .auth_repository
                .find_dispatcher_by_id(dispatcher_id)
                .await?,
            None => None,
        };
        let (dispatcher_user_id, dispatcher_username) = match dispatcher {
            Some(dispatcher) => (
                Some(dispatcher.user_id),
                Some(
                    self.auth_repository
                        .find_user_by_id(dispatcher.user_id)
                        .await?
                        .ok_or(AppError::NotFound)?
                        .username,
                ),
            ),
            None => (None, None),
        };

        // レッカー車情報を取得
        let tow_truck = match order.tow_truck_id {
            Some(tow_truck_id) => self
                .tow_truck_repository
                .find_tow_truck_by_id(tow_truck_id)
                .await?,
            None => None,
        };
        let (driver_user_id, driver_username) = match tow_truck {
            Some(tow_truck) => (
                Some(tow_truck.driver_id),
                Some(
                    self.auth_repository
                        .find_user_by_id(tow_truck.driver_id)
                        .await?
                        .ok_or(AppError::NotFound)?
                        .username,
                ),
            ),
            None => (None, None),
        };

        // ノードIDに基づいてエリアIDを取得
        let area_id = self
            .map_repository
            .get_area_id_by_node_id(order.node_id)
            .await?;

        Ok(OrderDto {
            id: order.id,
            client_id: order.client_id,
            client_username: Some(client_username),
            dispatcher_user_id,
            dispatcher_username,
            driver_user_id,
            driver_username,
            area_id,
            dispatcher_id: order.dispatcher_id,
            tow_truck_id: order.tow_truck_id,
            status: order.status,
            node_id: order.node_id,
            car_value: order.car_value,
            order_time: order.order_time,
            completed_time: order.completed_time,
        })
    }

    /// ページネーションされた注文リストを取得する
    /// 
    /// ボトルネックになりうる箇所: データベースからの大量データ取得
    /// - ページネーションとフィルタリングを適用することで、データベースからの取得負荷を軽減しています
    pub async fn get_paginated_orders(
        &self,
        page: i32,
        page_size: i32,
        sort_by: Option<String>,
        sort_order: Option<String>,
        status: Option<String>,
        area: Option<i32>,
    ) -> Result<Vec<OrderDto>, AppError> {
        let orders = self
            .order_repository
            .get_paginated_orders(page, page_size, sort_by, sort_order, status, area)
            .await?;

        let mut results = Vec::new();

        for order in orders {
            // クライアントのユーザー名を取得
            let client_username = self
                .auth_repository
                .find_user_by_id(order.client_id)
                .await?
                .ok_or(AppError::NotFound)?
                .username;

            // ディスパッチャー情報を取得
            let dispatcher = match order.dispatcher_id {
                Some(dispatcher_id) => self
                    .auth_repository
                    .find_dispatcher_by_id(dispatcher_id)
                    .await?,
                None => None,
            };
            let (dispatcher_user_id, dispatcher_username) = match dispatcher {
                Some(dispatcher) => (
                    Some(dispatcher.user_id),
                    Some(
                        self.auth_repository
                            .find_user_by_id(dispatcher.user_id)
                            .await?
                            .ok_or(AppError::NotFound)?
                            .username,
                    ),
                ),
                None => (None, None),
            };

            // レッカー車情報を取得
            let tow_truck = match order.tow_truck_id {
                Some(tow_truck_id) => self
                    .tow_truck_repository
                    .find_tow_truck_by_id(tow_truck_id)
                    .await?,
                None => None,
            };
            let (driver_user_id, driver_username) = match tow_truck {
                Some(tow_truck) => (
                    Some(tow_truck.driver_id),
                    Some(
                        self.auth_repository
                            .find_user_by_id(tow_truck.driver_id)
                            .await?
                            .ok_or(AppError::NotFound)?
                            .username,
                    ),
                ),
                None => (None, None),
            };

            // ノードIDに基づいてエリアIDを取得
            let order_area_id = self
                .map_repository
                .get_area_id_by_node_id(order.node_id)
                .await?;

            results.push(OrderDto {
                id: order.id,
                client_id: order.client_id,
                client_username: Some(client_username),
                dispatcher_id: order.dispatcher_id,
                dispatcher_user_id,
                dispatcher_username,
                tow_truck_id: order.tow_truck_id,
                driver_user_id,
                driver_username,
                area_id: order_area_id,
                status: order.status,
                node_id: order.node_id,
                car_value: order.car_value,
                order_time: order.order_time,
                completed_time: order.completed_time,
            });
        }

        Ok(results)
    }

    /// クライアント注文を作成する
    pub async fn create_client_order(
        &self,
        client_id: i32,
        node_id: i32,
        car_value: f64,
    ) -> Result<(), AppError> {
        match self
            .order_repository
            .create_order(client_id, node_id, car_value)
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(AppError::BadRequest),
        }
    }

    /// ディスパッチャー注文を作成する
    pub async fn create_dispatcher_order(
        &self,
        order_id: i32,
        dispatcher_id: i32,
        tow_truck_id: i32,
        order_time: DateTime<Utc>,
    ) -> Result<(), AppError> {
        if (self
            .order_repository
            .create_completed_order(order_id, tow_truck_id, order_time)
            .await)
            .is_err()
        {
            return Err(AppError::BadRequest);
        }

        self.order_repository
            .update_order_dispatched(order_id, dispatcher_id, tow_truck_id)
            .await?;
        self.tow_truck_repository
            .update_status(tow_truck_id, "busy")
            .await?;
        Ok(())
    }

    /// 完了した注文を取得する
    pub async fn get_completed_orders(&self) -> Result<Vec<CompletedOrderDto>, AppError> {
        let orders = self.order_repository.get_all_completed_orders().await?;
        let order_dtos = orders
            .into_iter()
            .map(CompletedOrderDto::from_entity)
            .collect();
        Ok(order_dtos)
    }
}