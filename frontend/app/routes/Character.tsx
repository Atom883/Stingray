import React from 'react';

interface CharacterProps {
    name: string;
    hp: string;
    className?: string;
}

const Character: React.FC<CharacterProps> = ({ name, hp, className }) => {
    return (
      <div className={`character-wrapper ${className}`}> 
        <div className="big-character-name">{name}</div> 
        <div className={`character-container`}>
          <div className="character-status"> 
            <div className="hp-container">
              <div className="hp-bar">
                <div className="hp-fill"></div>
              </div>
              <div className="hp-text">{hp}</div>
            </div>
            <div className="status-icons">
              <button className="status-icon red">攻撃</button>
              <button className="status-icon blue">防御</button>
            </div>
          </div>
        </div>
      </div>
    );
  };

export default Character;
