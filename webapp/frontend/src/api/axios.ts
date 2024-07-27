import axios, { AxiosInstance } from "axios";

/**
 * Axiosインスタンスを管理するSingletonクラス
 */
class Axios {
  // Axiosインスタンスを保持する静的プロパティ
  private static instance: AxiosInstance;

  // 外部からのインスタンス化を防ぐためのprivateコンストラクタ
  private constructor() {}

  /**
   * Axiosインスタンスを取得するメソッド
   * @returns AxiosInstance - 設定済みのAxiosインスタンス
   */
  public static getInstance(): AxiosInstance {
    // インスタンスが未作成の場合、新しく作成する
    if (!Axios.instance) {
      Axios.instance = axios.create({
        withCredentials: true  // CORSリクエストでクッキーを送信可能にする
      });
    }
    // 既存のインスタンスを返す
    return Axios.instance;
  }
}

export default Axios;