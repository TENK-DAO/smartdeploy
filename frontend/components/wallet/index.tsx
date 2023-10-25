import { Inter } from 'next/font/google'
import { useEffect, useState } from 'react';
import { isAllowed, setAllowed, getUserInfo, getPublicKey, isConnected, getNetwork } from '@stellar/freighter-api';
import { UserWalletInfoProps } from '@/pages';
import styles from './style.module.css';

const inter = Inter({ subsets: ['latin'] })

export default function WalletInfo(props: UserWalletInfoProps) {

    const [showWallet, setShowWallet] = useState(false);

    async function connect() {
        const freighterConnected = await isConnected();
        if (!freighterConnected) {
            props.data.setHasFreighter(false);
        }
        else {
            await setAllowed();
            if (await isAllowed()) {
                const publicKey = await getPublicKey();
                const network   = await getNetwork();
                props.data.setAddress(publicKey);
                props.data.setNetwork(network);
                props.data.setConnected(true);
            }
        }
    }

    async function stayConnected() {
        if (await isAllowed()) {
          const publicKey = await getUserInfo();
          const network   = await getNetwork();
          props.data.setAddress(publicKey.publicKey);
          props.data.setNetwork(network);
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
                {!props.data.address && props.data.hasFreighter ? (
                    <button className={styles.connectButton} onClick={() => connect()}><b>Connect Wallet</b></button>
                ) : !props.data.hasFreighter ? (
                    <p>You don't have <a href="https://www.freighter.app/" target="_blank">Freighter extension</a></p>
                ) : (
                    <>
                        <div className={styles.card}>{props.data.network}</div>
                        <div className={styles.card}>{props.data.address.substring(0, 4) + "..." + props.data.address.slice(-4)}</div>
                    </>
                )}
                </>
            )}
        </div>
    )
}