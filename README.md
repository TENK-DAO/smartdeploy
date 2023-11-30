# SmartDeploy

A framework for publishing, developing, deploying, and upgrading Soroban smart contracts.

You can think of it as a package manager for Smart Contracts.

Consider the following contract:

```rs
#![no_std]
use soroban_sdk::{self, contract, contractimpl};

#[contract]
pub struct ContractA;

#[contractimpl]
impl ContractA {
    pub fn add(x: u32, y: u32) -> u32 {
        x.checked_add(y).expect("no overflow")
    }
}

smartdeploy_sdk::core_riff!();
```

The last line `smartdeploy_sdk::core_riff` ensures that the contract is redeployable.

After building you can publish the contract:

```bash
smartdeploy call smartdeploy --fee 10000 -- \
                                publish \
                                --contract-name contract-a \
                                --wasm-file-path .../contract_a.wasm \
                                --author default
```

This invokes the smartdeploy contract directly and calling the method `publish`. It also uses `--wasm-file-path` read in the bytes of the file as the arg.

To deploy you can use the deploy subcommand which, as we'll see later, creates a CLI interface for calling a custom init function of the contract in the same transaction as the deploy.

```
smartdeploy deploy --published-name contract_a \
                   --deployed-name contract_a \
                   --source default
```



Now let's say that we have a second contract b

```rs
#![no_std]
use soroban_sdk::{self, contract, contractimpl, Address, Env};

smartdeploy_sdk::import_contract!(contract_a);

#[contract]
pub struct ContractB;

#[contractimpl]
impl ContractB {
    /// Add two numbers using the `add` method of ContractA.
    pub fn add_with(env: Env, contract_id: Address, x: u32, y: u32) -> u32 {
        contract_a::Client::new(&env, &contract_id).add(&x, &y)
    }
}

smartdeploy_sdk::core_riff!();
```

We can "install" contract_a and then reference it to create a client:

```bash
smartdeploy install contract_a
```

Now the `import_contract` macro let's you import the contract the same way you would depend on a normal rust depenedency.
```rs
smartdeploy_sdk::import_contract!(contract_a);
...
contract_a::Client::new(&env, &contract_id).add(&x, &y)
```

Now build the contract since contract_a was added to `target/smartdeploy/contract_a/index.wasm`
```bash
soroban contract build --package contract-b
```

Next after publishing and deploying `contract_b`, let's take this a step further and create a third contract.

```rs
#![no_std]
use soroban_sdk::{self, contract, contractimpl, symbol_short, Env, Symbol};

smartdeploy_sdk::import_contract!(contract_a);
smartdeploy_sdk::import_contract!(contract_b);

#[contract]
pub struct ContractC;

const KEY: &Symbol = &symbol_short!("KEY");

fn default_value(env: &Env) -> u32 {
    env.storage().instance().get(KEY).unwrap_or_default()
}

#[contractimpl]
impl ContractC {
    /// Initialize the contract with a number to be used as the default value.
    pub fn init(env: Env, num: u32) {
        if !env.storage().instance().has(KEY) {
            env.storage().instance().set(KEY, &num);
        }
    }
    /// Add two numbers using the `add` method of ContractA.
    pub fn add_with(env: Env, x: u32, y: Option<u32>) -> u32 {
        contract_b::new(&env).add_with(
            &contract_a::address(&env),
            &x,
            &y.unwrap_or_else(|| default_value(&env)),
        )
    }
}

smartdeploy_sdk::core_riff!();
```

We'll need to install `contract_b` as well

```bash
smartdeploy install contract_b
```

This also adds a `target/smartdeploy/contract_b/contract_id.txt` file with the contract id of the contract.
With the contract's id we can create client directly without the need to track down the contract id

```rs
contract_b::new(&env)
```

And we can access the address directly

```rs
contract_a::address(&env)
```

This is super handy for oracle contracts or any contract that you want developers to call in their contracts.

You'll also notice that this contract has an init method. Currently with the soroban CLI you cannot deploy and invoke a contract in one step. You can with smartdeploy!

Just like the awesome feature of the soroban CLI, smartdeploy generates a CLI for the contract that is to be deployed. You can access the possible methods with `--help`

```bash
smartdeploy deploy --published-name contract_c \
                   deployed-name c3pO \
                   --source default \ 
                   -- \
                   --help
```
```
smartdeploy deploy --published-name contract_c --deployed-name c3pO -- --help
Usage: c3pO [COMMAND]

Commands:
  add_with   
  owner_get  Returns the owner of the contract
  owner_set  Sets the owner of the contract. If one already set it transfers it to the new owner, if signed by owner.
  redeploy   Redeploy the contract to a wasm hash
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

```bash
smartdeploy deploy --published-name contract_c \
                   deployed-name c3pO \
                   --source default \ 
                   -- \
                   init --num 41
```

Now we can call the contract with `smartdeploy call` and leave off the `--y` argument, which should then default to `41`.

```bash
smartdeploy call c3pO -- add_with --x 1
42
```
This calls creates a cross contract call to `contract_b` and then it calls `contract_a`. All without having to manually reference any contract ids!


## Smartdeploy is also a Dapp!

Check it out here: [launch.smartdeploy.dev](https:://launch.smartdeploy.dev).

It current lets you see all published contracts:

<img width="1356" alt="image" src="https://github.com/TENK-DAO/smartdeploy/assets/1483244/f8ce3c56-e3df-47ec-98af-3ffc9236b5f1">


Deploy them:

<img width="1000" alt="image" src="https://github.com/TENK-DAO/smartdeploy/assets/1483244/388a53b2-e8e0-4d40-b265-9f06b5678671">

And see all deployed contracts

<img width="1369" alt="image" src="https://github.com/TENK-DAO/smartdeploy/assets/1483244/77dce042-7b49-40a5-aaec-d130a330e25a">

### Coming soon:
- Autogenerated init method input forms for when deploying like you can with the CLI. 
- Autogenerated forms for any deployed contract's methods (not just deployed smartdeploy)!
- Indexing will let dapp let you quickly search about contracts like [crates.io](https://crates.io).

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

