interface FinishFishBtnProps {
    onClick: () => void;
}

export default function FinishFishBtn({ onClick }: FinishFishBtnProps) {
    return (
        <button
            onClick={onClick}
            style={{
                position: 'fixed', // 画面上の固定位置に配置
                top: '20px', // 上からの距離
                left: '20px', // 左からの距離
                padding: '10px 20px', // ボタンのパディング
                fontSize: '16px', // フォントサイズ
                backgroundColor: '#e74c3c', // 背景色
                color: 'white', // 文字色
                border: 'none', // ボーダーをなくす
                borderRadius: '5px', // 角を丸くする
                cursor: 'pointer', // カーソルをポインターにする
                zIndex: 1000, // 他の要素より前面に表示
            }}
        >
            釣りを終了
        </button>
    );
}
