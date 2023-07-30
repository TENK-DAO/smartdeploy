#!/bin/bash

CURRENT_HASH=$(just soroban contract install \
       --source default \
       --wasm ./target/wasm32-unknown-unknown/release-with-logs/smartdeploy.wasm)

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



author=$(just soroban config identity address default)
ID=$(cat contract_id.txt)


smartdeploy="just soroban --quiet contract invoke  --source default --id $ID --"

if test "$FILE_HASH" = ""; then
    $smartdeploy publish \
      --contract_name smartdeploy \
      --hash $(cat hash.txt) \
      --author $author \
      --repo https://github.com/tenk-dao/smart-deploy
    
   $smartdeploy deploy --contract_name smartdeploy --owner default --deployed_name "smartdeploy"
fi

