import { Inter } from 'next/font/google'
import { useEffect, useState } from 'react';
import { isAllowed, setAllowed, getUserInfo, getPublicKey, isConnected, getNetwork } from '@stellar/freighter-api';
import styles from './style.module.css';

const inter = Inter({ subsets: ['latin'] })

export default function WalletInfo() {

    const [showWallet, setShowWallet] = useState(false);
    const [connected, setConnected] = useState(false);
    const [hasFreighter, setHasFreighter] = useState(true);
    const [address, setAddress] = useState("");
    const [network, setNetwork] = useState("");

    async function connect() {
        const freighterConnected = await isConnected();
        if (!freighterConnected) {
            setHasFreighter(false);
        }
        else {
            await setAllowed();
            if (await isAllowed()) {
                const publicKey = await getPublicKey();
                const network   = await getNetwork();
                setAddress(publicKey);
                setNetwork(network);
                setConnected(true);
            }
        }
    }

    async function stayConnected() {
        if (await isAllowed()) {
          const publicKey = await getUserInfo();
          const network   = await getNetwork();
          setAddress(publicKey.publicKey);
          setNetwork(network);
        }
    }

    useEffect(() => {
        const timer = setTimeout(() => {
          setShowWallet(true);
        }, 1000);
        stayConnected();
        return () => clearTimeout(timer);
    }, []);


    return (
        <div className={`${styles.walletInfo} ${inter.className}`}>
            {showWallet && (
                <>
                {!address && hasFreighter ? (
                    <button className={styles.connectButton} onClick={() => connect()}><b>Connect Wallet</b></button>
                ) : !hasFreighter ? (
                    <p>You don't have <a href="https://www.freighter.app/" target="_blank">Freighter extension</a></p>
                ) : (
                    <>
                        <div className={styles.card}>{network}</div>
                        <div className={styles.card}>{address.substring(0, 4) + "..." + address.slice(-4)}</div>
                    </>
                )}
                </>
            )}
        </div>
    )
}