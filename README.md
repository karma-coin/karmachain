# Karmachain Node

A Karmachain node implemented with the Substrate framework.

## Getting Started

Follow the steps below to get started with the Node Template, or get it up and running right from
your browser in just a few clicks using
the [Substrate Playground](https://docs.substrate.io/playground/) :hammer_and_wrench:

### Using Nix

Install [nix](https://nixos.org/) and optionally [direnv](https://github.com/direnv/direnv) and
[lorri](https://github.com/nix-community/lorri) for a fully plug and play experience for setting up
the development environment. To get all the correct dependencies activate direnv `direnv allow` and
lorri `lorri shell`.

### Setting up Rust

First, complete the [basic Rust setup instructions](./docs/rust-setup.md).

### Installing the Protobuf Compiler

Debian / Ubunutu:

```sh
sudo apt install protobuf-compiler
```

### Running

Use Rust's native `cargo` command to build and launch the template node:

```sh
cargo run --release -- --dev
```

#### Build chainspec with backup data 

```sh
cargo run -- build-spec --chain dev:path/to/file.json > chain-spec/chainSpec.json
```

#### Running with a chainspec


```sh
cargo run -- build-spec --dev > chain-spec/chainSpec.json
```

This uses one of the provided chain specs. 
You can modify `chainSpec.json` to change blockchain genesis params.
 * balances - specify genesis balance for specific account
 * sudo - set sudo account
 * identity - set phone verifiers account
 * appreciation - set character traits, communities etc

Use the following command to run local node in dev mode with Alice's session keys as validator.

```sh
 cargo run -- --chain=chain-spec/chainSpec.json --alice --validator
```

#### Enabling the offchain worker feature

Use the following command to launch node with an enabled offchain worker that distributes karma rewards.

```sh
cargo run --release -- --dev --offchain-worker always
```

Next, run the following command to insert the offchain worker keys to the chain.

```sh
curl --location 'http://localhost:9944/' \
--header 'Content-Type: application/json' \
--data '{
    "id": 1,
    "jsonrpc": "2.0",
    "method": "author_insertKey",
    "params": {
        "key_type": "rewa",
        "suri": "//Alice",
        "public": "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
    }
}'
```

### Running a node for tests

Running a node for tests and integration purposes requires both verifier and offchain features enabled.
Use the following command.

```console
cargo run --release --features dev -- --dev --verifier --bypass-token="dummy" --auth-dst="https://localhost:8080" --offchain-worker always --rpc-methods unsafe
```

Remember to insert the two keys to the chain once it is running.

### Building

Use the following command to build a node without launching it:

```sh
cargo build --release
```

### Interacting
Run a dev node and connect polkadot.js/apps to it using this url: https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer


# Sudo 

## Overview

The Sudo module allows for a single account (called the "sudo key")
to execute dispatchable functions that require a `Root` call
or designate a new account to replace them as the sudo key.
Only one account can be the sudo key at a time.

## Interface

### Dispatchable Functions

Only the sudo key can call the dispatchable functions from the Sudo module.

* `sudo` - Make a `Root` call to a dispatchable function.
* `set_key` - Assign a new account to be the sudo key.

## Genesis Config

The Sudo module depends on the `GenesisConfig`.
You need to set an initial superuser account as the sudo `key`.


# Treasury

The Treasury pallet provides a "pot" of funds that can be managed by stakeholders in the system and
a structure for making spending proposals from this pot.

## Overview

The Treasury Pallet itself provides the pot to store funds, and a means for stakeholders to propose,
approve, and deny expenditures. The chain will need to provide a method (e.g.inflation, fees) for
collecting funds.

By way of example, the Council could vote to fund the Treasury with a portion of the block reward
and use the funds to pay developers.

### Terminology

- **Proposal:** A suggestion to allocate funds from the pot to a beneficiary.
- **Beneficiary:** An account who will receive the funds from a proposal if the proposal is
  approved.
- **Deposit:** Funds that a proposer must lock when making a proposal. The deposit will be returned
  or slashed if the proposal is approved or rejected respectively.
- **Pot:** Unspent funds accumulated by the treasury pallet.

## Interface

### Dispatchable Functions

General spending/proposal protocol:
- `propose_spend` - Make a spending proposal and stake the required deposit.
- `reject_proposal` - Reject a proposal, slashing the deposit, can be called only with `Sudo`.
- `approve_proposal` - Accept the proposal, returning the deposit, can be called only with `Sudo`.
- `spend` - Make a spending proposal and immediately approve it, can be called only with `Sudo`.
