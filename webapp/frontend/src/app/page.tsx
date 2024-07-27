"use client";

import { Button, Container } from "@mui/material";
import { useAuth } from "@/context/AuthContext";
import { useRouter } from "next/navigation";
import { NextPage } from "next";
import { logout } from "@/api/user";

/**
 * ホームページのコンポーネント
 * @returns ホームページのJSX要素
 */
const Home: NextPage = () => {
  // Next.jsのルーターを使用
  const router = useRouter();
  
  // 認証情報を取得
  const { sessionToken, removeUserInfo } = useAuth();

  /**
   * ログアウト処理を行う関数
   */
  const handleLogout = async () => {
    // ログアウトAPIを呼び出し
    await logout(sessionToken);
    // ユーザー情報を削除
    removeUserInfo();
    // ログインページにリダイレクト
    router.push("/login");
  };

  return (
    <Container>
      <h2>レッカー車配車アプリケーション</h2>
      <div>
        {/* リクエスト一覧ページへの遷移ボタン */}
        <Button id="button-requests-page" variant="contained" onClick={() => router.push("/orders")}>
          クライアントからのリクエスト一覧ページ
        </Button>
        {/* ログアウトボタン */}
        <button id="button-logout" onClick={handleLogout}>
          Logout
        </button>
      </div>
    </Container>
  );
};

export default Home;
