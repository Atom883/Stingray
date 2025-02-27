import React, { use, useEffect, useState } from 'react';
import '../css/battle.css';
import JoinMatchBtn from '../components/JoinMatchBtn';
import { useNavigate } from 'react-router';


export default function BattleLobby() {
  const [socket,setSocket] = useState<WebSocket | null>(null);
  const [isMatching, setIsMatching] = useState(false); //待機中のフラグ
  const [isBattleStarted, setIsBattleStarted] = useState(false); //対戦中のフラグ
  const [myHp, setMyHp] = useState(100);
  const [opponentHp, setOpponentHp] = useState(100);
  const [message, setMessage] = useState(''); //状況に応じたメッセージを表示する
  const navigate = useNavigate();


  useEffect(() => {
  const ws = new WebSocket('ws://localhost:3000/ws');

  // ws.onopen = () => {
  //     console.log('websocketに接続しました！');
  //     setSocket(ws);
  //   ws.send('Match');
  // };


    // ws.onmessage = (event) => {
    //   const data = JSON.parse(event.data);
    //   console.log('サーバーからのメッセージ:',data);

    //   switch(data.type){
    //     case 'match_complete':
    //       setMessage('対戦相手が見つかりました！戦闘開始！');
    //       setIsMatching(false);
    //       setIsBattleStarted(true);
    //       break;
    //     case 'opponent_hp_update':
    //       settOpponentHp(data.hp);

    //   }
    // }

  },[])

  const handleMatchButtonClick = () => {
    setIsMatching(true); //待機中のフラグをtureに


    // ws.onmessage = (event) => {
    //   const data = JSON.parse(event.data);
    //   console.log("サーバーからのメッセージ",data)

    //   if(data.type === 'match_complete'){
    //     //マッチングが完了したら、対戦画面に遷移
    //     navigate('/battle');
    //   }
    // }

    // ws.onclose = () => {
    //   console.log('webSocket接続が閉じられました。')
    //   setSocket(null);
    //   setIsMatching(false); //待機中のフラグをfalseに
    // }

    // ws.onerror = (error) => {
    //   console.error('WebScketエラー',error);
    //   setSocket(null);
    //   setIsMatching(false); //待機中のフラグをfalseに
    // }

    // return () => {
    //   if(ws){
    //     ws.close();
    //   } 
    // }
  }

  return (
    <div className="battle-container">
      <div className="background">
      </div>
      {isMatching? (
        <div>対戦相手を待っています...</div>
      ) : (
        <JoinMatchBtn onMatchButtonClick={handleMatchButtonClick}/>
      )}
    </div>
  );
}
