use sqlx::FromRow;
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;

#[derive(FromRow, Clone, Debug)]
pub struct Node {
    pub id: i32,
    pub x: i32,
    pub y: i32,
}

#[derive(FromRow, Clone, Debug)]
pub struct Edge {
    pub node_a_id: i32,
    pub node_b_id: i32,
    pub weight: i32,
}

#[derive(Debug)]
pub struct Graph {
    pub nodes: HashMap<i32, Node>,
    pub edges: HashMap<i32, Vec<Edge>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id, node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges
            .entry(edge.node_a_id)
            .or_default()
            .push(edge.clone());

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

    pub fn shortest_path(&self, from_node_id: i32, to_node_id: i32) -> i32 {
        let mut distances = HashMap::new();
        let mut heap = BinaryHeap::new();

        #[derive(Copy, Clone, Eq, PartialEq)]
        struct State {
            cost: i32,
            position: i32,
        }

        // 優先度キューは`Ord`に依存します。トレイトを明示的に実装して、キューを最小ヒープにします
        impl Ord for State {
            fn cmp(&self, other: &Self) -> Ordering {
                // コストの順序を反転させることに注意してください。
                // 同点の場合は位置を比較します - この手順は
                // `PartialEq`と`Ord`の実装を一貫させるために必要です。
                other.cost.cmp(&self.cost)
                    .then_with(|| self.position.cmp(&other.position))
            }
        }

        // `PartialOrd`も実装する必要があります。
        impl PartialOrd for State {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        // 開始ノードの距離を初期化
        distances.insert(from_node_id, 0);
        heap.push(State { cost: 0, position: from_node_id });

        while let Some(State { cost, position }) = heap.pop() {
            // 目的地ノードに到達した場合、コストを返す
            if position == to_node_id {
                return cost;
            }

            // コストが記録されたコストより大きい場合、このノードをスキップ
            if cost > *distances.get(&position).unwrap_or(&i32::MAX) {
                continue;
            }

            // 隣接ノードを探索
            if let Some(edges) = self.edges.get(&position) {
                for edge in edges {
                    let next = State { cost: cost + edge.weight, position: edge.node_b_id };

                    // 隣接ノードへの短い経路が見つかった場合、距離を更新してヒープにプッシュ
                    if next.cost < *distances.get(&next.position).unwrap_or(&i32::MAX) {
                        heap.push(next);
                        distances.insert(next.position, next.cost);
                    }
                }
            }
        }

        // 目的地ノードに到達できない場合、i32::MAXを返す
        distances.get(&to_node_id).cloned().unwrap_or(i32::MAX)
    }
}

/// グラフ上の2つのノード間の最短距離を計算する
fn calculate_distance(graph: &Graph, node_id_1: i32, node_id_2: i32) -> i32 {
    graph.shortest_path(node_id_1, node_id_2)
}
