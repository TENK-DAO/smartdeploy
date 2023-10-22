import { FaRegClipboard } from "react-icons/fa";
import styles from './style.module.css';

import { smartdeploy } from "@/pages";
import { Ok, Err } from 'smartdeploy-client'
import { useAsync } from "react-async";

interface DeployedContract {
    name: string;
    address: string;
}


async function listAllDeployedContracts() {

    return await smartdeploy
                            .listDeployedContracts({start: undefined, limit: undefined})
                            .then((response) => {

                                if (response instanceof Ok) {

                                    let deployedContracts: DeployedContract[] = [];

                                    const contractArray =  response.unwrap();

                                    contractArray.forEach(([name, address]) => {

                                        const parsedDeployedContract: DeployedContract = {
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


export default function DeployedTab() {

    const { data, error, isPending } = useAsync({ promiseFn: listAllDeployedContracts});

    if (isPending) return (<p className={styles.load}>Loading...</p>);

    else if (error) { throw new Error("Error when trying to fetch Deployed Contracts");}

    else if (data) {

        const rows: JSX.Element[] = [];

        data.forEach((deployedContract, item) => {
            rows.push(
                <tr key={item}>
                    <td className={styles.contractCell}>{deployedContract.name}</td>
                    <td>{deployedContract.address}</td>
                    <td className={styles.clipboardIcon}><FaRegClipboard/></td>
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
                            <th>Copy</th>
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