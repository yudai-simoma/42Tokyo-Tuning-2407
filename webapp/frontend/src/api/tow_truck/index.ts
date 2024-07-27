import Axios from "../axios";

// レッカー車の情報を表す型定義
export type TowTruck = {
  id: number;
  status: string;
  node_id: number;
};

// Axiosのインスタンスを取得
const AxiosInstance = Axios.getInstance();

/**
 * 最寄りのレッカー車を取得する関数
 * @param order_id - 注文ID
 * @param session_token - セッショントークン（認証用）
 * @returns 最寄りのレッカー車の情報
 */
export const fetchNearestTowTruck = async (order_id: number, session_token: string) => {
  // Axiosを使用して最寄りのレッカー車を取得
  const res = await AxiosInstance.get<TowTruck>("/api/tow_truck/nearest", {
    params: {
      order_id  // クエリパラメータとして注文IDを渡す
    },
    headers: {
      Authorization: session_token  // セッショントークンをヘッダーに追加
    }
  });
  // レスポンスからデータを返す
  return res.data;
};