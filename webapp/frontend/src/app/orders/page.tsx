import { Container, Typography } from "@mui/material";
import { NextPage } from "next";
import { fetchOrders, OrdersQueryParams } from "@/api/order";
import { OrderTable } from "@/components/order";
import { cookies } from "next/headers";
import { User } from "@/api/user";
import { redirect } from "next/navigation";

// ページコンポーネントのプロパティ型定義
type Props = {
  searchParams: OrdersQueryParams;
};

/**
 * 注文リストページのコンポーネント
 * @param searchParams - クエリパラメータ（注文のフィルタリングやソート条件）
 */
const Orders: NextPage<Props> = async ({ searchParams }) => {
  // クッキーからセッション情報を取得
  const session = cookies().get("session");

  let area: number | null = null;
  let sessionToken = "";

  if (session) {
    // セッションが存在する場合、ユーザー情報を解析してトークンを取得
    const user: User = JSON.parse(session.value);
    sessionToken = user.session_token;

    // ユーザーがディスパッチャーの場合、エリアIDを設定
    if (user.role === "dispatcher") {
      area = user.area_id;
    }
  } else {
    // セッションが存在しない場合、ログインページにリダイレクト
    redirect("/login");
  }

  // 注文リストをAPIから取得
  const orders = await fetchOrders(searchParams, area, sessionToken);

  return (
    <Container>
      <Typography variant="h2" gutterBottom>
        リクエスト一覧
      </Typography>
      {/* 注文リストを表示するテーブルコンポーネント */}
      <OrderTable orders={orders} />
    </Container>
  );
};

export default Orders;

// ビルド時にSSGを防ぎ、常に動的レンダリングを強制する
export const dynamic = "force-dynamic";