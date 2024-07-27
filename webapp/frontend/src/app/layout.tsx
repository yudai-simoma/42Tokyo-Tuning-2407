import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.scss";
import { AuthProvider } from "@/context/AuthContext";

// Interフォントを設定（latinサブセットを使用）
const inter = Inter({ subsets: ["latin"] });

// アプリケーションのメタデータを設定
export const metadata: Metadata = {
  title: "42Tokyo-tuning-2407"
};

// ビューポートの設定
export const viewport = "width=device-width, initial-scale=1";

/**
 * ルートレイアウトコンポーネント
 * すべてのページで共通して使用されるレイアウトを定義
 * @param children - 子コンポーネント（各ページの内容）
 */
export default function RootLayout({
  children
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="ja">
      <body className={inter.className} style={{ backgroundColor: "white" }}>
        {/* AuthProviderで子コンポーネントをラップし、認証状態を提供 */}
        <AuthProvider>{children}</AuthProvider>
      </body>
    </html>
  );
}
