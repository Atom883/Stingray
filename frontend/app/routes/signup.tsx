import React, { useState } from "react";
import Cookies from "js-cookie";

const Signup: React.FC = () => {
	const [id, setId] = useState("");
	const [password, setPassword] = useState("");

	const handleSignup = async () => {
		try {
			const response = await fetch("/api/signup", {
				method: "POST",
				headers: {
					"Content-Type": "application/json",
				},
				body: JSON.stringify({ id, password }),
			});

			if (response.ok) {
				const data = await response.json();
				Cookies.set("sessionId", data.sessionId);
				alert("サインアップ成功");
			} else {
				alert("サインアップ失敗");
			}
		} catch (error) {
			console.error("エラー:", error);
			alert("エラーが発生しました");
		}
	};

	return (
		<div
			style={{
				display: "flex",
				flexDirection: "column",
				alignItems: "center",
				justifyContent: "center",
				height: "100vh",
				backgroundColor: "#f0f0f0",
			}}
		>
			<div
				style={{
					padding: "20px",
					border: "1px solid #ccc",
					borderRadius: "5px",
					backgroundColor: "white",
				}}
			>
				<h2 style={{ textAlign: "center", marginBottom: "20px" }}>
					サインアップ
				</h2>
				<div style={{ marginBottom: "10px" }}>
					<input
						type="text"
						placeholder="ID"
						value={id}
						onChange={(e) => setId(e.target.value)}
						style={{
							padding: "10px",
							width: "250px",
							borderRadius: "3px",
							border: "1px solid #ccc",
						}}
					/>
				</div>
				<div style={{ marginBottom: "10px" }}>
					<input
						type="password"
						placeholder="パスワード"
						value={password}
						onChange={(e) => setPassword(e.target.value)}
						style={{
							padding: "10px",
							width: "250px",
							borderRadius: "3px",
							border: "1px solid #ccc",
						}}
					/>
				</div>
				<button
					onClick={handleSignup}
					style={{
						padding: "10px 20px",
						backgroundColor: "#007bff",
						color: "white",
						border: "none",
						borderRadius: "3px",
						cursor: "pointer",
						width: "270px",
					}}
				>
					サインアップ
				</button>
			</div>
		</div>
	);
};

export default Signup;
