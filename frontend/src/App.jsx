import { useState } from 'react';
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

function App() {
  const [buttonStates, setButtonStates] = useState(Array(8).fill(false));
  const navigate = useNavigate();

  const toggleIcon = index => {
    const newButtonStates = [...buttonStates];
    newButtonStates[index] = !newButtonStates[index];
    setButtonStates(newButtonStates);
  };

  const verify = () => {
    if (buttonStates.some(state => state === true)) {
      navigate('/result', { state: { buttonStates: buttonStates } });
    } else {
      window.alert('少なくとも1つの画像を選択してください。');
    }
  };

  return (
    <div className="desktop">
      <div className="frame">
        {buttonStates.map((isVisible, index) => (
          <div className="box" key={index}>
            <button className="rectangle-button" onClick={() => toggleIcon(index)}>
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
          </div>
        ))}
      </div>
      <div className="title">
        <div className="model-s">犬に見えるのはどれ？</div>
      </div>
      <button className="input-field-button" onClick={verify}>
        <div className="input-field">
          <div className="text-wrapper">VERIFY</div>
        </div>
      </button>
    </div>
  );
}

export default App;
