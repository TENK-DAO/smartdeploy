# SmartDeploy

A framework for publishing, deploying, and upgrading Soroban smart contracts.

Uses [Loam-SDK](https://github.com/loambuild/loam-sdk)

## Publishing

Currently smart contracts are `installed` to the network allowing them to be deployed using the hash of the installed contract. However, unless you are the author of the smart contract you won't know much about which hashes correspond to which contracts.  Furthermore, there is no notion of a version of each hash so you would find many contracts with almost identical implementations.



## Trying it out

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

Currently the smartdeploy CLI has a `call` subcommand which lets you use a deployed contract's name when invoking it.

```bash
cargo run -- call smartdeploy -- --help
```

Can list out deployed_contracts

```bash
cargo run -- call smartdeploy -- list_deployed_contracts

```