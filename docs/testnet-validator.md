# Running a Testnet Validator Node

This guide describes how you to run validator node on karmachain 2.0 Testnet 1 (TN1).

## Setup

Running a karmachain node in validator mode requires:
* [Docker](https://docs.docker.com/engine/install/)
* [Git](https://github.com/git-guides/install-git)

- To start validating, you need to create accounts. You can use any Substrate compatible wallet but we recommend you'll use the [polkadot.js extension](https://chrome.google.com/webstore/detail/polkadot%7Bjs%7D-extension/mopnmbcafieddcagagdcbnhejhlodfdd).
- Reffer to this guide [Create an Account using Polkadot JS Extension](https://www.youtube.com/watch?v=sy7lvAqyzkY)

----

## Requesting tokens for your bond

[//]: # (TODO) 

## Downloading karmachain source files

TODO: Link to TN1 github release on github as validator should clone the release's code.

Downloading karmachain from GitHub.

```bash
git clone https://github.com/karma-coin/karmachain
cd karmachain
```

## Preparing your node

Build a docker image of your node.

```bash
docker build . --tag karmachain-node
```

## Running your node

```bash
docker run \
	--name karmachain-node \
	--rm \
	--mount source=chain-data,target=/chain-data \
	-p 30333:30333 -p 9944:9944 -p 9933:9933 \
	karmachain-node \
		--base-path /chain-data \
		--chain chain-spec/chainSpecTN1.json \
		--port 30333 \
		--ws-port 9944 \
		--unsafe-ws-external
		--rpc-port 9933 \
		--rpc-cors all \
		--rpc-methods Unsafe \
		--validator \
		--name <your-node-name> \
		--bootnodes /dns/testnet.karmaco.in/tcp/30333/p2p/12D3KooWFgrbXqQE1kp3ytTGTsgsVVFBp5P3TGYyGa2KaVs9nQta
```

## Bond tokens

It is highly recommended that you make your controller and stash accounts be two separate accounts. For this, you will create two accounts and make sure each of them have at least enough funds to pay the fees for making transactions. Keep most of your funds in the stash account since it is meant to be the custodian of your staking funds.

Make sure not to bond all your KCoin balance since you will be unable to pay transaction fees from your bonded balance.

Follow these steps to set up your validator.

- Bond the KCoin of the Stash account. These KCoin will be put at stake for the security of the network and can be slashed.
- Select the Controller. This is the account that will decide when to start or stop validating.
First, go to the Staking section. Click on `Account Actions`, and then the `+ Stash` button.`
![bond](./images/run-a-validator/bond.png)

- `Stash account` - Select your Stash account. In this example, we will bond 1 DOT, where the minimum bonding amount is 1. Make sure that your Stash account contains at least this much. You can, of course, stake more than this.
- `Controller account` - Select the Controller account created earlier. This account will also need a small amount of DOT in order to start and stop validating.
- `Value bonded` - How much DOT from the Stash account you want to bond/stake. Note that you do not need to bond all of the DOT in that account. Also note that you can always bond more DOT later. However, withdrawing any bonded amount requires the duration of the unbonding period. On Kusama, the unbonding period is 7 days. On Polkadot, the planned unbonding period is 28 days.
- `Payment destination` - The account where the rewards from validating are sent. More info here. Starting with runtime version v23 natively included in client version 0.9.3, payouts can go to any custom address. If you'd like to redirect payments to an account that is neither the controller nor the stash account, set one up. Note that it is extremely unsafe to set an exchange address as the recipient of the staking rewards.

Once everything is filled in properly, click `Bond` and sign the transaction with your Stash account. You should see an ExtrinsicSuccess message in few seconds.

Your bonded account will available under `Stashes`. 

You should now see a new card with all your accounts (note: you may need to refresh the screen). The bonded amount on the right corresponds to the funds bonded by the Stash account.

![bond](./images/run-a-validator/stash.png)

## Generating session keys

Run this command on the same machine (while the node is running with the default WS RPC port configured):

```bash
echo '{"id":1,"jsonrpc":"2.0","method":"author_rotateKeys","params":[]}' | websocat -n1 -B 99999999 ws://127.0.0.1:9944
```

The output will have a hex-encoded `result` field. Save this result for a later step.

## Submitting a `setKeys` Transaction

You need to tell the chain your Session keys by signing and submitting an extrinsic. This is what associates your validator with your Controller account.

Go to `Staking > Account Actions`, and click `Set Session Key` on the bonding account you generated earlier. Enter the output from author_rotateKeys in the field and click `Set Session Key`.

![setKeys](./images/run-a-validator/set-session-key.png)

## Validating

The `reward commission percentage` is the commission percentage that you can declare against your validator's rewards. This is the rate that your validator will be commissioned with.

Payment preferences - You can specify the percentage of the rewards that will get paid to you. The remaining will be split among your nominators.

You can also determine if you would like to receive nominations with the `allows new nominations` option.

![validate](./images/run-a-validator/validate.png)

Click `Validate`.

Navigate to the `Staking` tab, you will see a list of active validators currently running on the network. At the top of the page, it shows the number of validator slots that are available as well as the number of nodes that have signaled their intention to be a validator. You can go to the `Waiting` tab to double check to see whether your node is listed there.

The validator set is refreshed every era. In the next era, if there is a slot available and your node is selected to join the validator set, your node will become an active validator. Until then, it will remain in the waiting queue. If your validator is not selected to become part of the validator set, it will remain in the waiting queue until it is. There is no need to re-start if you are not selected for the validator set in a particular era.
