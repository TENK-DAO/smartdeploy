import Popup from 'reactjs-popup';
import styles from './style.module.css';
import { useState, useEffect, ChangeEvent } from 'react';

export default function PopupDappInfo() {

  const [isOpen, setIsOpen] = useState(false);

  const handleOnchange = (e: ChangeEvent<HTMLInputElement>) => {
    const hidePopUp = e.target.checked;
    localStorage.setItem("hidePopUp", JSON.stringify(hidePopUp));
  };

  const clearStorage = () => {
    localStorage.clear();
  };

  useEffect(() => {
    //clearStorage();
    const hidePopUp = localStorage.getItem("hidePopUp");
    if (hidePopUp === null) {
      setIsOpen(true);
    } else if (hidePopUp === "true") {
      setIsOpen(false);
    } else if (hidePopUp === "false") {
      setIsOpen(true);
    }
  }, []);

  return (
    <Popup  open={isOpen} closeOnDocumentClick={false} position="bottom right">
        <div className={styles.popupContainer}>
          <div className={styles.header}> Important Information </div>
          <div className={styles.content}>
            <p className={styles.mainMessage}><b>To fully use SmartDeploy, you need to connect your Freighter Wallet and select Future Net.</b><br/>
            Below are the steps to follow to interact with smart contracts:</p>
            <p>
            1. Get Freighter: Download the extension <a href="https://www.freighter.app/" target="_blank">here</a><br/>
            2. Enable Experimental Mode (Freighter Settings → Preferences, enable Experimental Mode)<br/>
            3. Select Future Net in the top right.
            </p>
            <input className={styles.checkbox} type="checkbox" onChange={handleOnchange}></input>
            <label className={styles.label}>Don't show again</label>
          </div>
          <div className={styles.buttonContainer}>
            <button
              className={styles.understood}
              onClick={() => {
                setIsOpen(false)
              }}
            >
              Understood
            </button>
          </div>
        </div>
    </Popup>
  )
}