use sqlx::FromRow;
use std::collections::HashMap;

/// ノードを表す構造体
#[derive(FromRow, Clone, Debug)]
pub struct Node {
    pub id: i32,
    pub x: i32,
    pub y: i32,
}

/// エッジを表す構造体
#[derive(FromRow, Clone, Debug)]
pub struct Edge {
    pub node_a_id: i32,
    pub node_b_id: i32,
    pub weight: i32,
}

/// グラフを表す構造体
#[derive(Debug)]
pub struct Graph {
    pub nodes: HashMap<i32, Node>,
    pub edges: HashMap<i32, Vec<Edge>>,
}

impl Graph {
    /// 新しいグラフを作成する
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    /// グラフにノードを追加する
    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id, node);
    }

    /// グラフにエッジを追加する
    pub fn add_edge(&mut self, edge: Edge) {
        self.edges
            .entry(edge.node_a_id)
            .or_default()
            .push(edge.clone());

        // 逆方向のエッジも追加
        let reverse_edge = Edge {
            node_a_id: edge.node_b_id,
            node_b_id: edge.node_a_id,
            weight: edge.weight,
        };
        self.edges
            .entry(reverse_edge.node_a_id)
            .or_default()
            .push(reverse_edge);
    }

    /// 2つのノード間の最短経路を計算する
    /// 
    /// ボトルネックになりうる箇所: 最短経路の計算
    /// - ノード数が多い場合、計算量が O(V^2) となり、処理時間が長くなる可能性があります
    /// - 大規模なグラフでは、より効率的なアルゴリズム（例：ダイクストラ法）の使用を検討する必要があります
    pub fn shortest_path(&self, from_node_id: i32, to_node_id: i32) -> i32 {
        let mut distances = HashMap::new();
        distances.insert(from_node_id, 0);

        for _ in 0..self.nodes.len() {
            for node_id in self.nodes.keys() {
                if let Some(edges) = self.edges.get(node_id) {
                    for edge in edges {
                        let new_distance = distances
                            .get(node_id)
                            .and_then(|d: &i32| d.checked_add(edge.weight))
                            .unwrap_or(i32::MAX);
                        let current_distance = distances.get(&edge.node_b_id).unwrap_or(&i32::MAX);
                        if new_distance < *current_distance {
                            distances.insert(edge.node_b_id, new_distance);
                        }
                    }
                }
            }
        }

        distances.get(&to_node_id).cloned().unwrap_or(i32::MAX)
    }
}