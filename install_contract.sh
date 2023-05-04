
#!/usr/bin/env bash
set -e
ID=$(soroban smartdeploy fetch_contract_id --deployed_name $1 | jq -r .)
echo $ID
FILE=./target/bin/soroban-$1
echo "#!/usr/bin/env bash \nsoroban contract invoke --id ${ID} -- \$@" > $FILE

chmod +x $FILE