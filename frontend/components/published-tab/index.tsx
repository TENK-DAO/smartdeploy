import { BsSendPlus } from 'react-icons/bs';
import Popup from 'reactjs-popup';
import styles from './style.module.css';

import { smartdeploy, UserWalletInfoProps, UserWalletInfo } from "@/pages";
import { isConnected } from '@stellar/freighter-api';
import { Ok, Err, Option, Version } from 'smartdeploy-client'
import { useAsync } from "react-async";
import { useState, ChangeEvent, Dispatch, SetStateAction } from 'react';

interface PublishedContract {
    index: number;
    name: string;
    author: string;
    version: Version;
    version_string: string;
    hash: string;
}

type DeployIconComponentProps = {
    userWalletInfo: UserWalletInfoProps;
    contract_name: string;
    version: Option<Version>;
    version_string: string;
}

async function listAllPublishedContracts() {

    return await smartdeploy
                            .listPublishedContracts({start: undefined, limit: undefined})
                            .then((response) => {

                                if (response instanceof Ok) {

                                    let publishedContracts: PublishedContract[] = [];

                                    const contractArray =  response.unwrap();

                                    contractArray.forEach(([name, publishedContract], i) => {
                                        
                                        const version: Version = publishedContract.versions.keys().next().value;
                                        const major = version.major;
                                        const minor = version.minor;
                                        const patch = version.patch;
                                        const versionString = `v.${major}.${minor}.${patch}`;

                                        const hash = publishedContract.versions.values().next().value.hash.join('');
                                        const parsedPublishedContract: PublishedContract = {
                                            index: i,
                                            name: name,
                                            author: publishedContract.author.toString(),
                                            version: version,
                                            version_string: versionString,
                                            hash: hash
                                        }

                                        publishedContracts.push(parsedPublishedContract);
                                    });
                                    
                                    //console.log(publishedContracts);
                                    return publishedContracts;

                                } else if (response instanceof Err) {
                                    response.unwrap();
                                } else {
                                    throw new Error("listPublishedContracts returns undefined. Impossible to fetch the published contracts.");
                                }
                            });

}

//{contract_name, version, deployed_name, owner, salt}: 
//{contract_name: string, version: Option<Version>, deployed_name: string, owner: string, salt: Option<Buffer>}
async function deploy(userWalletInfo: UserWalletInfo, deployed_name: string, setIsDeploying: Dispatch<SetStateAction<boolean>>, setDeployedName: Dispatch<SetStateAction<string>>) {
    
    // Check if the user has Freighter
    if (!(await isConnected())) {
        window.alert("Impossible to interact with Soroban: you don't have Freighter extension.\n You can install the extension here: https://www.freighter.app/");
        setIsDeploying(false);
    }
    else {
        // Check if the Wallet is connected
        if (userWalletInfo.address === "") {
            alert("Wallet not connected. Please, connect a Stellar account.");
            setIsDeploying(false);
        }
        // Check is the network is Futurenet
        else if (userWalletInfo.network !== "FUTURENET") {
            alert("Wrong Network. Please, switch to Future Net.");
            setIsDeploying(false);
        }
        else {
            // Check if deployed name is empty
            if (deployed_name === "") {
                alert("Deployed name cannot be empty. Please, choose a deployed name.");
                setIsDeploying(false);
            }
            // Check if deployed name contains spaces
            else if (deployed_name.includes(' ')) {
                alert("Deployed name cannot includes spaces. Please, remove the spaces.");
                setIsDeploying(false);
            }
            // Now that everything is ok, deploy the contract
            else {
                setDeployedName("");
                setIsDeploying(false);
            }
        }
    }
}

