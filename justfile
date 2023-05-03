# Load in `.env`
set dotenv-load

export PATH := './target/bin:' + env_var('PATH')
TARGET_DIR := './target/wasm32-unknown-unknown/release-with-logs'
SMARTDEPLOY := TARGET_DIR / 'smartdeploy.wasm'
BASE := TARGET_DIR / 'base.wasm'
soroban := 'target/bin/soroban'
# smartdeploy := 'soroban contract invoke --id ' + env_var('DEFAULT_ID') + ' -- '
# hash := if path_exists({{SMARTDEPLOY}}) == "true" {`soroban contract install --wasm ./target/wasm32-unknown-unknown/contracts/example_status_message.wasm --config-dir ./target` } else {""}
id:=`cat contract_id.txt`

smartdeploy +args:
    soroban contract invoke -- {{args}}

path:
    echo ${PATH}

target:
    echo {{TARGET_DIR}}
    echo {{SMARTDEPLOY}}

build profile='release-with-logs':
    cargo build --target wasm32-unknown-unknown --profile {{profile}} 


[private]
setup_default:
   soroban config identity generate -d default --config-dir $CONFIG_DIR

@setup:
    echo {{ if path_exists(soroban) == "true" { "" } else { `cargo install_soroban` } }}
    echo {{ if path_exists(env_var('CONFIG_DIR') / 'identity/default.toml') == "true" { "" } else { `just setup_default` } }}
    

deploy_self:
    ./deploy.sh

deploy: build setup
    soroban contract deploy --id $DEFAULT_ID --wasm {{SMARTDEPLOY}} --config-dir $CONFIG_DIR
    just smartdeploy owner_set --owner default
    just smartdeploy --help

publish name hash kind='Patch' author='default':
    soroban contract invoke --id {{id}} -- publish --contract_name {{name}} --hash {{hash}} --author {{author}}