import { BsSendPlus } from 'react-icons/bs';
import Popup from 'reactjs-popup';
import styles from './style.module.css';

import { smartdeploy } from "@/pages";
import { Ok, Err, Option, Version } from 'smartdeploy-client'
import { useAsync } from "react-async";
import { useState, Dispatch, SetStateAction } from 'react';

interface PublishedContract {
    index: number;
    name: string;
    author: string;
    version: Version;
    version_string: string;
    hash: string;
}

type DeployIconComponentProps = {
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
async function deploy() {
}

function DeployIconComponent(props: DeployIconComponentProps) {

    const [wouldDeployed, setWouldDeploy] = useState<boolean>(false);
  
    return (
        <>
            {!wouldDeployed ? (
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
                    <Popup  open={wouldDeployed} closeOnDocumentClick={false}>
                        <div className={styles.popupContainer}>
                            <button className={styles.close} onClick={() => setWouldDeploy(false)}>
                                &times;
                            </button>
                            <div className={styles.header}>Deploy <span className={styles.nameColor}>{props.contract_name} ({props.version_string})</span> </div>
                            <div className={styles.content}>
                                <p className={styles.mainMessage}><b>You are about to create an instance of <span className={styles.nameColor}>{props.contract_name}</span> published contract where you will be the owner.</b><br/></p>
                                <div className={styles.deployedNameDiv}>
                                    <b>Please choose a contract instance name:</b>
                                    <input className={styles.deployedNameInput} type="text" spellCheck={false} placeholder="deployed_name"></input>
                                </div>
                            </div>
                            <div className={styles.buttonContainer}>
                                <button className={styles.button} onClick={() => { deploy() }}>
                                    Deploy
                                </button>
                                <button className={styles.button} onClick={() => setWouldDeploy(false)}>
                                    Cancel
                                </button>
                            </div>
                        </div>
                    </Popup>
            </>
            )}
        </>
    );
}


export default function PublishedTab() {

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