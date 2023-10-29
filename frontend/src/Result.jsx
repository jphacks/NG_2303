import { useState, useEffect } from 'react';
import { useLocation } from 'react-router-dom';
import { useNavigate } from 'react-router-dom';
import './App.css';
import "./style.css";
import icon from './icon-check.png';

function Result() {
  const location = useLocation();
  const [buttonStates, setButtonStates] = useState(location.state.buttonStates || Array(8).fill(false));
  const navigate = useNavigate();

  // 正しい答えを示す配列。これは仮のもので、実際にはこの値を設定する必要があります。
  const correctAnswers = [true, false, true, false, true, false, true, false];

  useEffect(() => {
    if (location.state && location.state.buttonStates) {
      setButtonStates(location.state.buttonStates);
    }
  }, [location.state]);

  const back = () => {
    navigate('/', { state: { buttonStates: buttonStates } });
  };

  // buttonStatesとcorrectAnswersが一致するかどうかをチェック
  const isMatching = JSON.stringify(buttonStates) === JSON.stringify(correctAnswers);

  return (
    <div className="desktop">
      <div className="frame">
        {buttonStates.map((isVisible, index) => (
          <div className="box" key={index}>
            <button className="rectangle-button">
              <div className="rectangle">
                {isVisible && <img className="icon-check" alt="Icon check" src={icon} />}
              </div>
            </button>
            <div className="ling" 
            style={{border: correctAnswers[index] ? '1.5px solid var(--Gradient-8, #2FB8FF)' : 'none'}} 
            />
          </div>
        ))}
      </div>
      <div className="title">
        <div className="model-s">{isMatching ? 'あなたはAIをだませた！' : 'あなたはAIをだませなかった！'}</div>
      </div>
      <button className="input-field-button" onClick={back}>
        <div className="input-field">
          <div className="text-wrapper">BACK</div>
        </div>
      </button>
    </div>
  );
}

export default Result;
