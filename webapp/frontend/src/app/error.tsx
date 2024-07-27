// "use client" ディレクティブは、このコンポーネントがクライアントサイドでレンダリングされることを示します。
"use client";

import { useEffect } from "react";

// エラーハンドリング用のカスタムコンポーネント
export default function Error({ error }: { error: Error & { digest?: string } }) {
  // useEffectフックを使用して、エラーが発生した際にコンソールにエラーメッセージをログ出力
  useEffect(() => {
    console.error(error);
  }, [error]); // errorが変更されるたびにこのエフェクトが実行される

  return (
    <div id="error-page">
      {/* ユーザーフレンドリーなエラーメッセージを表示 */}
      <h1>Oops!</h1>
      <p>Sorry, an unexpected error has occurred.</p>
      <p>
        {/* 実際のエラーメッセージを表示 */}
        <i>{error.message}</i>
      </p>
    </div>
  );
}

