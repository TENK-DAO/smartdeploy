#!/bin/bash

CURRENT_HASH=$(stellar contract install --source default --network testnet --wasm ./target/loam/smartdeploy.wasm)
echo current hash "$CURRENT_HASH"
author=$(stellar keys address default)
echo "$author"

FILE_HASH=""

if test -e "./hash.txt"; then
  FILE_HASH=$(cat ./hash.txt)
  echo "New Binary!"
fi

if test "$CURRENT_HASH" = "$FILE_HASH"; then
  :
else
  FILE_HASH=""
  echo -n "$CURRENT_HASH" >./hash.txt

  ID=$(stellar contract deploy \
    --wasm-hash "$CURRENT_HASH" --alias smartdeploy)
  loam update-env --name SOROBAN_CONTRACT_ID --value "$ID"
  loam update-env --name SMARTDEPLOY_CONTRACT_ID --value "$ID"
  echo -n "$ID" > contract_id.txt
fi

ID=$(cat contract_id.txt)

if test "$ID" = ""; then
  echo "No ID found"
  exit 1
fi
echo $ID

# ./subscribe_events.sh $ID

# smartdeploy="soroban --quiet contract invoke --id $ID"

# if test "$FILE_HASH" = ""; then
#   just publish smartdeploy
#   just smartdeploy claim_already_deployed_contract --deployed_name smartdeploy --owner default --id "$ID"
#   just set_owner default
# fi

if test "$SOROBAN_NETWORK" = "testnet"; then
  echo "\"$ID\"" > ./crates/smartdeploy-cli/src/testnet/smartdeploy.json
fi
