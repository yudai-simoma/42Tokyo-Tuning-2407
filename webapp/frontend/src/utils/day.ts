import dayjs from "dayjs";

/**
 * 日付と時間をフォーマットする関数
 * @param dateTimeString - フォーマットする日付と時間の文字列
 * @param format - 出力フォーマットを指定する文字列
 * @returns フォーマットされた日付と時間の文字列
 */
export const formatDateTime = (dateTimeString: string, format: string) => {
  // dayjsを使用して日付文字列を指定されたフォーマットに変換
  return dayjs(dateTimeString).format(format);
};
