import React, { useState, useEffect, use } from "react";
import { useNavigate } from "react-router";
import FinishFishBtn from "~/components/FinishFishBtn";
import NowFish from "~/components/NowFish";

type Alphabet = {
	char: string;
	x: number;
	y: number;
	direction: "left" | "right";
};

const alphabetList: string[] = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".split("");

export default function AnimatedAlphabets() {
	const [alphabets, setAlphabets] = useState<Alphabet[]>([]);
	const [positionX, setPositionX] = useState(0);
	const [positionY, setPositionY] = useState(0);
	const [showNowFish, setShowNowFish] = useState(false);
	const [caughtAlphabet, setCaughtAlphabet] = useState<Alphabet | null>(null);
	const [caughtAlphabets, setCaughtAlphabets] = useState<{ [key: string]: number }>({}); 
	const navigate = useNavigate();

	useEffect(() => {
		// スクロールを禁止
		document.body.style.overflow = "hidden";

		// アンマウント時にスクロールを許可
		return () => {
			document.body.style.overflow = "auto";
		};
	}, []);

	useEffect(() => {
		// 初期アルファベットの配置と方向を設定
		const initialAlphabets: Alphabet[] = alphabetList.map((char) => ({
			char,
			x: Math.random() * window.innerWidth * 0.8 + window.innerWidth * 0.1,
			y: Math.random() * window.innerHeight * 0.4 + window.innerHeight * 0.5,
			direction: Math.random() < 0.5 ? "left" : "right",
		}));
		setAlphabets(initialAlphabets);
	}, []);

	useEffect(() => {
		const interval = setInterval(() => {
			setAlphabets((prevAlphabets) =>
				prevAlphabets.map((alphabet) => {
					const speed = 2; // アルファベットの移動速度
					let newX = alphabet.x;
					let newDirection = alphabet.direction;

					if (alphabet.direction === "left") {
						newX -= speed;
						if (newX < 0) {
							newDirection = "right";
							newX = 0;
						}
					} else {
						newX += speed;
						if (newX > window.innerWidth) {
							newDirection = "left";
							newX = window.innerWidth;
						}
					}

					return {
						...alphabet,
						x: newX,
						direction: newDirection,
					};
				}),
			);
		}, 50);

		return () => clearInterval(interval);
	}, []);

	// 針と糸の移動
	useEffect(() => {
		const handleKeyDown = (event: KeyboardEvent) => {
			const step = 10;
			if (event.key === "ArrowLeft") {
				setPositionX((prev) => prev - step);
			} else if (event.key === "ArrowRight") {
				setPositionX((prev) => prev + step);
			} else if (event.key === "ArrowUp") {
				setPositionY((prev) => prev - step);
			} else if (event.key === "ArrowDown") {
				setPositionY((prev) => prev + step);
			} else if (event.key === " " && showNowFish) {
				// スペースキーでアルファベットを捕まえる
				if (caughtAlphabet) {
					setCaughtAlphabets((prevCaughtAlphabets) => ({
                        ...prevCaughtAlphabets,
                        [caughtAlphabet.char]: (prevCaughtAlphabets[caughtAlphabet.char] || 0) + 1,
                    })); 
					setCaughtAlphabet(null); // caughtAlphabetをリセットする
					setAlphabets((alpfabet) =>
						alpfabet.filter(
							(alphabet) => alphabet.char !== caughtAlphabet.char,
						),
					);
				}
			}
		};

		window.addEventListener("keydown", handleKeyDown);

		return () => {
			window.removeEventListener("keydown", handleKeyDown);
		};
	}, [showNowFish, caughtAlphabet]);

	useEffect(() => {
		let isInRange = false; // 初期値をfalseに設定

		for (const alphabet of alphabets) {
			const hookTipY = positionY + window.innerHeight - 20; // 針の先端のy座標を計算
			const distanceX = Math.abs(positionX - alphabet.x + 100);
			const distanceY = Math.abs(hookTipY - alphabet.y); // 針の先端とアルファベットの距離

			if (distanceX < 20 && distanceY < 20) {
				// 当たり判定
				isInRange = true; // 範囲内であればtrueに変更
				setCaughtAlphabet(alphabet);
				break; // 一度でも範囲内になったらループを抜ける
			}
		}

		setShowNowFish(isInRange); // ループ後に状態を更新
	}, [positionX, positionY, alphabets]);

	const handleFinishFish = async () => {
		try {
			const response = await fetch(
				"http://localhost/api/add_fish",
				{
					method: "POST",
					headers: {
						"Content-Type": "application/json",
					},
					body: JSON.stringify(caughtAlphabets),
				},
			);
			if (response.ok) {
				console.log("釣り上げたアルファベット:", caughtAlphabets);
				navigate("/");
			}
		} catch (error) {
			console.error("エラー:", error);
			alert("エラーが発生しました");
		}
	};

	return (
		<div style={{ position: "relative", height: "100vh" }}>
			{/* 背景画像 */}
			<img
				src="https://thumb.photo-ac.com/65/65bef188873dc91decd37fe0d23fee22_t.jpeg"
				alt=""
				style={{
					position: "absolute",
					width: "100%",
					height: "100%",
					objectFit: "cover",
				}}
			/>
			{/* アルファベット */}
			{alphabets.map((alphabet) => (
				<div
					key={alphabet.char}
					style={{
						position: "absolute",
						top: alphabet.y + "px",
						left: alphabet.x + "px",
						fontSize: "30px",
						color: "red",
						transform: "translate(-50%, -50%)",
					}}
				>
					{alphabet.char}
				</div>
			))}
			<div style={{ position: "relative", width: "200px", height: "300px" }}>
				{/* ふにゃふにゃの糸 */}
				<svg
					width="200"
					height={window.innerHeight}
					style={{
						position: "absolute",
						top: `calc(0px + ${positionY}px)`,
						left: `calc(50% + ${positionX}px)`,
						transform: "translateX(-50%)",
					}}
				>
					<path
						d={`M 100 0 C 80 50, 120 100, 100 150 S 80 ${window.innerHeight}, 100 ${window.innerHeight}`}
						stroke="gray"
						strokeWidth="2"
						fill="transparent"
					/>
				</svg>

				{/* 針 */}
				<div
					style={{
						position: "absolute",
						top: `calc(${window.innerHeight}px + ${positionY}px - 20px)`,
						left: `calc(50% + ${positionX}px)`,
						transform: "translateX(-50%)",
						width: "0",
						height: "0",
						borderLeft: "10px solid transparent",
						borderRight: "10px solid transparent",
						borderTop: "20px solid black",
					}}
				/>

				{/* 当たり判定範囲 */}
				<div
					style={{
						position: "absolute",
						top: `calc(${window.innerHeight}px + ${positionY}px - 20px)`,
						left: `calc(50% + ${positionX}px)`,
						transform: "translateX(-50%)",
						width: "0",
						height: "0",
						borderLeft: "10px solid transparent",
						borderRight: "10px solid transparent",
						borderTop: `20px solid ${showNowFish ? "red" : "black"}`, // showNowFishの値に応じて色を変更
					}}
				/>
			</div>
			{showNowFish && <NowFish />}
			{<FinishFishBtn onClick={handleFinishFish} />}
		</div>
	);
}
