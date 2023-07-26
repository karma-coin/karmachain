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

```shell
 cargo run -- --chain=chain-spec/chainSpec.json --alice --validator
```

#### Running with the verifier role 

Use the following command to launch a node that supports sign-up verification.

```sh
cargo run --release -- --dev --verifier --bypass-token="dummy" --auth-dst="https://localhost:8080"
```

Next, use the following command to add verifier keys to the node.

```sh
curl --location 'http://localhost:9944/' \
--header 'Content-Type: application/json' \
--data '{
    "id": 1,
    "jsonrpc": "2.0",
    "method": "author_insertKey",
    "params": {
        "key_type": "Veri",
        "suri": "//Alice",
        "public": "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
    }
}'
```

#### Enabling the offchain worker feature

Use the following command to launch node with an enabled offchain worker that distributes karma rewards.

```shell
cargo run --release -- --dev --offchain-worker always
```

Next, run the following command to insert the offchain worker keys to the chain.

```sha
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

```sh
cargo run --release -- --dev --verifier --bypass-token="dummy" --auth-dst="https://localhost:8080" --offchain-worker always --rpc-methods unsafe
```

Remember to insert the two keys to the chain once it is running.

### Building

Use the following command to build a node without launching it:

```sh
cargo build --release
```

### Reading the embedded docs

Once the node has been built, use the following command can be used to explore all parameters and
subcommands.

```sh
./target/release/karmachain-node -h
```

### Single-Node Development Chain

This following command will start the single-node dev chain with non-persistent state.

```bash
./target/release/karmachain-node --dev
```

The following command purges the dev chain's state.

```bash
./target/release/karmachain-node purge-chain --dev
```

Use the following command to run a dev chain with detailed logging.

```bash
RUST_BACKTRACE=1 ./target/release/karmachain-node -ldebug --dev
```

> Development chain means that the state of our chain will be in a tmp folder while the nodes are
> running. Also, **alice** account will be authority and sudo account as declared in the
> [genesis state](https://github.com/substrate-developer-hub/substrate-karmachain-node/blob/main/node/src/chain_spec.rs#L49).
> At the same time the following accounts will be pre-funded:
> - Alice
> - Bob
> - Alice//stash
> - Bob//stash

In case of being interested in maintaining the chain' state between runs a base path must be added
so the db can be stored in the provided folder instead of a temporal one. We could use this folder
to store different chain databases, as a different folder will be created per different chain that
is ran. The following commands shows how to use a newly created folder as our db base path.

```bash
// Create a folder to use as the db base path
$ mkdir my-chain-state

// Use of that folder to store the chain state
$ ./target/release/karmachain-node --dev --base-path ./my-chain-state/

