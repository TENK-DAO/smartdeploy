# SmartDeploy

A framework for publishing, deploying, and upgrading Soroban smart contracts.

## Getting started

The key idea behind smartdeploy is that contract should be redeployable and maintainable. To solve this smartdeploy allows for publishing contract binaries and set of primitives to ensure that contracts can be redeployed.

### `smartdeploy-sdk`

First you need to add smartdeploy-sdk to your contract's `Cargo.toml`

```toml
smartdeploy-sdk = { git = "https://github.com/TENK-DAO/smartdeploy", "tag" = "v0.1.0" }
```

### `soroban-cli`

Currently smartdeploy relies on yet unreleased features in `soroban-cli`, so you must install it with the following

```bash
cargo install --git https://github.com/ahalabs/soroban-tools --branch smartdeploy --debug --force soroban-cli
```

Next you need to add the following line to your contract:

```rs
smartdeploy_sdk::core_riff!();
```

This adds three methods to your contract: `owner_set`, `owner_get`, and `redeploy`. After deploying your contract you must call `owner_set`, with your account so that you can call `redploy`, providing the hash of an installed contract (see next section for more info).

However, while you are developing this can be tedious so there is also `dev_deploy`.

```rs
smartdeploy::dev_deploy!();
```

This adds the method `dev_deploy` to your contract. This let's you redeploy your contract with just a wasm binary. However, it is not meant to be used on a public network as anyone can call it.

So assuming you choose the later option, after initially deploying your contract, you'll get a contract id, e.g.`CA5YTYGCKWYJW7P3JNSMVKV2HRN4TOFMZ2VW4AQ5WQS4P3BGHNOU3PPX`.

Then after making changes to your contract you can redeploy with the following:

```bash
soroban contract invoke --id CA5YTYGCKWYJW7P3JNSMVKV2HRN4TOFMZ2VW4AQ5WQS4P3BGHNOU3PPX \
                        --network <standalone/futurenet> \
                        --fee 1000000 \
                        --source <address or name of identity> \ 
                        -- \ 
                        dev_deploy --wasm-file-path <path/to/contarct.wasm>
```

This makes use of the new `--<arg>-file-path` argument that is added to the contract's generated CLI, allowing you to pass a file as the input to the contract's method. `--fee` is also used because the default is 100, which is probably not enough for the cost of including the bytes of your contract.

The other core idea behind smartdeploy is that contract can be published so that anyone can deploy an instance of it. This means that this fee is only paid by the contract publisher.

Note you can also use a `.env` file in the same directory where you call `invoke` to set CLI arguments. For example,

```.env
SOROBAN_NETWORK=standalone
SOROBAN_ACCOUNT=default
SOROBAN_FEE=1000000
```


Uses [Loam-SDK](https://github.com/loambuild/loam-sdk)

## Publishing

Currently smart contracts are `installed` to the network allowing them to be deployed using the hash of the installed contract. However, unless you are the author of the smart contract you won't know much about which hashes correspond to which contracts.  Furthermore, there is no notion of a version of each hash so you would find many contracts with almost identical implementations.



## Trying it out

First clone this repo:

```
git clone https://github.com/TENK-DAO/smartdeploy
cd smartdeploy
```

## Setup

### install `just`

[just](https://github.com/casey/just) is a task running for helping executing graphs of dependent tasks.

```bash
cargo install just
```

though it's prefered to use

```bash
cargo binstall just
```

```bash
cargo install cargo-binstall 
```

As this the relesaed binary, skipping having to build it. 

At the moment only Unix OSes are supported (sorry!) in the follow steps. 

### Setup local soroban binary

This will install the correct binaries to `./target/bin`.

```bash
just setup
```

### Setting up smartdeploy itself

Currently standalone is the default network. (see [.env](./.env))

You'll need docker installed. Then you can open a separate terminal and run:

```bash
just start_docker
```

To deploy your own Smartdeploy first run:

```bash
just clean
```

Then

```bash
just publish_all
```

This command creates a new smartdeploy contract and publishes all the examples to it. Then deploys a contract with the same name.  Lastly a local script is created for each one and this path is visible to the just script.

```bash
just soroban --list
```

should print something like:

```
Installed Plugins:
    smartdeploy
    increment
    errors
```

And now that soroban has a plugin system you can invoke it like

```
just soroban increment --help
```

or for short

```bash
just s increment --help
```


# Smartdeploy CLI

## Install
```
cargo install
```

Currently the smartdeploy CLI has a `call` subcommand which lets you use a deployed contract's name when invoking it.

```bash
cargo run -- call smartdeploy -- --help
```

Can list out deployed_contracts

```bash
cargo run -- call smartdeploy -- list_deployed_contracts
```

