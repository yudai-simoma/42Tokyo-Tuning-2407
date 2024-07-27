import Axios from "../axios";
import Fetch from "../fetch";
import queryString from "query-string";

// 注文情報の型定義
export type Order = {
  id: number;
  status: string;
  node_id: number;
  area_id: number;
  tow_truck_id: number;
  car_value: number;
  client_id: number;
  client_username: string;
  dispatcher_user_id: number;
  dispatcher_username: string;
  driver_user_id: number;
  driver_username: string;
  order_time: string;
  completed_time: string;
};

// 注文クエリパラメータの型定義
export type OrdersQueryParams = {
  status: string;
  sort_by: string;
  sort_order: string;
};

// AxiosとFetchのインスタンスを取得
const AxiosInstance = Axios.getInstance();
const FetchInstance = Fetch.getInstance();

/**
 * 注文リストを取得する関数
 * @param query_params - クエリパラメータ（ステータス、ソート条件など）
 * @param area - エリアID（nullの場合もあり）
 * @param session_token - セッショントークン（認証用）
 * @returns 注文のリスト
 */
export const fetchOrders = async (query_params: OrdersQueryParams, area: number | null, session_token: string) => {
  // クエリパラメータを文字列に変換
  const queryParams = queryString.stringify({
    ...query_params,
    status: "pending",  // デフォルトでステータスを "pending" に設定
    sort_by: "order_time",  // デフォルトで注文時間でソート
    sort_order: "asc",  // 昇順でソート
    area
  });

  // Fetchを使用して注文リストを取得
  const orders = await FetchInstance.fetch<Order[]>(`/api/order/list?${queryParams}`, {
    headers: { Authorization: session_token }  // セッショントークンをヘッダーに追加
  });

  return orders;
};

/**
 * 特定の注文を取得する関数
 * @param order_id - 注文ID
 * @param session_token - セッショントークン（認証用）
 * @returns 注文の詳細情報
 */
export const fetchOrder = async (order_id: string, session_token: string) => {
  // Fetchを使用して特定の注文を取得
  const order = await FetchInstance.fetch<Order>(`/api/order/${order_id}`, {
    headers: { Authorization: session_token }  // セッショントークンをヘッダーに追加
  });

  return order;
};

/**
 * レッカー車を手配する関数
 * @param dispatcher_id - ディスパッチャーのユーザーID
 * @param order_id - 注文ID
 * @param tow_truck_id - レッカー車ID
 * @param order_time - 注文時間
 * @param session_token - セッショントークン（認証用）
 */
export const arrangeTowTruck = async (
  dispatcher_id: number,
  order_id: number,
  tow_truck_id: number,
  order_time: string,
  session_token: string
) => {
  // Axiosを使用してレッカー車の手配をリクエスト
  await AxiosInstance.post(
    "/api/order/dispatcher",
    {
      dispatcher_id,
      order_id,
      tow_truck_id,
      order_time
    },
    { timeout: 5000, headers: { Authorization: session_token } }  // タイムアウトとセッショントークンを設定
  );
};