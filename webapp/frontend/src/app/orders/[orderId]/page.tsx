import { fetchOrder } from "@/api/order";
import { User } from "@/api/user";
import { OrderDetail } from "@/components/order";
import { Container, Typography } from "@mui/material";
import { NextPage } from "next";
import { cookies } from "next/headers";

// ページコンポーネントのプロパティ型定義
type Props = {
  params: {
    orderId: string;
  };
};

/**
 * 注文詳細ページのコンポーネント
 * @param params - URLパラメータ（orderId）
 */
const Order: NextPage<Props> = async ({ params }) => {
  const { orderId } = params;
  
  // クッキーからセッション情報を取得
  const session = cookies().get("session");

  let sessionToken = "";
  if (session) {
    // セッションが存在する場合、ユーザー情報を解析してトークンを取得
    const user: User = JSON.parse(session.value);
    sessionToken = user.session_token;
  }

  // 注文情報をAPIから取得
  const order = await fetchOrder(orderId, sessionToken);

  return (
    <Container>
      <Typography variant="h2" gutterBottom>
        リクエスト詳細
      </Typography>
      {/* 注文詳細コンポーネントを表示 */}
      <OrderDetail order={order} />
    </Container>
  );
};

export default Order;

// ビルド時にSSGを防ぎ、常に動的レンダリングを強制する
export const dynamic = "force-dynamic";