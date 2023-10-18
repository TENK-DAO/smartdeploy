import styles from './style.module.css';

export default function PublishedTab() {
    return(
        <div className={styles.publishedTabContainer}>
            <table className={styles.publishedTabHead}>
                <caption>PUBLISHED CONTRACTS</caption>
                <colgroup>
                    <col className={styles.contractCol}></col>
                    <col className={styles.authorCol}></col>
                    <col className={styles.versionCol}></col>
                    <col className={styles.hashCol}></col>
                </colgroup>
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
                    <colgroup>
                        <col className={styles.contractCol}></col>
                        <col className={styles.authorCol}></col>
                        <col className={styles.versionCol}></col>
                        <col className={styles.hashCol}></col>
                    </colgroup>
                    <tbody>
                        <tr>
                            <td className={styles.contractCell}>Errorrrrrrrrrrrrrrrrrrrrrrrr</td>
                            <td>CBG572OEPI6LNOUHCOCEUOCDII72UNUWNHCCGB4CHCHB52VQ3RI6NEOWK</td>
                            <td>v0.0.105256</td>
                            <td>Icon</td>
                        </tr>
                        <tr>
                            <td className={styles.contractCell}>Smart_Deploy</td>
                            <td>CBG572OEPI6LNOUHCOCEUOCDII72UNUWNHCCGB4CHCHB52VQ3RI6NEOWK</td>
                            <td>v0.0.105256</td>
                            <td>Icon</td>
                        </tr>

                        <tr>
                            <td className={styles.contractCell}>Errorrrrrrrrr</td>
                            <td>CBG572OEPI6LNOUHCOCEUOCDII72UNUWNHCCGB4CHCHB52VQ3RI6NEOWK</td>
                            <td>v0.0.105256</td>
                            <td>Icon</td>
                        </tr>
                        <tr>
                            <td className={styles.contractCell}>Err</td>
                            <td>CBG572OEPI6LNOUHCOCEUOCDII72UNUWNHCCGB4CHCHB52VQ3RI6NEOWK</td>
                            <td>v0.0.105256</td>
                            <td>Icon</td>
                        </tr>

                        <tr>
                            <td className={styles.contractCell}>Errorrrr</td>
                            <td>CBG572OEPI6LNOUHCOCEUOCDII72UNUWNHCCGB4CHCHB52VQ3RI6NEOWK</td>
                            <td>v0.0.105256</td>
                            <td>Icon</td>
                        </tr>
                        <tr>
                            <td className={styles.contractCell}>Errorrrrrrrr</td>
                            <td>CBG572OEPI6LNOUHCOCEUOCDII72UNUWNHCCGB4CHCHB52VQ3RI6NEOWK</td>
                            <td>v0.0.105256</td>
                            <td>Icon</td>
                        </tr>

                        <tr>
                            <td className={styles.contractCell}>zeobv</td>
                            <td>CBG572OEPI6LNOUHCOCEUOCDII72UNUWNHCCGB4CHCHB52VQ3RI6NEOWK</td>
                            <td>v0.0.105256</td>
                            <td>Icon</td>
                        </tr>
                        <tr>
                            <td className={styles.contractCell}>Errorrrrrrrrrrrrrrrrrrrrrrrr</td>
                            <td>CBG572OEPI6LNOUHCOCEUOCDII72UNUWNHCCGB4CHCHB52VQ3RI6NEOWK</td>
                            <td>v0.0.105256</td>
                            <td>Icon</td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
    )
}