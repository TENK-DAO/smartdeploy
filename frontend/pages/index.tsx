import Head from 'next/head'
import Image from 'next/image'
import { Inter } from 'next/font/google'
import { useState, Dispatch, SetStateAction } from 'react'
import styles from '@/styles/Home.module.css'
import WalletInfo from '@/components/wallet'
import PublishedTab from '@/components/published-tab'
import DeployedTab from '@/components/deployed-tab'
import PopupDappInfo from '@/components/dapp-info-popup'
import { FaDiscord, FaTwitter, FaGithub } from "react-icons/fa";
import { Contract, networks } from 'smartdeploy-client';

const inter = Inter({ subsets: ['latin'] })

// Smartdeploy Contract Instance
export const smartdeploy = new Contract({
  ...networks.futurenet,
  rpcUrl: 'https://rpc-futurenet.stellar.org:443',
});

export type UserWalletInfo = {
  connected: boolean;
  setConnected: Dispatch<SetStateAction<boolean>>;
  hasFreighter: boolean;
  setHasFreighter: Dispatch<SetStateAction<boolean>>;
  address: string;
  setAddress: Dispatch<SetStateAction<string>>;
  network: string;
  setNetwork: Dispatch<SetStateAction<string>>;
}

export type UserWalletInfoProps = {
  data: UserWalletInfo;
}

export default function Home() {

  // State variables from Freighter Wallet
  const [connected, setConnected] = useState<boolean>(false);
  const [hasFreighter, setHasFreighter] = useState<boolean>(true);
  const [address, setAddress] = useState<string>("");
  const [network, setNetwork] = useState<string>("");

  const userWalletInfo: UserWalletInfo = {
    connected: connected,
    setConnected: setConnected,
    hasFreighter: hasFreighter,
    setHasFreighter: setHasFreighter,
    address: address,
    setAddress: setAddress,
    network: network,
    setNetwork: setNetwork,
  }

  return (
    <>
      <Head>
        <title>SmartDeploy Dapp</title>
        <meta name="description" content="A framework for publishing, deploying, and upgrading Soroban smart contracts." />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/sd-logo-det.ico" />
      </Head>

      

      <div className={styles.headerBar}>
        <div className={styles.container}>
          <div className={styles.social}>
            <a
              href="https://www.smartdeploy.dev/"
              rel="noopener noreferrer"
            >
              <Image
                className={styles.socialItem}
                src="/sd-logo-written-header.svg"
                alt="SmartDeploy Logo"
                width={127}
                height={26}
                priority
              />
            </a>
            <a
              href="https://github.com/TENK-DAO/smartdeploy"
              target="_blank"
              rel="noopener noreferrer"
            >
              <FaGithub className={styles.socialItem} style={{ fill: 'rgb(161, 161, 163)' }}/>
            </a>
            <a
              href="https://discord.com/invite/6fKqnSfr"
              target="_blank"
              rel="noopener noreferrer"
            >
              <FaDiscord className={styles.socialItem} style={{ fill: 'rgb(161, 161, 163)' }}/>
            </a>
            <a
              href="https://twitter.com/TENKDAO"
              target="_blank"
              rel="noopener noreferrer"
            >
              <FaTwitter className={styles.socialItem} style={{ fill: 'rgb(161, 161, 163)' }}/>
            </a>
          </div>
          <WalletInfo data={userWalletInfo}/>
        </div>
      </div>

      <main className={`${styles.main} ${inter.className}`}>
        
        <div className={styles.center}>
          <Image
            className={styles.logo}
            src="/sd-logo-written.svg"
            alt="SmartDeploy Logo"
            width={340}
            height={70}
            priority
          />
          <p className={styles.smartdeployMessage}>A framework for publishing, deploying, invoking and upgrading Soroban smart contracts</p>
        </div>

        <PopupDappInfo/>
        <PublishedTab data={userWalletInfo}/>
        <DeployedTab/>
        
        <div className={styles.grid}>
          <a
            href="https://nextjs.org/docs?utm_source=create-next-app&utm_medium=default-template&utm_campaign=create-next-app"
            className={styles.card}
            target="_blank"
            rel="noopener noreferrer"
          >
            <h2>
              Docs <span>-&gt;</span>
            </h2>
            <p>
              Find in-depth information about Next.js features and&nbsp;API.
            </p>
          </a>

          <a
            href="https://nextjs.org/learn?utm_source=create-next-app&utm_medium=default-template&utm_campaign=create-next-app"
            className={styles.card}
            target="_blank"
            rel="noopener noreferrer"
          >
            <h2>
              Learn <span>-&gt;</span>
            </h2>
            <p>
              Learn about Next.js in an interactive course with&nbsp;quizzes!
            </p>
          </a>

          <a
            href="https://vercel.com/new?utm_source=create-next-app&utm_medium=default-template&utm_campaign=create-next-app"
            className={styles.card}
            target="_blank"
            rel="noopener noreferrer"
          >
            <h2>
              Deploy <span>-&gt;</span>
            </h2>
            <p>
              Instantly deploy your Next.js site to a shareable URL
              with&nbsp;Vercel.
            </p>
          </a>
        </div>        

        

      </main>

      <div className={styles.footer}>
        <div className={styles.left}>
          <p>© {new Date().getFullYear()} SmartDeploy. All rights reserved.</p>
        </div>
        <div className={styles.tenkLogo}>
          <a
              href="https://tenk.app/"
              target="_blank"
              rel="noopener noreferrer"
            >
              By{' '}
              <Image
                src="/TENK_logo-det.svg"
                alt="Tenk Logo"
                width={110}
                height={26}
                priority
              />
            </a>
        </div>
        <div className={styles.right}>
          <a
            href="https://smartdeploy.dev/privacy"
            target='_blank'
            >
              <p>Privacy Policy</p>
          </a>
          <a
            href="https://smartdeploy.dev/terms"
            target='_blank'
            >
              <p>Terms Of Use</p>
          </a>
          <a
            href="https://smartdeploy.dev/contact"
            target='_blank'
            >
              <p>Contact Us</p>
          </a>
        </div>
      </div>

    </>
  )
}
