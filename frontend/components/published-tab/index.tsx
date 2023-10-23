import { BsSendPlus } from 'react-icons/bs';
import styles from './style.module.css';

import { smartdeploy } from "@/pages";
import { Ok, Err } from 'smartdeploy-client'
import { useAsync } from "react-async";

interface PublishedContract {
    index: number;
    name: string;
    author: string;
    version: string;
    hash: string;
}

async function listAllPublishedContracts() {

    return await smartdeploy
                            .listPublishedContracts({start: undefined, limit: undefined})
                            .then((response) => {

                                if (response instanceof Ok) {

                                    let publishedContracts: PublishedContract[] = [];

                                    const contractArray =  response.unwrap();

                                    contractArray.forEach(([name, publishedContract], i) => {
                                        
                                        const version = publishedContract.versions.keys().next().value;
                                        const major = version.major;
                                        const minor = version.minor;
                                        const patch = version.patch;
                                        const versionString = `v.${major}.${minor}.${patch}`;

                                        const hash = publishedContract.versions.values().next().value.hash.join('');
                                        const parsedPublishedContract: PublishedContract = {
                                            index: i,
                                            name: name,
                                            author: publishedContract.author.toString(),
                                            version: versionString,
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



export default function PublishedTab() {

    // Mettre en dehors de PublishedTab()
    const deploy = async() => {

    }

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
                    <td>{publishedContract.version}</td>
                    <td className={styles.deployIconCell}><BsSendPlus className={styles.deployIcon} onClick={deploy}/></td>
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