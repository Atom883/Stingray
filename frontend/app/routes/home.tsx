import { use, useEffect, useState } from "react";
import { Link } from "react-router-dom";
import Cookies from "js-cookie";
import { useAtom } from 'jotai';
import { userDataAtom } from '../jotai/atoms';
import homeBgm from "../music/homeBgm.mp3"

export default function Home() {

  //bgmの再生
  useEffect(() => {
    const audio = new Audio(homeBgm);
    audio.loop = true;
    audio.play();

    return () => {
      audio.pause(); // 再生を停止
      audio.currentTime = 0; // 再生位置をリセット
    };
  },[])

	const [userData, setUserData] = useAtom(userDataAtom);
	// 仮のデータ（釣りで獲得したアルファベット）
	const [letters, setLetters] = useState([
		"A",
		"B",
		"C",
		"D",
		"E",
		"F",
		"G",
		"H",
		"I",
		"J",
		"K",
	]);
	const [currentPage, setCurrentPage] = useState(1);
	const itemsPerPage = 3; // 1ページに表示するアイテム数

	// ページネーションのロジック
	const totalPages = Math.ceil(letters.length / itemsPerPage);
	const currentLetters = letters.slice(
		(currentPage - 1) * itemsPerPage,
		currentPage * itemsPerPage,
	);

	const goToPage = (page: number) => {
		if (page < 1 || page > totalPages) return;
		setCurrentPage(page);
	};

	// 栄養ボタンがクリックされたときの動作
	const [isNutritionVisible, setIsNutritionVisible] = useState(false);

	const toggleNutrition = () => {
		setIsNutritionVisible(!isNutritionVisible);
	};

	// APIを呼び出す関数
	const handleDelete = async (letter: string) => {
		try {
			const response = await fetch("http://localhost:8000/api/eat-fish", {
				method: "POST",
				headers: {
					"Content-Type": "application/json",
				},
				body: JSON.stringify({ letter }), // IDを送信
			});

			if (response.ok) {
				const data = await response.json();
				Cookies.set("sessionId", data.sessionId);
				alert("削除成功");
			} else {
				alert("削除失敗");
			}
		} catch (error) {
			console.error("エラー:", error);
			alert("エラーが発生しました");
		}
	};

	// アルファベットクリック時の処理
	const handleLetterClick = async (letter: string) => {
		try {
			await handleDelete(letter); 

			setLetters((prevLetters) => prevLetters.filter((item) => item !== letter));
		} catch (error) {
			console.error("APIエラー:", error);
		}
	};


	return (
		<div
			style={{
				height: "100vh", // Full viewport height
				display: "flex",
				justifyContent: "center",
				alignItems: "center",
				flexDirection: "column",
				position: "relative",
				backgroundImage:
					"url('https://sakuranouta.biz/wp-content/uploads/2023/07/%E6%B5%B7%E3%81%AE%E4%B8%AD%E3%81%AE%E9%83%A8%E5%B1%8B_3.png')", // 背景画像
				backgroundSize: "cover", // 背景画像をカバー
				backgroundPosition: "center", // 画像を中央に配置
				overflow: "hidden", // 小さな文字が画面外に出ないようにする
				backgroundColor: "rgba(184, 171, 171, 0.8)",
				backgroundBlendMode: "overlay",
			}}
		>
			<div
				style={{
					position: "absolute",
					top: "7%",
					left: "7%",
					width: "350px",
					padding: "10px",
					backgroundColor: "rgba(0, 0, 0, 0.5)",
					borderRadius: "10px",
					color: "white",
					fontSize: "1.2rem",
					textAlign: "center",
				}}
			>
				{/* HPテキスト */}
				<div style={{ marginBottom: "5px" }}>HP: {userData.aState.hp} / {userData.aState.maxHp}</div>

				{/* HPゲージ */}
				<div
					style={{
						width: "100%",
						height: "20px",
						backgroundColor: "#444",
						borderRadius: "5px",
						overflow: "hidden",
						border: "1px solid #fff",
					}}
				>
					<div
						style={{
							width: `${userData.aState.hp}%`, // HPの割合に応じてバーの長さを調整
							height: "100%",
							backgroundColor: "#FFFF00",
							transition: "width 0.3s ease-in-out",
						}}
					></div>
				</div>
			</div>

			{/* 中央の大きな赤い文字 A */}
			<div
				style={{
					fontSize: "11rem", // 大きな文字サイズ
					color: userData.aState.color, // 赤色（Hexコード）
					fontWeight: userData.aState.isBold ? "bold" : "normal", // isBoldを反映
                    fontFamily: userData.aState.font || "inherit", // fontが設定されていれば適用
					position: "absolute",
					zIndex: 1, // 他の要素より前に表示
					textShadow: userData.aState.isOutlined ? "4px 4px 8px rgba(0, 0, 0, 0.7)" : "none", // 文字に影を追加
				}}
			>
				A
			</div>

			{/* タイトル */}
			<div
				style={{
					fontSize: "3rem",
					color: "#FFFF00", // 黄色（Hexコード）
					fontWeight: "bold",
					position: "absolute",
					top: "20%",
					zIndex: 1,
					textShadow: "4px 4px 8px rgba(0, 0, 0, 0.7)", // 文字に影を追加
				}}
			>
				えいのお部屋
			</div>

			{/* 栄養ボタン */}
			<div
				style={{
					display: "flex",
					justifyContent: "center",
					gap: "1rem",
					position: "absolute",
					bottom: "10%",
				}}
			>
				<Link to="/custom">
					<button
						style={{
							display: "flex",
							alignItems: "center",
							padding: "20px 40px",
							fontSize: "1.5rem",
							backgroundColor: "#E0E0E0",
							borderRadius: "8px",
							border: "2px solid #BDBDBD",
							boxShadow: "0 4px 6px rgba(0, 0, 0, 0.1)",
							cursor: "pointer",
							transition: "all 0.3s ease",
						}}
            onMouseOver={(e) => {
              (e.target as HTMLButtonElement).style.backgroundColor = "#BDBDBD";
            }}
            onMouseOut={(e) => {
              (e.target as HTMLButtonElement).style.backgroundColor = "#E0E0E0";
            }}

					>
						<img
							src="https://icooon-mono.com/i/icon_13136/icon_131361_64.png"
							alt="wearing"
							style={{ width: "40px", marginRight: "12px" }}
						/>
						影変
					</button>
				</Link>

				<Link to="/battle">
					<button
						style={{
							display: "flex",
							alignItems: "center",
							padding: "20px 40px",
							fontSize: "1.5rem",
							backgroundColor: "#E0E0E0",
							borderRadius: "8px",
							border: "2px solid #BDBDBD",
							boxShadow: "0 4px 6px rgba(0, 0, 0, 0.1)",
							cursor: "pointer",
							transition: "all 0.3s ease",
						}}
            onMouseOver={(e) => {
              (e.target as HTMLButtonElement).style.backgroundColor = "#BDBDBD";
            }}
            onMouseOut={(e) => {
              (e.target as HTMLButtonElement).style.backgroundColor = "#E0E0E0";
            }}
					>
						<img
							src="https://icooon-mono.com/i/icon_12209/icon_122091_64.png"
							alt="battle"
							style={{ width: "40px", marginRight: "12px" }}
						/>
						英雄
					</button>
				</Link>

        <Link to="/fishing">
          <button
            style={{
              display: "flex",
              alignItems: "center",
              padding: "20px 40px",
              fontSize: "1.5rem",
              backgroundColor: "#E0E0E0",
              borderRadius: "8px",
              border: "2px solid #BDBDBD",
              boxShadow: "0 4px 6px rgba(0, 0, 0, 0.1)",
              cursor: "pointer",
              transition: "all 0.3s ease",
            }}
            onMouseOver={(e) => {
              (e.target as HTMLButtonElement).style.backgroundColor = "#BDBDBD";
            }}
            onMouseOut={(e) => {
              (e.target as HTMLButtonElement).style.backgroundColor = "#E0E0E0";
            }}
          >
            <img
              src="https://icooon-mono.com/i/icon_15011/icon_150111_64.png"
              alt="fishing"
              style={{ width: "40px", marginRight: "12px" }}
            />
            永釣
          </button>
        </Link>

        <button
          onClick={toggleNutrition}
          style={{
            display: "flex",
            alignItems: "center",
            padding: "20px 40px",
            fontSize: "1.5rem",
            backgroundColor: "#E0E0E0",
            borderRadius: "8px",
            border: "2px solid #BDBDBD",
            boxShadow: "0 4px 6px rgba(0, 0, 0, 0.1)",
            cursor: "pointer",
            transition: "all 0.3s ease",
          }}
            onMouseOver={(e) => {
              (e.target as HTMLButtonElement).style.backgroundColor = "#BDBDBD";
            }}
            onMouseOut={(e) => {
              (e.target as HTMLButtonElement).style.backgroundColor = "#E0E0E0";
            }}
        >
          <img
            src="https://icooon-mono.com/i/icon_10071/icon_100711_64.png"
            alt="training"
            style={{ width: "40px", marginRight: "12px" }}
          />
          栄養
        </button>
      </div>

      {/* 栄養ボタンで表示されるアルファベットのリスト */}
      {isNutritionVisible && (
        <div
          style={{
            fontSize: "2rem",
            color: "black",
            position: "absolute",
            top: "15%", // 栄養ボタンの上に配置
            left: "calc(55% + 7rem)", // Aの隣に表示
            zIndex: 1,
            textAlign: "left",
            width: "30%",
            backgroundColor: "rgba(255, 255, 255, 0.7)",
            padding: "7px",
            borderRadius: "15px",
            boxShadow: "0 4px 6px rgba(0, 0, 0, 0.3)",
          }}
        >
          <h2>獲得したアルファベット:</h2>
          <div>
            {currentLetters.map((letter, index) => (
              <div
                key={index}
                style={{
                  cursor: "pointer",
                  padding: "5px",
                  border: "1px solid black",
                  marginBottom: "5px",
                  backgroundColor: "lightgray",
                  borderRadius: "5px",
                  textAlign: "center",
                }}
                onClick={() => handleLetterClick(letter)} // クリックでHP回復
              >
                {letter}
              </div>
            ))}
          </div>

          {/* ページネーション */}
          <div style={{ marginTop: "10px", textAlign: "center" }}>
            <button onClick={() => goToPage(currentPage - 1)} disabled={currentPage === 1}>
              前
            </button>
            <span>{`${currentPage} / ${totalPages}`}</span>
            <button onClick={() => goToPage(currentPage + 1)} disabled={currentPage === totalPages}>
              次
            </button>
          </div>
        </div>
      )}
    </div>
  );

}
