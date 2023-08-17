# Load in `.env`
set dotenv-load

export PATH := './target/bin:' + env_var('PATH')
export SOROBAN_NETWORK := 'futurenet'
TARGET_DIR := './target/loam'
SMARTDEPLOY := TARGET_DIR / 'smartdeploy.wasm'
BASE := TARGET_DIR / 'base.wasm'
soroban := 'target/bin/soroban'
loam := 'target/bin/loam'
FILE := 'target/bin/soroban-smartdeploy'
UPLOAD_FEE := '10000000'
# smartdeploy := 'soroban contract invoke --id ' + env_var('DEFAULT_ID') + ' -- '
# hash := if path_exists({{SMARTDEPLOY}}) == "true" {`soroban contract install --wasm ./target/wasm32-unknown-unknown/contracts/example_status_message.wasm --config-dir ./target` } else {""}
id:=`cat contract_id.txt`
ROOT_DIR := 'target/contracts/smartdeploy'

[private]
@default: setup
    @just build

@soroban +args:
    {{soroban}} {{args}}

# Execute plugin
s name +args:
    @just soroban {{ name }} {{ args }}

smartdeploy +args:
    @soroban contract invoke --id {{id}} -- {{args}}

smartdeploy_raw +args:
    @soroban contract invoke --id {{id}} {{args}}

@soroban_install name:
    @soroban contract install --wasm ./target/wasm32-unknown-unknown/release-with-logs/{{name}}.wasm

@generate: build
    @just soroban contract bindings typescript \
        --contract-id {{id}} \
        --wasm {{SMARTDEPLOY}} \
        --output-dir {{ ROOT_DIR }}/smartdeploy \
        --overwrite \

target:
    echo {{TARGET_DIR}}
    echo {{SMARTDEPLOY}}

build:
    loam build --profile release-with-logs --out-dir target/loam


[private]
setup_default:
   soroban config identity generate default --config-dir $CONFIG_DIR --network futurenet

@setup:
    echo {{ if path_exists(soroban) == "true" { "" } else { `cargo install_soroban_dev` } }}
    echo {{ if path_exists(loam) == "true" { "" } else { `cargo install_loam` } }}
    echo {{ if path_exists(env_var('CONFIG_DIR') / 'identity/default.toml') == "true" { "" } else { `just setup_default` } }}
    

@deploy_self: build
    @./deploy.sh

[private]
@claim_self:
    just smartdeploy claim_deployed_contract --deployed_name smartdeploy --id $(cat contract_id.txt)

[private]
@install_self:
    echo "#!/usr/bin/env bash \njust soroban contract invoke --id {{id}} -- \$@" > {{ FILE }}
    chmod +x {{ FILE }}


publish_all: build
    #!/usr/bin/env bash
    just install_self;
    for name in $(loam build --ls)
    do
        if [ "$name" != "smartdeploy" ]; then
            echo $name;
            name="${name//-/_}";
            just publish_one $name
        fi
    done

[private]
@publish_one name:
    @just publish {{ name }}
    @just deploy {{ name }} {{ name }}
    @just install_contract {{ name }}

@deploy contract_name deployed_name owner='default':
    just smartdeploy_raw --source {{owner}} -- deploy --contract_name {{contract_name}} --deployed_name {{deployed_name}} --owner {{owner}}

@publish name kind='Patch' author='default':
    just smartdeploy_raw --fee {{UPLOAD_FEE}} --source {{author}} -- \
        publish \
        --contract_name {{name}} \
        --bytes-file-path ./target/loam/{{name}}.wasm \
        --kind {{kind}} \
        --author {{author}} \

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

