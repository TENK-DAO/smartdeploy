# shellcheck disable=all
# Load in `.env`
set dotenv-load

export PATH := './target/bin:' + env_var('PATH')
TARGET_DIR := './target/loam'
SMARTDEPLOY := TARGET_DIR / 'smartdeploy.wasm'
BASE := TARGET_DIR / 'base.wasm'
stellar := 'target/bin/stellar'
loam := 'target/bin/loam'
FILE := 'target/bin/stellar-smartdeploy'
UPLOAD_FEE := '10000000'
# smartdeploy := 'stellar contract invoke --id ' + env_var('DEFAULT_ID') + ' -- '
# hash := if path_exists({{SMARTDEPLOY}}) == "true" {`stellar contract install --wasm ./target/wasm32-unknown-unknown/contracts/example_status_message.wasm --config-dir ./target` } else {""}
ROOT_DIR := 'target/contracts/smartdeploy'

[private]
@default: setup build
    stellar config network add standalone \
        --rpc-url http://localhost:8000/stellar/rpc \
        --network-passphrase "Standalone Network ; February 2017"

# check clippy and cargo-spellcheck
check:
    cargo spellcheck -c ./.config/spellcheck.toml

@stellar +args:
   stellar {{args}}

# Execute plugin
s name +args:
    @stellar {{ name }} {{ args }}

smartdeploy +args:
    @just smartdeploy_raw -- {{args}}

@smartdeploy_raw +args:
    echo $stellar_CONTRACT_ID
    @stellar contract invoke {{args}}

@stellar_install name:
    @stellar contract install --wasm ./target/wasm32-unknown-unknown/release-with-logs/{{name}}.wasm

@generate: build
    @stellar contract bindings typescript \
        --contract-id $stellar_CONTRACT_ID \
        --wasm {{SMARTDEPLOY}} \
        --output-dir {{ ROOT_DIR }}/smartdeploy \
        --overwrite \

target:
    echo {{TARGET_DIR}}
    echo {{SMARTDEPLOY}}

build +args='':
    loam build --profile release-with-logs --out-dir target/loam {{args}}


[private]
setup_default:
   -stellar keys generate default

@setup:
    cargo binstall -y --install-path ./target/bin stellar-cli  --version 21.0.0
    cargo binstall -y --install-path ./target/bin loam-cli --version 0.9.4
    just setup_default


@fund_default:
    stellar keys fund default

@deploy_self:
    just build --package smartdeploy
    @./deploy.sh

[private]
@claim_self owner='default':
    echo $stellar_CONTRACT_ID
    just smartdeploy claim_already_deployed_contract --deployed_name smartdeploy --owner {{owner}}

@set_owner owner:
    @just smartdeploy_raw -- owner_set --new_owner {{ owner }} 

[private]
@install_self:
    echo "#!/usr/bin/env bash \nstellar contract invoke -- \$@" > {{ FILE }}
    chmod +x {{ FILE }}


publish_all: fund_default
    #!/usr/bin/env bash
    set -e;
    just install_self;
    for name in $(loam build --ls)
    do
        if [ "$name" != "smartdeploy" ]; then
            echo $name;
            just build --package $name;
            name="${name//-/_}";
            just publish_and_deploy_one $name
            cargo r -- install $name
        fi
    done

[private]
@publish_and_deploy_one name:
    @just publish {{ name }}
    @just deploy {{ name }} {{ name }}

@deploy contract_name deployed_name owner='default':
    @just smartdeploy_raw --source {{owner}} -- deploy --contract_name {{contract_name}} --deployed_name {{deployed_name}} --owner {{owner}}

@dev_deploy name file owner='default':
    just smartdeploy_raw --fee {{UPLOAD_FEE}} --source {{owner}} -- \
        dev_deploy \
        --owner {{owner}}} \
        --name {{name}} \
        --wasm-file-path {{file}}} \

@publish name kind='Patch' author='default':
    @just smartdeploy_raw --fee {{UPLOAD_FEE}} --source {{author}} -- \
        publish \
        --contract_name {{name}} \
        --wasm-file-path ./target/loam/{{name}}.wasm \
        --kind {{kind}} \
        --author {{author}} \

# Delete non-wasm artifacts
@clean:
    rm -rf .stellar/*.json hash.txt target/bin/stellar-* target/smartdeploy/*

# Delete installed binaries
@clean_installed_binaries:
    rm target/bin/loam target/bin/stellar target/bin/smartdeploy

# List Published Contracts
published_contracts start='0' limit='100':
    @just smartdeploy list_published_contracts --start {{start}} --limit {{limit}} | jq .

# List Deployed Contracts
deployed_contracts start='0' limit='100':
    @just smartdeploy list_deployed_contracts --start {{start}} --limit {{limit}} | jq .



@install_contract name:
    ./install_contract.sh {{name}}

start_docker:
    docker run --rm -it \
    -p 8000:8000 \
    --name stellar \
    stellar/quickstart:stellar-dev@sha256:c1030a6ee75c31ba6807b8feddded2af23789b5f2c9be3ac55a550630a35ef42 \
    --local \
    --enable-stellar-rpc \

smartdeploy_id:
    just smartdeploy fetch_contract_id --deployed_name smartdeploy | jq .
