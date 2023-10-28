import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import './App.css';
import "./style.css";
import icon from './icon-check.png';

function App() {
  const [buttonStates, setButtonStates] = useState(Array(8).fill(false));
  const navigate = useNavigate();

  const toggleIcon = index => {
    const newButtonStates = [...buttonStates];
    newButtonStates[index] = !newButtonStates[index];
    setButtonStates(newButtonStates);
  };

  const verify = () => {
    navigate('/result', { state: { buttonStates: buttonStates } });
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
      <div className="title">
        <div className="model-s">パンダに見えるのはどれ？</div>
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
