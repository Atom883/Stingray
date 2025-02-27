import React, { useEffect, useState } from 'react';
import '../css/battle.css';
import Character from './Character'; // Characterコンポーネントをインポート


export default function Battle() {
  const [socket,setSocket] = useState<WebSocket | null>(null);

  useEffect(() => {
    const ws = new WebSocket('ws://localhost:8080');
    ws.onopen = () => {
      console.log('接続しました！');
    };
    // ws.onclose = () => {
    // console.log('接続を解除しました！');
    // };
    ws.onmessage = (event) => {
      console.log('Message from server:', event.data);
    };

    return() =>{
      if(ws){
        ws.close();
      }
    };
  },[])
  


  return (
    <div className="battle-container">
      <div className="background">
        {/* 背景画像（CSSで設定） */}
      </div>
      <Character name="A" hp="HP 36/36" className="left" />
      <div className="vs">VS</div>
      <Character name="A" hp="HP 36/36" className="right" />
    </div>
  );
}