// Check the folder structure created inside the base path after running the chain
$ ls ./my-chain-state
chains
$ ls ./my-chain-state/chains/
dev
$ ls ./my-chain-state/chains/dev
db keystore network
```


### Connecting with Polkadot-JS Apps and other Front-ends

Once the node is running locally, you can connect it with **Polkadot-JS Apps** front-end to interact with your chain. 
This url will run the polkadot-js ui in your browser for your local node:

[https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) connecting the Apps to your
local node template.

### Multi-Node Local Testnet

If you want to see the multi-node consensus algorithm in action, refer to our
[Simulate a network tutorial](https://docs.substrate.io/tutorials/get-started/simulate-network/).


## Template Structure

A Substrate project such as this consists of a number of components that are spread across a few
directories.

### Node

A blockchain node is an application that allows users to participate in a blockchain network.
Substrate-based blockchain nodes expose a number of capabilities:

- Networking: Substrate nodes use the [`libp2p`](https://libp2p.io/) networking stack to allow the
  nodes in the network to communicate with one another.
- Consensus: Blockchains must have a way to come to
  [consensus](https://docs.substrate.io/main-docs/fundamentals/consensus/) on the state of the
  network. Substrate makes it possible to supply custom consensus engines and also ships with
  several consensus mechanisms that have been built on top of
  [Web3 Foundation research](https://research.web3.foundation/en/latest/polkadot/NPoS/index.html).
- RPC Server: A remote procedure call (RPC) server is used to interact with Substrate nodes.

There are several files in the `node` directory - take special note of the following:

- [`chain_spec.rs`](./node/src/chain_spec.rs): A
  [chain specification](https://docs.substrate.io/main-docs/build/chain-spec/) is a
  source code file that defines a Substrate chain's initial (genesis) state. Chain specifications
  are useful for development and testing, and critical when architecting the launch of a
  production chain. Take note of the `development_config` and `testnet_genesis` functions, which
  are used to define the genesis state for the local development chain configuration. These
  functions identify some
  [well-known accounts](https://docs.substrate.io/reference/command-line-tools/subkey/)
  and use them to configure the blockchain's initial state.
- [`service.rs`](./node/src/service.rs): This file defines the node implementation. Take note of
  the libraries that this file imports and the names of the functions it invokes. In particular,
  there are references to consensus-related topics, such as the
  [block finalization and forks](https://docs.substrate.io/main-docs/fundamentals/consensus/#finalization-and-forks)
  and other [consensus mechanisms](https://docs.substrate.io/main-docs/fundamentals/consensus/#default-consensus-models)
  such as Aura for block authoring and GRANDPA for finality.

After the node has been [built](#build), refer to the embedded documentation to learn more about the
capabilities and configuration parameters that it exposes:

```shell
./target/release/karmachain-node --help
```

### Runtime

In Substrate, the terms
"runtime" and "state transition function"
are analogous - they refer to the core logic of the blockchain that is responsible for validating
blocks and executing the state changes they define. The Substrate project in this repository uses
[FRAME](https://docs.substrate.io/main-docs/fundamentals/runtime-intro/#frame) to construct a
blockchain runtime. FRAME allows runtime developers to declare domain-specific logic in modules
called "pallets". At the heart of FRAME is a helpful
[macro language](https://docs.substrate.io/reference/frame-macros/) that makes it easy to
create pallets and flexibly compose them to create blockchains that can address
[a variety of needs](https://substrate.io/ecosystem/projects/).

Review the [FRAME runtime implementation](./runtime/src/lib.rs) included in this template and note
the following:

- This file configures several pallets to include in the runtime. Each pallet configuration is
  defined by a code block that begins with `impl $PALLET_NAME::Config for Runtime`.
- The pallets are composed into a single runtime by way of the
  [`construct_runtime!`](https://crates.parity.io/frame_support/macro.construct_runtime.html)
  macro, which is part of the core
  FRAME Support [system](https://docs.substrate.io/reference/frame-pallets/#system-pallets) library.

### Pallets

The runtime in this project is constructed using many FRAME pallets that ship with the
[core Substrate repository](https://github.com/paritytech/substrate/tree/master/frame) and a
template pallet that is [defined in the `pallets`](./pallets/template/src/lib.rs) directory.

A FRAME pallet is compromised of a number of blockchain primitives:

- Storage: FRAME defines a rich set of powerful
  [storage abstractions](https://docs.substrate.io/main-docs/build/runtime-storage/) that makes
  it easy to use Substrate's efficient key-value database to manage the evolving state of a
  blockchain.
- Dispatchables: FRAME pallets define special types of functions that can be invoked (dispatched)
  from outside of the runtime in order to update its state.
- Events: Substrate uses [events and errors](https://docs.substrate.io/main-docs/build/events-errors/)
  to notify users of important changes in the runtime.
- Errors: When a dispatchable fails, it returns an error.
- Config: The `Config` configuration interface is used to define the types and parameters upon
  which a FRAME pallet depends.

### Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain.

```bash
./scripts/docker_run.sh
```

This command will firstly compile your code, and then start a local development network. You can
also replace the default command
(`cargo build --release && ./target/release/karmachain-node --dev --ws-external`)
by appending your own. A few useful ones are as follow.

```bash
# Run Substrate node without re-compiling
./scripts/docker_run.sh ./target/release/karmachain-node --dev --ws-external

# Purge the local dev chain
./scripts/docker_run.sh ./target/release/karmachain-node purge-chain --dev

# Check whether the code is compilable
./scripts/docker_run.sh cargo check
```
