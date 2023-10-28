import React, { useState } from 'react';
import './App.css';
import "./style.css";
import icon from './icon-check.png';

function App() {
  const [buttonStates, setButtonStates] = useState(
    Array(8).fill(false)  // 8つのボタンに対する状態を保持する配列。初期状態は全て非表示（false）。
  );

  const toggleIcon = index => {
    const newButtonStates = [...buttonStates];
    newButtonStates[index] = !newButtonStates[index];  // 状態を反転させる
    setButtonStates(newButtonStates);
  };

  return (
    <div className="desktop">
      <div className="frame">
        {buttonStates.map((isVisible, index) => (
          <div className="box" key={index}>
            <button className="rectangle-button" onClick={() => toggleIcon(index)}>
              <div className="rectangle">
                {isVisible && <img className="icon-check" alt="Icon check" src={icon} />}
              </div>
            </button>
          </div>
        ))}
      </div>
      {/* <div className="frame1"> */ }
  {/* </div> */ }
      <div className="title">
        <div className="model-s">パンダに見えるのはどれ？</div>
      </div>
      <div className="div" />
      <button className="input-field-button">
        <div className="input-field">
          <div className="text-wrapper">VERIFY</div>
        </div>
      </button>
    </div>
  );
}

export default App;
