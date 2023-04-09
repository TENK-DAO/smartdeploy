# SmartDeploy

A framework for publishing, deploying, and upgrading Soroban smart contracts.

## Publishing

Currently smart contracts are `installed` to the network allowing them to be deployed using the hash of the installed contract. However, unless you are the author of the smart contract you won't know much about which hashes correspond to which contracts.  Furthermore, there is no notion of a version of each hash so you would find many contracts with almost identical implementations.



## Usage

At the moment only Unix OSes are supported in the follow steps. 

The smart deploy contract is deployed to `futurenet`!

First you'll need the CLI, currently needs to use a special branch to handle result types.

```bash
cargo install soroban-cli --git https://github.com/ahalabs/soroban-tools --rev 1b13fdb89f43bcbb8646fc8e3642264873a2b2fb
```

Next you'll need to create an identity.

```bash
soroban config identity generate --global default
```

Then we need to fund the account:

```bash
curl "https://friendbot-futurenet.stellar.org/?addr=$(soroban config identity address --global default)"
```


Next you need to add futurenet as a network:

```bash
soroban config network add --global \
                           --rpc-url https://rpc-futurenet.stellar.org:443/soroban/rpc \
                           --network-passphrase "Test SDF Future Network ; October 2022" \
                           futurenet
```

## Invoking the contract

Currently SmartDeploy is deployed to `35341e1989b452ebba2119754f5b02b043a4838477902718073770d8354dca63`.

To see if it's working let's look at the contract's help doc.

```bash
soroban contract invoke --id 35341e1989b452ebba2119754f5b02b043a4838477902718073770d8354dca63 \
                        --network futurenet \
                        --source default \
                        -- --help
```

You should see something like

```
Usage: 35341e1989b452ebba2119754f5b02b043a4838477902718073770d8354dca63 [COMMAND]

Commands:
  register_name   Register a contract name to allow publishing.
  publish         Publish a contract.
                  Currently a contract's version is a `u32` and publishing will increment it.
                  If no repo is provided, then the previously published binary's repo will be used. 
                  If it's the first time then it will be empty.
  fetch           Fetch the hash for a given contract_name.
                  If version is not provided, it is the most recent version.
  fetch_metadata  Fetch metadata for a given contract_name.
                  If version is not provided, it is the most recent version.
  deploy          Deploys a new published contract returning the deployed contract's id.
  help            Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## Creating an alias

That command was a bit long and this behaves like its own CLI command so let's make an alias for it!

```bash
alias smartdeploy="soroban contract invoke --id 35341e1989b452ebba2119754f5b02b043a4838477902718073770d8354dca63 --network futurenet --source default --"
```

Now it's just:

```bash
smartdeploy --help
```

Add this alias to your .bashrc or .zshrc to have this added when you launch your shell.

The hello world example is published so let's fetch the metadata. First let's see the help doc:

```bash
smartdeploy fetch_metadata --help
```

Should return

```
Fetch metadata for a given contract_name.
If version is not provided, it is the most recent version.

Usage: fetch_metadata [OPTIONS]

Options:
      --version <Option<u32>>
          Example:
            --version 1

      --contract_name <String>
          Example:
            --contract_name '"hello world"'

  -h, --help
          Print help (see a summary with '-h')
```

The contract is named `hello_world`.

```bash
smartdeploy fetch_metadata --contract_name hello_world
```

Should return `"{"repo":"https://github.com/AhaLabs/soroban-examples/tree/0d977e1b56d3b7007855f6557248e17f37081699/hello_world"}"`.

This allows you to audit the code before deciding to deploy it.

Next we can fetch the hash of the contract with `fetch`:

```bash
smartdeploy fetch --contract_name hello_world
```

should return: `"6c453071976d247e6c8552034ba24a7b6ba95d599eb216d72a15bf8bd7176a8a"`

We could now copy the hash and manually deploy it:

```bash
soroban contract deploy --wasm-hash 6c453071976d247e6c8552034ba24a7b6ba95d599eb216d72a15bf8bd7176a8a --identity default --network futurenet --salt "00"`
```

But SmartDeploy has deploy in the name!

```bash
smartdeploy deploy --help
```

```
Deploys a new published contract returning the deployed contract's id.
If no salt provided it will use the current sequence number.

Usage: deploy [OPTIONS]

Options:
      --salt <Option<hex_bytes>>
          Example:
            --salt beefface123

      --contract_name <String>
          Example:
            --contract_name '"hello world"'

      --version <Option<u32>>
          Example:
            --version 1

  -h, --help
          Print help (see a summary with '-h')
```

So let's deploy a hello world contract.

```bash
smartdeploy deploy --contract_name hello_world
```

This should return a contract id.  One such example is c9ccbd0e91bb8ea123b323b22379f76024a4ff4cd6b1541aa4f68fa531d65af6.

You can test to see if it's working with checking for the help docs.

```bash
soroban contract invoke  --network futurenet --id c9ccbd0e91bb8ea123b323b22379f76024a4ff4cd6b1541aa4f68fa531d65af6 -- --help
```

Should return:
```
Usage: c9ccbd0e91bb8ea123b323b22379f76024a4ff4cd6b1541aa4f68fa531d65af6 [COMMAND]

Commands:
  hello  
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help

```

Finally, let's call the deployed contract!

```bash
soroban contract invoke --id c9ccbd0e91bb8ea123b323b22379f76024a4ff4cd6b1541aa4f68fa531d65af6 --network futurenet -- hello --to world
```

## Deploying your own version of smartdeploy

To make it eaiser you can use the provided scripts to set things up:

```bash
./deploy.sh && source_me.sh
```

This will deploy the initial smart deploy contract and then create an alias for smartdeploy
