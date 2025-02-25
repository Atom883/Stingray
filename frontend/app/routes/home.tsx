import type { Route } from "./+types/home";
import { Welcome } from "../welcome/welcome";

export function meta(_: Route.MetaArgs) {
	return [
		{ title: "New React Router App" },
		{ name: "description", content: "Welcome to React Router!" },
	];
}

import { Link } from "react-router-dom"; // ← 追加

export default function Home() {
	return (
		<div>
			<Welcome />
			<div
				style={{
				display: "flex",
				justifyContent: "center", // 水平方向に中央揃え
				gap: "1rem",              // ボタン同士の間隔
				marginTop: "20px",
				}}>

				{/* 戦いボタン → /battle */}
				<Link to="/battle">
				<button>戦い</button>
				</Link>

				{/* 釣りボタン → /fishing */}
				<Link to="/fishing">
				<button>釣り</button>
				</Link>

				{/* ごはんボタン → /training */}
				<Link to="/training">
				<button>ごはん</button>
				</Link>
			</div>
		</div>
	);
}
