# Load in `.env`
set dotenv-load

export PATH := './target/bin:' + env_var('PATH')
export SOROBAN_NETWORK := 'futurenet'
TARGET_DIR := './target/wasm32-unknown-unknown/release-with-logs'
SMARTDEPLOY := TARGET_DIR / 'smartdeploy.wasm'
BASE := TARGET_DIR / 'base.wasm'
soroban := 'target/bin/soroban'
FILE := 'target/bin/soroban-smartdeploy'
# smartdeploy := 'soroban contract invoke --id ' + env_var('DEFAULT_ID') + ' -- '
# hash := if path_exists({{SMARTDEPLOY}}) == "true" {`soroban contract install --wasm ./target/wasm32-unknown-unknown/contracts/example_status_message.wasm --config-dir ./target` } else {""}
id:=`cat contract_id.txt`
ROOT_DIR := 'target/contracts/smartdeploy'

@soroban +args:
    {{soroban}} {{args}}

# Execute plugin
s name +args:
    @just soroban {{ name }} {{ args }}

smartdeploy +args:
    @soroban contract invoke --id {{id}} -- {{args}}

@soroban_install name:
    @soroban contract install --wasm ./target/wasm32-unknown-unknown/release-with-logs/{{name}}.wasm

@generate: build
    (cd ~/c/s/soroban-cli; cargo build)
    @just soroban contract bindings ts \
        --wasm {{SMARTDEPLOY}} \
        --contract-id {{id}} \
        --contract-name smartdeploy \
        --root-dir {{ ROOT_DIR }}
    cd {{ ROOT_DIR }}; npm i && npm run build

target:
    echo {{TARGET_DIR}}
    echo {{SMARTDEPLOY}}

build package="smartdeploy" profile='release-with-logs':
    soroban contract build --package {{package}} --profile {{profile}} 


[private]
setup_default:
   soroban config identity generate -d default --config-dir $CONFIG_DIR

@setup:
    echo {{ if path_exists(soroban) == "true" { "" } else { `cargo install_soroban` } }}
    echo {{ if path_exists(env_var('CONFIG_DIR') / 'identity/default.toml') == "true" { "" } else { `just setup_default` } }}
    

@deploy_self: build
    @./deploy.sh

@install_self:
    echo "#!/usr/bin/env bash \njust soroban contract invoke --id {{id}} -- \$@" > {{ FILE }}
    chmod +x {{ FILE }}


publish_all:
    #!/usr/bin/env bash
    just install_self;
    for name in $(cargo metadata --format-version 1 --no-deps | jq -r '.packages[].name')
    do
        if [ "$name" != "smartdeploy" ]; then
            name="${name//-/_}";
            hash=$(just soroban_install $name);
            just publish_one $name $hash
        fi
    done

[private]
@publish_one name hash:
    @just publish {{ name }} {{ hash }}
    @just deploy {{ name }} {{ name }}
    @just install_contract {{ name }}

@deploy contract_name deployed_name owner='default':
    just smartdeploy deploy --contract_name {{contract_name}} --deployed_name {{deployed_name}} --owner {{owner}}

@publish name hash kind='Patch' author='default':
    @soroban --quiet contract invoke --id {{id}} -- publish --contract_name {{name}} --hash {{hash}} --author {{author}}

# Delete non-wasm artifacts
@clean:
    rm -rf .soroban/*.json hash.txt target/bin/soroban-*



# List Published Contracts
published_contracts start='0' limit='100':
    @just smartdeploy list_published_contracts --start {{start}} --limit {{limit}} | jq .

# List Deployed Contracts
deployed_contracts start='0' limit='100':
    @just smartdeploy list_deployed_contracts --start {{start}} --limit {{limit}} | jq .



@install_contract name:
    ./install_contract.sh {{name}}

