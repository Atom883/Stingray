import React from 'react';

interface JoinMatchBtnProps {
    onMatchButtonClick: () => void;
}

export default function JoinMatchBtn({ onMatchButtonClick }: JoinMatchBtnProps) {
    return (
        <div style={{
            display: 'flex',
            justifyContent: 'center',
            alignItems: 'center',
            height: '100vh', // 画面の高さいっぱいにする場合
        }}>
            <button
            onClick={onMatchButtonClick} 
            style={{
                padding: '10px 20px',
                fontSize: '16px',
                cursor: 'pointer',
                zIndex: 5,
                backgroundColor: 'rgb(0, 6, 190)',
                color: 'white',
                transition: 'background-color 0.3s',
            }} onMouseEnter={(e) => e.currentTarget.style.backgroundColor = 'rgb(0, 0, 122)'}
               onMouseLeave={(e) => e.currentTarget.style.backgroundColor = 'rgb(0, 6, 190)'}>
                対戦相手を探す
            </button>
        </div>
    );
}
