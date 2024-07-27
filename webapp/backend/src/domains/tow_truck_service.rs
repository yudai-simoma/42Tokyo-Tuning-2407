use super::dto::tow_truck::TowTruckDto;
use super::map_service::MapRepository;
use super::order_service::OrderRepository;
use crate::errors::AppError;
use crate::models::graph::Graph;
use crate::models::tow_truck::TowTruck;

/// レッカー車リポジトリのトレイト
pub trait TowTruckRepository {
    /// ページネーションされたレッカー車リストを取得する
    async fn get_paginated_tow_trucks(
        &self,
        page: i32,
        page_size: i32,
        status: Option<String>,
        area_id: Option<i32>,
    ) -> Result<Vec<TowTruck>, AppError>;
    
    /// レッカー車の位置を更新する
    async fn update_location(&self, truck_id: i32, node_id: i32) -> Result<(), AppError>;
    
    /// レッカー車のステータスを更新する
    async fn update_status(&self, truck_id: i32, status: &str) -> Result<(), AppError>;
    
    /// IDに基づいてレッカー車を検索する
    async fn find_tow_truck_by_id(&self, id: i32) -> Result<Option<TowTruck>, AppError>;
}

/// レッカー車サービスの構造体
#[derive(Debug)]
pub struct TowTruckService<
    T: TowTruckRepository + std::fmt::Debug,
    U: OrderRepository + std::fmt::Debug,
    V: MapRepository + std::fmt::Debug,
> {
    tow_truck_repository: T,
    order_repository: U,
    map_repository: V,
}

impl<
        T: TowTruckRepository + std::fmt::Debug,
        U: OrderRepository + std::fmt::Debug,
        V: MapRepository + std::fmt::Debug,
    > TowTruckService<T, U, V>
{
    /// 新しいレッカー車サービスを作成する
    pub fn new(tow_truck_repository: T, order_repository: U, map_repository: V) -> Self {
        TowTruckService {
            tow_truck_repository,
            order_repository,
            map_repository,
        }
    }

    /// IDに基づいてレッカー車を取得する
    pub async fn get_tow_truck_by_id(&self, id: i32) -> Result<Option<TowTruckDto>, AppError> {
        let tow_truck = self.tow_truck_repository.find_tow_truck_by_id(id).await?;
        Ok(tow_truck.map(TowTruckDto::from_entity))
    }

    /// ページネーションされたレッカー車リストを取得する
    pub async fn get_all_tow_trucks(
        &self,
        page: i32,
        page_size: i32,
        status: Option<String>,
        area: Option<i32>,
    ) -> Result<Vec<TowTruckDto>, AppError> {
        let tow_trucks = self
            .tow_truck_repository
            .get_paginated_tow_trucks(page, page_size, status, area)
            .await?;
        let tow_truck_dtos = tow_trucks
            .into_iter()
            .map(TowTruckDto::from_entity)
            .collect();

        Ok(tow_truck_dtos)
    }

    /// レッカー車の位置を更新する
    pub async fn update_location(&self, truck_id: i32, node_id: i32) -> Result<(), AppError> {
        self.tow_truck_repository
            .update_location(truck_id, node_id)
            .await?;

        Ok(())
    }

    /// 最寄りの利用可能なレッカー車を取得する
    /// 
    /// ボトルネックになりうる箇所: グラフ計算とソート処理
    /// - グラフの構築と最短経路計算は計算コストが高い可能性があります
    /// - レッカー車のソートも、レッカー車の数が多い場合は処理時間がかかる可能性があります
    pub async fn get_nearest_available_tow_trucks(
        &self,
        order_id: i32,
    ) -> Result<Option<TowTruckDto>, AppError> {
        let order = self.order_repository.find_order_by_id(order_id).await?;
        let area_id = self
            .map_repository
            .get_area_id_by_node_id(order.node_id)
            .await?;
        let tow_trucks = self
            .tow_truck_repository
            .get_paginated_tow_trucks(0, -1, Some("available".to_string()), Some(area_id))
            .await?;

        let nodes = self.map_repository.get_all_nodes(Some(area_id)).await?;
        let edges = self.map_repository.get_all_edges(Some(area_id)).await?;

        let mut graph = Graph::new();
        for node in nodes {
            graph.add_node(node);
        }
        for edge in edges {
            graph.add_edge(edge);
        }

        let sorted_tow_trucks_by_distance = {
            let mut tow_trucks_with_distance: Vec<_> = tow_trucks
                .into_iter()
                .map(|truck| {
                    let distance = calculate_distance(&graph, truck.node_id, order.node_id);
                    (distance, truck)
                })
                .collect();

            tow_trucks_with_distance.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            tow_trucks_with_distance
        };

        if sorted_tow_trucks_by_distance.is_empty() || sorted_tow_trucks_by_distance[0].0 > 10000000
        {
            return Ok(None);
        }

        let sorted_tow_truck_dtos: Vec<TowTruckDto> = sorted_tow_trucks_by_distance
            .into_iter()
            .map(|(_, truck)| TowTruckDto::from_entity(truck))
            .collect();

        Ok(sorted_tow_truck_dtos.first().cloned())
    }
}

/// グラフ上の2つのノード間の最短距離を計算する
fn calculate_distance(graph: &Graph, node_id_1: i32, node_id_2: i32) -> i32 {
    graph.shortest_path(node_id_1, node_id_2)
}