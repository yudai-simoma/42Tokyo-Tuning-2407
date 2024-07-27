"use client";

import { Order } from "@/api/order";
import { formatDateTime } from "@/utils/day";
import { Table, TableBody, TableCell, TableHead, TableRow, TextField } from "@mui/material";
import Image from "next/image";
import { useEffect, useMemo, useState } from "react";
import styles from "./OrderTable.module.scss";
import { useRouter } from "next/navigation";

type Props = {
  orders: Order[];
};

// 注文テーブルを表示するコンポーネント
const OrderTable: React.FC<Props> = ({ orders }) => {
  // 検索用の状態管理
  const [search, setSearch] = useState("");

  // 検索条件に基づいて注文をフィルタリング
  const filteredOrders = useMemo(() => {
    return orders.filter(
      (order) =>
        order.status.includes(search) ||
        order.client_username.includes(search) ||
        order.dispatcher_username?.includes(search) ||
        order.driver_username?.includes(search)
    );
  }, [search]);

  const router = useRouter();

  // 行クリック時の処理
  const handleRowClick = (orderId: number) => () => {
    router.push(`/orders/${orderId}`);
  };

  // ユーザー画像のローダー関数
  const imageLoader = (userId: number) => () => {
    return `/api/user_image/${userId}`;
  };

  // k6の負荷試験用：イメージのロードが完了したらaltに"completed"を追加
  const completedImage = () => {
    const images = document.querySelectorAll("img");
    images.forEach((image) => {
      image.addEventListener("load", () => {
        const alt = image.alt;
        if (!alt.includes("(completed)")) {
          image.alt = alt + " (completed)";
        }
      });
    });
  };

  // コンポーネントマウント時に画像完了処理を実行
  useEffect(() => {
    completedImage();
  }, []);

  return (
    <>
      {/* 検索フィールド */}
      <TextField
        fullWidth
        label="Search"
        value={search}
        onChange={(e) => setSearch(e.target.value)}
        style={{ marginBottom: "16px" }}
      />
      {/* 注文テーブル */}
      <Table id="order-table">
        <TableHead>
          <TableRow>
            <TableCell></TableCell>
            <TableCell>ステータス</TableCell>
            <TableCell>クライアント名</TableCell>
            <TableCell>ディスパッチャー名</TableCell>
            <TableCell>ドライバー名</TableCell>
            <TableCell>リクエスト日時</TableCell>
            <TableCell>リクエスト完了日時</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {/* フィルタリングされた注文を最大10件表示 */}
          {filteredOrders.slice(0, 10).map((order) => (
            <TableRow key={order.id} className={styles.row} hover onClick={handleRowClick(order.id)}>
              <TableCell>{order.id}</TableCell>
              <TableCell>{order.status}</TableCell>
              <TableCell>
                <div className={styles.user}>
                  {/* クライアントの画像 */}
                  <Image
                    className={styles.icon}
                    loader={imageLoader(order.client_id)}
                    src={`${order.client_id}.png`}
                    alt={`Picture of the user: ${order.client_id}`}
                    width={50}
                    height={50}
                  />
                  <span>{order.client_username}</span>
                </div>
              </TableCell>
              <TableCell>
                {order.dispatcher_user_id ? (
                  <div className={styles.user}>
                    {/* ディスパッチャーの画像 */}
                    <Image
                      className={styles.icon}
                      loader={imageLoader(order.dispatcher_user_id)}
                      src={`${order.dispatcher_user_id}.png`}
                      alt={`Picture of the user(${order.dispatcher_user_id})`}
                      width={50}
                      height={50}
                    />
                    <span>{order.dispatcher_username}</span>
                  </div>
                ) : (
                  <>-</>
                )}
              </TableCell>
              <TableCell>
                {order.driver_user_id ? (
                  <div className={styles.user}>
                    {/* ドライバーの画像 */}
                    <Image
                      className={styles.icon}
                      loader={imageLoader(order.driver_user_id)}
                      src={`${order.driver_user_id}.png`}
                      alt={`Picture of the user(${order.driver_user_id})`}
                      width={50}
                      height={50}
                    />
                    <span>{order.driver_username}</span>
                  </div>
                ) : (
                  <>-</>
                )}
              </TableCell>
              <TableCell>{formatDateTime(order.order_time, "YYYY年MM月DD日 HH:mm:ss")}</TableCell>
              <TableCell>
                {order.completed_time ? formatDateTime(order.completed_time, "YYYY年MM月DD日 HH:mm:ss") : "未完了"}
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </>
  );
};

export default OrderTable;
