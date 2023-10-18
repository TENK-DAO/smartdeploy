import styles from './style.module.css';

export default function PublishedTab() {
    return(
        <div className={styles.publishedTabContainer}>
            <table className={styles.publishedTabHead}>
                <caption>PUBLISHED CONTRACTS</caption>
                <col className={styles.contractCol}></col>
                <col className={styles.authorCol}></col>
                <col className={styles.versionCol}></col>
                <col className={styles.hashCol}></col>
                <thead>
                    <tr>
                        <th>Contract</th>
                        <th>Author</th>
                        <th>Version</th>
                        <th>Hash</th>
                    </tr>
                </thead>
            </table>
            <div className={styles.publishedTabContentContainer}>
                <table className={styles.publishedTabContent}>
                    <col className={styles.contractCol}></col>
                    <col className={styles.authorCol}></col>
                    <col className={styles.versionCol}></col>
                    <col className={styles.hashCol}></col>
                    <tbody>
                        <tr>
                            <td className={styles.contractCell}>Errorzzzzzzzzzzzzzzzzzzzzzzzzs</td>
                            <td className={styles.authorCell}>CBG572OEPI6LNOUHCOCEUOCDII72UNUWNHCCGB4CHCHB52VQ3RI6NEOWK</td>
                            <td className={styles.versionCell}>v0.0.105256</td>
                            <td className={styles.hashCell}>Icon</td>
                        </tr>
                        <tr>
                            <td>vzpeivb</td>
                            <td>C</td>
                            <td>v0.0.1</td>
                            <td>Icon</td>
                        </tr>
                        <tr>
                            <td>Errors</td>
                            <td>k</td>
                            <td>v0.0.1</td>
                            <td>Icon</td>
                        </tr>
                        <tr>
                            <td>Errors</td>
                            <td>k</td>
                            <td>v0.0.1</td>
                            <td>Icon</td>
                        </tr>
                        <tr>
                            <td>Errors</td>
                            <td>k</td>
                            <td>v0.0.1</td>
                            <td>Icon</td>
                        </tr>
                        <tr>
                            <td>Errors</td>
                            <td>k</td>
                            <td>v0.0.1</td>
                            <td>Icon</td>
                        </tr>
                        <tr>
                            <td>Errors</td>
                            <td>k</td>
                            <td>v0.0.1</td>
                            <td>Icon</td>
                        </tr>
                        <tr>
                            <td>Errors</td>
                            <td>k</td>
                            <td>v0.0.1</td>
                            <td>Icon</td>
                        </tr>
                        <tr>
                            <td>Errors</td>
                            <td>k</td>
                            <td>v0.0.1</td>
                            <td>Icon</td>
                        </tr>
                        <tr>
                            <td>Errors</td>
                            <td>k</td>
                            <td>v0.0.1</td>
                            <td>Icon</td>
                        </tr>
                        <tr>
                            <td>Errors</td>
                            <td>k</td>
                            <td>v0.0.1</td>
                            <td>Icon</td>
                        </tr>
                        <tr>
                            <td>Errors</td>
                            <td>k</td>
                            <td>v0.0.1</td>
                            <td>Icon</td>
                        </tr>
                    </tbody>

                </table>
            </div>
        </div>
        
    )
}