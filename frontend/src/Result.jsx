import { useState, useEffect } from 'react';
import { useLocation } from 'react-router-dom';
import { useNavigate } from 'react-router-dom';
import './App.css';
import "./style.css";
import icon from './icon-check.png';
import dog1 from './pictures/dog1.png';
import dog2 from './pictures/dog2.png';
import dog3 from './pictures/dog3.png';
import dog4 from './pictures/dog4.png';
import dog5 from './pictures/dog5.png';
import dog6 from './pictures/dog6.jpg';
import dog7 from './pictures/dog7.png';
import dog8 from './pictures/dog8.png';

function Result() {
  const location = useLocation();
  const [buttonStates, setButtonStates] = useState(location.state.buttonStates || Array(8).fill(false));
  const navigate = useNavigate();

  // 正しい答えを示す配列。これは仮のもので、実際にはこの値を設定する必要があります。
  const correctAnswers = [false, false, false, false, true, true, false, true];

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
              {index === 0 && <img className="picture" src={dog1} />}
                {index === 1 && <img className="picture" src={dog2} />}
                {index === 2 && <img className="picture" src={dog3} />}
                {index === 3 && <img className="picture" src={dog4} />}
                {index === 4 && <img className="picture" src={dog5} />}
                {index === 5 && <img className="picture" src={dog6} />}
                {index === 6 && <img className="picture" src={dog7} />}
                {index === 7 && <img className="picture" src={dog8} />}
                {isVisible && <img className="icon-check" alt="Icon check" src={icon} />}
              </div>
            </button>
            <div className="ling"
              style={{ border: correctAnswers[index] ? '1.5px solid var(--Gradient-8, #2FB8FF)' : 'none' }}
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
