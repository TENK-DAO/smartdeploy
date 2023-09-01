#!/bin/bash



CURRENT_HASH=$(just soroban contract install --source default --wasm ./target/loam/smartdeploy.wasm)
echo current hash $CURRENT_HASH
author=$(just soroban config identity address default)
echo $author

FILE_HASH=""

if test -e "./hash.txt"; then
  FILE_HASH=$(cat ./hash.txt)
  echo "New Binary!"
fi

if test "$CURRENT_HASH" = "$FILE_HASH"; then
  :
else
  FILE_HASH=""
  echo -n "$CURRENT_HASH" > ./hash.txt
    printf -v a "%08d" $RANDOM
    printf -v b "%08d" $RANDOM
    printf -v c "%08d" $RANDOM
    printf -v d "%08d" $RANDOM
    printf -v e "%08d" $RANDOM
    printf -v f "%08d" $RANDOM
    printf -v g "%08d" $RANDOM
    printf -v h "%08d" $RANDOM

    SALT=$a$b$c$d$e$f$g$h

    ID=$(just soroban contract deploy \
            --salt $SALT \
            --wasm-hash $CURRENT_HASH);
    echo -n $ID > contract_id.txt
fi



ID=$(cat contract_id.txt)

if test "$ID" = ""; then
  echo "No ID found"
  exit 1
fi
echo $ID


smartdeploy="just soroban --quiet contract invoke --id $ID"

if test "$FILE_HASH" = ""; then
   just publish smartdeploy
   just claim_self
fi

just smartdeploy_id > .soroban/network/$SOROBAN_NETWORK/smartdeploy.json
