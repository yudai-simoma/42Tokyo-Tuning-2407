/**
 * APIリクエストを行うためのFetchクラス
 * Singletonパターンを使用して実装されています
 */
class Fetch {
  // APIのベースURLを環境変数から取得、デフォルトは "http://nginx"
  private static baseURL: string = process.env.API_BASE_URL || "http://nginx";

  // Fetchクラスのインスタンスを保持する静的プロパティ
  private static instance: Fetch;

  /**
   * Fetchインスタンスを取得するメソッド
   * @returns Fetch - 設定済みのFetchインスタンス
   */
  public static getInstance(): Fetch {
    if (!Fetch.instance) {
      Fetch.instance = new Fetch();
    }
    return Fetch.instance;
  }

  /**
   * APIリクエストを実行するメソッド
   * @param endpoint - APIエンドポイント
   * @param options - フェッチオプション（オプショナル）
   * @returns Promise<T> - APIレスポンスの型付きPromise
   */
  public async fetch<T>(endpoint: string, options?: RequestInit): Promise<T> {
    try {
      // ベースURLとエンドポイントを結合し、キャッシュを無効化してリクエストを送信
      const response = await fetch(`${Fetch.baseURL}${endpoint}`, { cache: "no-cache", ...options });

      // レスポンスが正常でない場合はエラーをスロー
      if (!response.ok) {
        throw new Error(`Fetch request failed with status ${response.status}`);
      }

      // レスポンスをJSON形式で解析して返す
      return await response.json();
    } catch (error) {
      // エラーをコンソールに出力し、再スロー
      console.error("Fetch error:", error);
      throw error;
    }
  }
}

export default Fetch;