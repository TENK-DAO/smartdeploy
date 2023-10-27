# shellcheck disable=all
# Load in `.env`
set dotenv-load

export PATH := './target/bin:' + env_var('PATH')
export SOROBAN_NETWORK := env_var('SOROBAN_NETWORK')
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
    @just smartdeploy_raw -- {{args}}

@smartdeploy_raw +args:
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

build +args='':
    loam build --profile release-with-logs --out-dir target/loam {{args}}


[private]
setup_default:
   -soroban config identity generate default --config-dir $CONFIG_DIR

@setup:
    cargo binstall -y --install-path ./target/bin soroban-cli --version 20.0.0-rc.4.1
    echo {{ if path_exists(loam) == "true" { "" } else { `cargo install_loam` } }}
    echo {{ if path_exists(env_var('CONFIG_DIR') / '.soroban/identity/default.toml') == "true" { "" } else { `just setup_default` } }}


@fund_default:
    soroban config identity fund default

@deploy_self:
    just build --package smartdeploy
    @./deploy.sh

[private]
@claim_self:
    just smartdeploy claim_deployed_contract --deployed_name smartdeploy --id {{ id }}

[private]
@install_self:
    echo "#!/usr/bin/env bash \njust soroban contract invoke --id {{id}} -- \$@" > {{ FILE }}
    chmod +x {{ FILE }}


publish_all: fund_default deploy_self
    #!/usr/bin/env bash
    set -e;
    echo $SOROBAN_NETWORK;
    just install_self;
    for name in $(loam build --ls)
    do
        if [ "$name" != "smartdeploy" ]; then
            echo $name;
            just build --package $name;
            name="${name//-/_}";
            just publish_one $name
            cargo run --quiet -- install $name
        fi
    done

[private]
@publish_one name:
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
    rm -rf .soroban/*.json hash.txt target/bin/soroban-* target/smartdeploy/*

# Delete installed binaries
@clean_installed_binaries:
    rm target/bin/loam target/bin/soroban target/bin/smartdeploy

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
    stellar/quickstart:soroban-dev@sha256:c1030a6ee75c31ba6807b8feddded2af23789b5f2c9be3ac55a550630a35ef42 \
    --standalone \
    --enable-soroban-rpc \

smartdeploy_id:
    just smartdeploy fetch_contract_id --deployed_name smartdeploy | jq .
