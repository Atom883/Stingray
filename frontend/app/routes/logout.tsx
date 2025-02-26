import Cookies from "js-cookie";

export const logout = async () => {
	try {
		await fetch("/api/logout", { method: "POST" });
		Cookies.remove("sessionId"); // セッションIDをクッキーから削除
		alert("ログアウトしました");
		// 必要に応じてリダイレクト処理を追加
	} catch (error) {
		console.error("ログアウトエラー:", error);
		alert("ログアウトに失敗しました");
	}
};
