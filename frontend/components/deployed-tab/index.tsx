import { FaRegClipboard } from "react-icons/fa";
import { MdDone } from "react-icons/md"
import styles from './style.module.css';

import { smartdeploy } from "@/pages";
import { Ok, Err } from 'smartdeploy-client'
import { useAsync } from "react-async";
import { useState, useEffect, Dispatch, SetStateAction } from "react";

interface DeployedContract {
    index: number;
    name: string;
    address: string;
}

type ClipboardIconComponentProps = {
    address: string;
}


async function listAllDeployedContracts() {

    return await smartdeploy
                            .listDeployedContracts({start: undefined, limit: undefined})
                            .then((response) => {

                                if (response instanceof Ok) {

                                    let deployedContracts: DeployedContract[] = [];

                                    const contractArray =  response.unwrap();

                                    contractArray.forEach(([name, address], i) => {

                                        const parsedDeployedContract: DeployedContract = {
                                            index: i,
                                            name: name,
                                            address: address.toString(),
                                        }

                                        deployedContracts.push(parsedDeployedContract);
                                        
                                    });
                                    
                                    //console.log(deployedContracts);
                                    return deployedContracts;

                                } else if (response instanceof Err) {
                                    response.unwrap();
                                } else {
                                    throw new Error("listDeployedContracts returns undefined. Impossible to fetch the deployed contracts.");
                                }
                            });

}

async function copyAddr(setCopied: Dispatch<SetStateAction<boolean>> , addr: string) {
    await navigator.clipboard
                             .writeText(addr)
                             .then(() => {
                                setCopied(true);
                             })
                             .catch((err) => {
                                console.error("Failed to copy address: ", err);
                             });
}

function ClipboardIconComponent(props: ClipboardIconComponentProps) {

    const [copied, setCopied] = useState<boolean>(false);
  
    useEffect(() => {
        if(copied === true) {
          const timer = setTimeout(() => {
            setCopied(false)
          }, 1500);
          return () => clearTimeout(timer);
        }
    }, [copied]);
  
    return (
        <>
            {!copied ? (
                <td className={styles.clipboardIconCell}>
                    <FaRegClipboard 
                        className={styles.clipboardIcon}
                        onClick={ () => copyAddr(setCopied, props.address)}
                    />
                </td>
            ) : (
                <td className={styles.clipboardIconCell}>
                    <p className={styles.copiedMessage}><MdDone style={{ marginRight: '0.2rem' }}/>{' '}Copied!</p>
                </td>
            )}
        </>
    );
}


export default function DeployedTab() {

    //const [copied, setCopied] = useState(false);
    

    const { data, error, isPending } = useAsync({ promiseFn: listAllDeployedContracts});

    if (isPending) return (<p className={styles.load}>Loading...</p>)

    else if (error) { throw new Error("Error when trying to fetch Deployed Contracts") }

    else if (data) {      

        const rows: JSX.Element[] = [];

        data.forEach((deployedContract) => {
            rows.push(
                <tr key={deployedContract.index}>
                    <td className={styles.contractCell}>{deployedContract.name}</td>
                    <td>{deployedContract.address}</td>
                    <ClipboardIconComponent address={deployedContract.address}/>
                </tr>
            );
        });

        return(
            <div className={styles.deployedTabContainer}>
                <table className={styles.deployedTabHead}>
                    <caption>DEPLOYED CONTRACTS</caption>
                    <colgroup>
                        <col className={styles.contractCol}></col>
                        <col className={styles.addressCol}></col>
                        <col className={styles.copyCol}></col>
                    </colgroup>
                    <thead>
                        <tr>
                            <th>Contract</th>
                            <th>Address</th>
                            <th className={styles.copyThead}>Copy</th>
                        </tr>
                    </thead>
                </table>
                <div className={styles.deployedTabContentContainer}>
                    <table className={styles.deployedTabContent}>
                        <colgroup>
                            <col className={styles.contractCol}></col>
                            <col className={styles.addressCol}></col>
                            <col className={styles.copyCol}></col>
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