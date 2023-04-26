# Load in `.env`
set dotenv-load

export PATH := './target/bin:' + env_var('PATH')
TARGET_DIR := './target/wasm32-unknown-unknown/release-with-logs'
SMARTDEPLOY := TARGET_DIR / 'smartdeploy.wasm'
BASE := TARGET_DIR / 'base.wasm'
soroban := 'target/bin/soroban'
smartdeploy := 'soroban contract invoke --id ' + env_var('DEFAULT_ID')   + ' --config-dir ' + env_var('CONFIG_DIR') + ' -- '
# hash := if path_exists({{SMARTDEPLOY}}) == "true" {`soroban contract install --wasm ./target/wasm32-unknown-unknown/contracts/example_status_message.wasm --config-dir ./target` } else {""}


path:
    echo ${PATH}

target:
    echo {{TARGET_DIR}}
    echo {{SMARTDEPLOY}}

build:
    cargo build --target wasm32-unknown-unknown --profile release-with-logs 


[private]
setup_default:
   soroban config identity generate -d default --config-dir $CONFIG_DIR

@setup:
    echo {{ if path_exists(soroban) == "true" { "" } else { `cargo install_soroban` } }}
    echo {{ if path_exists(env_var('CONFIG_DIR') / 'identity/default.toml') == "true" { "" } else { `just setup_default` } }}
    

deploy: build setup
    soroban contract deploy --id $DEFAULT_ID --wasm {{SMARTDEPLOY}} --config-dir $CONFIG_DIR
    {{smartdeploy}} owner_set --owner default
    {{smartdeploy}} --help
    
    