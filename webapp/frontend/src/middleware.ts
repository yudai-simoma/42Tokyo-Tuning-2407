import { cookies } from "next/headers";
import { NextResponse, NextRequest } from "next/server";

/**
 * ミドルウェア関数
 * リクエストが認証されているかをチェックし、認証されていない場合はログインページにリダイレクトする
 * @param req - NextRequestオブジェクト
 * @returns NextResponse - 次の処理に進むか、リダイレクトするレスポンス
 */
export function middleware(req: NextRequest) {
  // クッキーからセッション情報を取得
  const session = cookies().get("session");

  // セッションが存在しない場合、ログインページにリダイレクト
  if (!session) {
    return NextResponse.redirect(new URL("/login", req.url));
  }

  // セッションが存在する場合、次の処理に進む
  return NextResponse.next();
}

// ミドルウェアの設定
export const config = {
  // ミドルウェアを適用するパスのパターンを指定
  matcher: ["/", "/orders/:path*"]
};
