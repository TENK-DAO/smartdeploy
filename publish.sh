
#!/usr/bin/env bash

smartdeploy="soroban contract invoke  --source default --id $(cat contract_id.txt) --"

for name in $(cargo metadata --format-version 1 --no-deps | jq -r '.packages[].name' | rg --color never soroban)
do
  name="${name//-/_}"
  hash=$(soroban contract install --wasm ./target/wasm32-unknown-unknown/release-with-logs/$name.wasm);
  $smartdeploy publish --contract_name $name --hash $hash --author default
  $smartdeploy deploy --contract_name $name --deployed_name $name --owner default
done


# for i in  
# do
#   echo $i
# then