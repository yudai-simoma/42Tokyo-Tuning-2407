use sqlx::mysql::MySqlPool;
use std::env;

/// データベース接続プールを作成する関数
/// 
/// 環境変数 `DATABASE_URL` を使用してデータベースに接続します。
/// 成功した場合、接続プール `MySqlPool` を返します。
/// 失敗した場合、パニックを引き起こします。
/// 
/// ボトルネックになりうる箇所: データベース接続の確立
/// - データベース接続の確立は時間がかかる可能性があるため、非同期処理として実装されています。
pub async fn create_pool() -> MySqlPool {
    // 環境変数 `DATABASE_URL` を取得
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    // データベースに接続して接続プールを作成
    MySqlPool::connect(&database_url)
        .await
        .expect("Failed to create pool")
}