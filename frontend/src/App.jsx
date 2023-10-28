import './App.css';
import "./style.css";
import icon from './icon-check.png';

function App() {
  return (
    <div className="desktop">
      <div className="frame">
        <div className="box">
          <button className="rectangle-button">
            <div className="rectangle">
            <img className="icon-check" alt="Icon check" src={icon} /></div>
          </button>
        </div>
        <div className="box">
          <button className="rectangle-button">
            <div className="rectangle">
            <img className="icon-check" alt="Icon check" src={icon} /></div>
          </button>
        </div>
        <div className="box">
          <button className="rectangle-button">
            <div className="rectangle">
            <img className="icon-check" alt="Icon check" src={icon} /></div>
          </button>
        </div>
        <div className="box">
          <button className="rectangle-button">
            <div className="rectangle">
            <img className="icon-check" alt="Icon check" src={icon} /></div>
          </button>
        </div>
        <div className="box">
          <button className="rectangle-button">
            <div className="rectangle">
            <img className="icon-check" alt="Icon check" src={icon} /></div>
          </button>
        </div>
        <div className="box">
          <button className="rectangle-button">
            <div className="rectangle">
            <img className="icon-check" alt="Icon check" src={icon} /></div>
          </button>
        </div>
        <div className="box">
          <button className="rectangle-button">
            <div className="rectangle">
            <img className="icon-check" alt="Icon check" src={icon} /></div>
          </button>
        </div>
        <div className="box">
          <button className="rectangle-button">
            <div className="rectangle">
            <img className="icon-check" alt="Icon check" src={icon} /></div>
          </button>
        </div>
      </div>
      {/* <div className="frame1"> */}
        {/* </div> */}
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