function DeployIconComponent(props: DeployIconComponentProps) {

    const [wouldDeploy, setWouldDeploy]   = useState<boolean>(false); 
    const [deployedName, setDeployedName] = useState<string>("");
    const [isDeploying, setIsDeploying]   = useState<boolean>(false);

    const handleInputChange = (e: ChangeEvent<HTMLInputElement>) => {
        setDeployedName(e.target.value);
    }
  
    return (
        <>
            {!wouldDeploy ? (
                <td className={styles.deployIconCell}>
                    <BsSendPlus
                        className={styles.deployIcon}
                        onClick={() => setWouldDeploy(true) }
                    />
                </td>
            ) : (
                <>
                    <td className={styles.deployIconCell}>
                        <p className={styles.deployingMessage}>Deploying...</p>
                    </td>
                    <Popup  open={wouldDeploy} closeOnDocumentClick={false}>
                        <div className={styles.popupContainer}>
                            <button className={styles.close} onClick={() => setWouldDeploy(false)}>
                                &times;
                            </button>
                            <div className={styles.header}>Deploy <span className={styles.nameColor}>{props.contract_name} ({props.version_string})</span> </div>
                            <div className={styles.content}>
                                <p className={styles.mainMessage}><b>You are about to create an instance of <span className={styles.nameColor}>{props.contract_name}</span> published contract where you will be the owner.</b><br/></p>
                                <div className={styles.deployedNameDiv}>
                                    <b>Please choose a contract instance name:</b>
                                    <input 
                                        className={styles.deployedNameInput} 
                                        type="text" 
                                        spellCheck={false} 
                                        placeholder="deployed_name" 
                                        value={deployedName}
                                        onChange={handleInputChange}>
                                    </input>
                                </div>
                            </div>
                            <div className={styles.buttonContainer}>
                                {!isDeploying ? (
                                    <>
                                        <button className={styles.button} 
                                                onClick={() => {
                                                    setIsDeploying(true);
                                                    deploy(props.userWalletInfo.data, deployedName, setIsDeploying, setDeployedName);
                                                }}
                                        >
                                            Deploy
                                        </button>
                                        <button className={styles.button} onClick={() => setWouldDeploy(false)}>
                                            Cancel
                                        </button>
                                    </>
                                ) : (
                                    <button className={styles.buttonWhenDeploying}>
                                        Deploying...
                                    </button>
                                )}
                            </div>
                        </div>
                    </Popup>
                </>
            )}
        </>
    );
}


export default function PublishedTab(props: UserWalletInfoProps) {

    const { data, error, isPending } = useAsync({ promiseFn: listAllPublishedContracts});
    
    if (isPending) return (<p className={styles.load}>Loading...</p>);

    else if (error) { throw new Error("Error when trying to fetch Published Contracts");}

    else if (data) {

        const rows: JSX.Element[] = [];

        data.forEach((publishedContract) => {
            rows.push(
                <tr key={publishedContract.index}>
                    <td className={styles.contractCell}>{publishedContract.name}</td>
                    <td>{publishedContract.author}</td>
                    <td>{publishedContract.version_string}</td>
                    <DeployIconComponent
                        userWalletInfo={props}
                        contract_name={publishedContract.name}
                        version={publishedContract.version}
                        version_string={publishedContract.version_string}
                    />
                </tr>
            );
        });
        
        return(
            <div className={styles.publishedTabContainer}>
                <table className={styles.publishedTabHead}>
                    <caption>PUBLISHED CONTRACTS</caption>
                    <colgroup>
                        <col className={styles.contractCol}></col>
                        <col className={styles.authorCol}></col>
                        <col className={styles.versionCol}></col>
                        <col className={styles.deployCol}></col>
                    </colgroup>
                    <thead>
                        <tr>
                            <th>Contract</th>
                            <th>Author</th>
                            <th>Version</th>
                            <th>Deploy</th>
                        </tr>
                    </thead>
                </table>
                <div className={styles.publishedTabContentContainer}>
                    <table className={styles.publishedTabContent}>
                        <colgroup>
                            <col className={styles.contractCol}></col>
                            <col className={styles.authorCol}></col>
                            <col className={styles.versionCol}></col>
                            <col className={styles.deployCol}></col>
                        </colgroup>
                        <tbody>
                            {rows}
                        </tbody>
                    </table>
                </div>
            </div>
        )
    }
    
}