
#!/usr/bin/env bash


for name in $(cargo metadata --format-version 1 --no-deps | jq -r '.packages[].name' | rg --color never soroban)
do

  name="${name//-/_}"
  hash=$(just soroban_install $name);
  just publish $name $hash
  just deploy $name $name
done

