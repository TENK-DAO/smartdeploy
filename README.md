# SmartDeploy

A framework for publishing, deploying, and upgrading Soroban smart contracts.

## Getting started

The key idea behind smartdeploy is that contract should be redeployable and maintainable. To solve this smartdeploy allows for publishing contract binaries and set of primitives to ensure that contracts can be redeployed.

### `smartdeploy-sdk`

First you need to add smartdeploy-sdk to your contract's `Cargo.toml`

```bash
cargo add smartdeploy-sdk
```

### `soroban-cli`

Currently smartdeploy relies on yet unstable version of the CLI

```bash
cargo install soroban-cli --version 20.0.0-rc4
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



# Smartdeploy CLI

## Install
```
cargo install smartdeploy-cli
```

Currently the smartdeploy CLI has a `install` and `call` subcommand which lets you use a deployed contract's name when invoking it.

```bash
cargo run -- call smartdeploy -- --help
```

Can list out deployed_contracts

```bash
cargo run -- call smartdeploy -- list_deployed_contracts
```

