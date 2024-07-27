"use client";

import { useState, useEffect } from "react";
import { TextField, Button, Container, Typography } from "@mui/material";
import { useAuth } from "@/context/AuthContext";
import { useRouter } from "next/navigation";
import { NextPage } from "next";
import { login } from "@/api/user";

/**
 * ログインページのコンポーネント
 * @returns ログインフォームを含むJSX要素
 */
const Login: NextPage = () => {
  // ユーザー名、パスワード、エラーメッセージの状態を管理
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");
  
  // Next.jsのルーターを使用
  const router = useRouter();
  
  // 認証情報を取得
  const { isAuthenticated, setUserInfo } = useAuth();

  /**
   * フォーム送信時の処理
   * @param event - フォームイベント
   */
  const handleSubmit = async (event: React.FormEvent) => {
    event.preventDefault();
    try {
      // ログインAPIを呼び出し、ユーザー情報を取得
      const data = await login(username, password);
      // ユーザー情報をコンテキストに設定
      setUserInfo(data);
      // ホームページにリダイレクト
      router.push("/");
    } catch {
      // エラーメッセージを設定
      setError("ユーザ名もしくはパスワードが違います");
    }
  };

  // 認証済みの場合、ホームページにリダイレクト
  useEffect(() => {
    if (isAuthenticated) {
      router.replace("/");
    }
  }, [isAuthenticated, router]);

  // 認証済みの場合は何も表示しない
  if (isAuthenticated) {
    return null;
  }

  return (
    <Container maxWidth="sm">
      <Typography variant="h2" gutterBottom>
        ログイン
      </Typography>
      {error && <Typography color="error">{error}</Typography>}
      <form onSubmit={handleSubmit}>
        <TextField
          fullWidth
          id="input-username"
          label="Username"
          margin="normal"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
          required
        />
        <TextField
          fullWidth
          id="input-password"
          label="Password"
          type="password"
          margin="normal"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          required
        />
        <Button
          fullWidth
          id="button-login"
          variant="contained"
          color="primary"
          type="submit"
          style={{ marginTop: "16px" }}
        >
          Login
        </Button>
      </form>
    </Container>
  );
};

export default Login;