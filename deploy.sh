#!/bin/bash

make

CURRENT_HASH=$(soroban contract install \
       --network futurenet \
       --source default \
       --wasm ./target/wasm32-unknown-unknown/release-with-logs/smart_deploy.wasm)


FILE_HASH=""



if test -e "./hash.txt"; then
  FILE_HASH=$(cat ./hash.txt)
else
  echo "New Binary!"
fi

if test "$CURRENT_HASH" = "$FILE_HASH"; then
  echo "Already deployed"
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

    echo $SALT

    ID=$(soroban contract deploy \
            --network futurenet \
            --salt $SALT \
            --wasm-hash $CURRENT_HASH);
    echo -n $ID > contract_id.txt
fi

ID=$(cat contract_id.txt)


echo SmartDeploy $ID
smartdeploy="soroban contract invoke --network futurenet --source default --id $ID --"
$smartdeploy --help

if test "$FILE_HASH" = ""; then
    $smartdeploy register_name --contract_name hello_world --author "$(soroban config identity address default)"
    $smartdeploy publish_binary \
      --contract_name hello_world \
      --hash 6c453071976d247e6c8552034ba24a7b6ba95d599eb216d72a15bf8bd7176a8a \
      --repo https://github.com/AhaLabs/soroban-examples/tree/0d977e1b56d3b7007855f6557248e17f37081699/hello_world
fi
