import Axios from "../axios";

// ユーザーの役割を定義する型
export type Role = "client" | "dispatcher" | "driver" | "admin";

// ユーザー情報を表す型定義
export type User = {
  user_id: number;
  user_name: string;
  session_token: string;
} & (
  | {
      role: "dispatcher";
      dispatcher_id: number;
      area_id: number;
    }
  | {
      role: "client";
    }
  | {
      role: "driver";
      driver_id: number;
    }
  | {
      role: "admin";
    }
);

// Axiosのインスタンスを取得
const AxiosInstance = Axios.getInstance();

/**
 * ログイン処理を行う関数
 * @param username - ユーザー名
 * @param password - パスワード
 * @returns ログインしたユーザーの情報
 */
export const login = async (username: string, password: string) => {
  // ログインAPIを呼び出し、ユーザー情報を取得
  const { data } = await AxiosInstance.post<User>("/api/login", {
    username,
    password
  });

  // セッション情報をサーバーサイドに保存
  await AxiosInstance.post("/session", data);

  return data;
};

/**
 * ログアウト処理を行う関数
 * @param session_token - セッショントークン（nullの場合もあり）
 */
export const logout = async (session_token: string | null) => {
  if (session_token) {
    // ログアウトAPIを呼び出し
    await AxiosInstance.post("/api/logout", { session_token }, { headers: { Authorization: session_token } });
  }
  // サーバーサイドのセッション情報を削除
  await AxiosInstance.delete("/session");
};

/**
 * 現在のセッション情報を取得する関数
 * @returns ユーザー情報（セッションが存在しない場合はundefined）
 */
export const getSession = async () => {
  try {
    // サーバーサイドからセッション情報を取得
    const response = await AxiosInstance.get<User>("/session");
    return response.data;
  } catch (error: any) {
    // エラーが発生した場合はコンソールにログを出力
    console.error("An error occurred while fetching the session:", error);
    return undefined;
  }
};