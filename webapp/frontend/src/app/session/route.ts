import { cookies } from "next/headers";
import { NextRequest, NextResponse } from "next/server";

/**
 * GETリクエストを処理する関数
 * セッション情報を取得し、認証されていない場合は401エラーを返す
 * @returns NextResponse - セッション情報またはエラーメッセージを含むレスポンス
 */
export async function GET() {
  // クッキーからセッション情報を取得
  const session = cookies().get("session")?.value;

  // セッションが存在しない場合は401エラーを返す
  if (!session) {
    return NextResponse.json({ message: "Unauthorized" }, { status: 401 });
  }

  // セッション情報をJSON形式で返す
  return NextResponse.json(JSON.parse(session), { status: 200 });
}

/**
 * POSTリクエストを処理する関数
 * リクエストボディからセッション情報を取得し、クッキーに設定する
 * @param req - NextRequestオブジェクト
 * @returns NextResponse - 成功メッセージを含むレスポンス
 */
export async function POST(req: NextRequest) {
  // リクエストボディからセッション情報を取得
  const session = await req.json();
  
  // セッション情報をクッキーに設定（有効期限3600秒）
  cookies().set("session", JSON.stringify(session), { maxAge: 3600 });

  // 成功メッセージを返す
  return NextResponse.json({ message: "Set cookie successfully" });
}

/**
 * DELETEリクエストを処理する関数
 * セッションクッキーを削除する
 * @returns NextResponse - 成功メッセージを含むレスポンス
 */
export async function DELETE() {
  // セッションクッキーを削除
  cookies().delete("session");

  // 成功メッセージを返す
  return NextResponse.json({ message: "Delete cookie successfully" });
}