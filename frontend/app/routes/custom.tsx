import { useEffect, useState } from "react";
import { Link } from "react-router-dom";

export default function Custom() {
	// ▼ State
	const [fontColor, setFontColor] = useState("#FF0000");
	const [isBold, setIsBold] = useState(false);
	const [isOutlined, setIsOutlined] = useState(false);
	const [fontFamily, setFontFamily] = useState("Arial, sans-serif");

	// ▼ フォント候補
	const fontList = [
		{ label: "Arial", value: "Arial, sans-serif" },
		{ label: "Times New Roman", value: "'Times New Roman', serif" },
		{ label: "Georgia", value: "Georgia, serif" },
		{ label: "Comic Sans MS", value: "'Comic Sans MS', cursive, sans-serif" },
		{ label: "Impact", value: "Impact, sans-serif" },
		{ label: "Monospace", value: "monospace" },
		{ label: "Courier New", value: "'Courier New', monospace" },
		{ label: "Trebuchet MS", value: "'Trebuchet MS', sans-serif" },
	];

	// ▼ 色候補
	const colors = [
		{ label: "Red", value: "#FF0000" },
		{ label: "Blue", value: "#0000FF" },
		{ label: "Green", value: "#008000" },
		{ label: "Orange", value: "#FFA500" },
		{ label: "Purple", value: "#800080" },
		{ label: "Black", value: "#000000" },
		{ label: "Gray", value: "#808080" },
		{ label: "White", value: "#FFFFFF" },
	];

	// ▼ "A" に適用する最終スタイル
	const aStyle = {
		fontSize: "500px",
		color: fontColor,
		fontWeight: isBold ? "bold" : "normal",
		textShadow: isOutlined
			? `-1px 0px 2px #000, 0px 1px 2px #000,
         1px 0px 2px #000, 0px -1px 2px #000`
			: "2px 2px 6px rgba(0, 0, 0, 0.5)",
		transition: "all 0.3s ease",
		fontFamily: fontFamily,
	};

	useEffect(() => {
		// スクロールを禁止
		document.body.style.overflow = "hidden";

		// アンマウント時にスクロールを許可
		return () => {
			document.body.style.overflow = "auto";
		};
	}, []);

	return (
		<div
			style={{
				width: "100%",
				height: "100vh",
				display: "flex", // 左右に分割
				position: "relative",
				backgroundImage:
					"url('https://thumb.ac-illust.com/68/680c27809fecb90197d2f0d7a789c66a_t.jpeg')", // 背景画像
				backgroundSize: "cover", // 背景画像をカバー
				backgroundPosition: "center", // 画像を中央に配置
				overflow: "hidden", // 小さな文字が画面外に出ないようにする
				backgroundColor: "rgba(184, 171, 171, 0.8)",
				backgroundBlendMode: "overlay",
			}}
		>
			{/* ◆ 左側：フォームを中央に配置するコンテナ */}
			<div
				style={{
					width: "50%",
					padding: "20px 0 0 60px",
					display: "flex",
					justifyContent: "center", // 水平中央
					alignItems: "center", // 垂直中央
				}}
			>
				{/* ◆ フォーム本体（パネル風） */}
				<div
					style={{
						backgroundColor: "rgba(255, 255, 255, 0.8)",
						border: "4px solid #F08030",
						borderRadius: "20px",
						boxShadow: "0 8px 12px rgba(0, 0, 0, 0.3)",
						padding: "50px",
						fontFamily: "'Press Start 2P', cursive",
						textAlign: "center",
					}}
				>
					<h2
						style={{
							marginBottom: "35px",
							color: "#222",
							fontSize: "1.8rem",
							fontWeight: "bold",
						}}
					>
						Customize "A"
					</h2>

					{/* ▼ フォント選択 */}
					<div style={{ marginBottom: "20px" }}>
						<label style={{ fontSize: "1.2rem", marginRight: "8px" }}>
							Font:
						</label>
						<select
							value={fontFamily}
							onChange={(e) => setFontFamily(e.target.value)}
							style={{ fontSize: "1.2rem", padding: "8px" }}
						>
							{fontList.map((f) => (
								<option key={f.value} value={f.value}>
									{f.label}
								</option>
							))}
						</select>
					</div>

					{/* ▼ 色選択 */}
					<div style={{ marginBottom: "20px" }}>
						<label style={{ fontSize: "1.2rem", marginRight: "8px" }}>
							Color:
						</label>
						<select
							value={fontColor}
							onChange={(e) => setFontColor(e.target.value)}
							style={{ fontSize: "1.2rem", marginRight: "8px" }}
						>
							{colors.map((c) => (
								<option key={c.value} value={c.value}>
									{c.label}
								</option>
							))}
						</select>
					</div>

					{/* ▼ Bold / Outline */}
					<div style={{ marginBottom: "20px" }}>
						<label style={{ fontSize: "1.2rem", marginRight: "15px" }}>
							<input
								type="checkbox"
								checked={isBold}
								onChange={(e) => setIsBold(e.target.checked)}
							/>
							Bold
						</label>
						<label style={{ fontSize: "1.2rem", marginRight: "15px" }}>
							<input
								type="checkbox"
								checked={isOutlined}
								onChange={(e) => setIsOutlined(e.target.checked)}
							/>
							Outline
						</label>
					</div>
				</div>
			</div>

			{/* ◆ 右側：Aを中央に表示 */}
			<div
				style={{
					width: "50%",
					padding: "0 60px 0 0",
					display: "flex",
					justifyContent: "center", // Aを横方向中央
					alignItems: "center", // Aを縦方向中央
				}}
			>
				<div style={aStyle}>A</div>
			</div>

			{/* ◆ 右下にホームボタン */}
			<div
				style={{
					position: "absolute",
					bottom: "20px",
					right: "20px",
				}}
			>
				<Link to="/">
					<button
						style={{
							padding: "10px 20px",
							fontSize: "1rem",
							borderRadius: "8px",
							border: "none",
							backgroundColor: "#F08030",
							color: "#fff",
							cursor: "pointer",
							boxShadow: "0 2px 4px rgba(0, 0, 0, 0.3)",
						}}
					>
						Home
					</button>
				</Link>
			</div>
		</div>
	);
}
