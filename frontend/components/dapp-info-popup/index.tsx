import Popup from 'reactjs-popup';
import styles from './style.module.css';
import { useState, useEffect } from 'react';

export default function PopupDappInfo() {

  const [isOpen, setIsOpen] = useState(false);

  useEffect(() => {
    setIsOpen(true);
  }, []);

  return (
    <Popup  open={isOpen} position="bottom right">
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
            <input className={styles.checkbox} type="checkbox" id="maCaseACocher"></input>
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

/*

{(close: any) => (
          <div className={styles.modal}>
            <button className={styles.close} onClick={close}>
              &times;
            </button>
            <div className={styles.header}> Modal Title </div>
            <div className={styles.content}>
              {' '}
              Lorem ipsum dolor sit amet consectetur adipisicing elit. Atque, a nostrum.
              Dolorem, repellat quidem ut, minima sint vel eveniet quibusdam voluptates
              delectus doloremque, explicabo tempore dicta adipisci fugit amet dignissimos?
              <br />
              Lorem ipsum dolor sit amet, consectetur adipisicing elit. Consequatur sit
              commodi beatae optio voluptatum sed eius cumque, delectus saepe repudiandae
              explicabo nemo nam libero ad, doloribus, voluptas rem alias. Vitae?
            </div>
            <div className={styles.actions}>
              <button
                className="button"
                onClick={() => {
                  console.log('modal closed ');
                  close();
                }}
              >
                Understood
              </button>
            </div>
          </div>
        )}

*/