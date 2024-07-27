"use client";

import { getSession, Role, User } from "@/api/user";
import React, { createContext, useContext, useState, ReactNode, useEffect } from "react";

// 認証コンテキストの型定義
interface AuthContextType {
  isAuthenticated: boolean;
  sessionToken: string | null;
  role: Role | null;
  userId: number | null;
  dispatcherId: number | null;
  areaId: number | null;
  setUserInfo: (user: User) => void;
  removeUserInfo: () => void;
}

// 認証コンテキストの作成
const AuthContext = createContext<AuthContextType | undefined>(undefined);

// 認証プロバイダーコンポーネント
export const AuthProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  // 認証状態を管理するための状態変数
  const [isAuthenticated, setIsAuthenticated] = useState<boolean | undefined>(undefined);
  const [sessionToken, setSessionToken] = useState<string | null>(null);
  const [role, setRole] = useState<Role | null>(null);
  const [userId, setUserId] = useState<number | null>(null);
  const [dispatcherId, setDispatcherId] = useState<number | null>(null);
  const [areaId, setAreaId] = useState<number | null>(null);

  // ユーザー情報を設定する内部関数
  const setUserInfoState = (user: User) => {
    setSessionToken(user.session_token);
    setRole(user.role);
    setUserId(user.user_id);
    if (user.role === "dispatcher") {
      setDispatcherId(user.dispatcher_id);
      setAreaId(user.area_id);
    }
  };

  // ユーザー情報を設定し、認証状態を更新する関数
  const setUserInfo = (user: User) => {
    setIsAuthenticated(true);
    setUserInfoState(user);
  };

  // ユーザー情報をクリアし、認証状態をリセットする関数
  const removeUserInfo = () => {
    setIsAuthenticated(false);
    setSessionToken(null);
    setRole(null);
    setUserId(null);
    setDispatcherId(null);
    setAreaId(null);
  };

  // ユーザーの認証状態を確認する関数
  const verifyUser = async () => {
    if (window.location.pathname === "/login") {
      setIsAuthenticated(false);
      return;
    }

    const session = await getSession();
    if (!session) {
      setIsAuthenticated(false);
      return;
    }

    setIsAuthenticated(true);
    setUserInfoState(session);
  };

  // コンポーネントマウント時にユーザー認証を確認
  useEffect(() => {
    verifyUser();
  }, []);

  // 認証状態が未確定の場合は何も表示しない
  if (isAuthenticated === undefined) {
    return <></>;
  }

  // 認証コンテキストプロバイダーを返す
  return (
    <AuthContext.Provider
      value={{ isAuthenticated, sessionToken, role, userId, dispatcherId, areaId, setUserInfo, removeUserInfo }}
    >
      {children}
    </AuthContext.Provider>
  );
};

// 認証情報を使用するためのカスタムフック
export const useAuth = (): AuthContextType => {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error("useAuth must be used within an AuthProvider");
  }
  return context;
};
