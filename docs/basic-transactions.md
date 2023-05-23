# Basic transactions
This document details the transactions supported by Karmachain 2.0 Testnest1 (TN1).

## Creating an account

1. Install the [polkadot.js chrome extension](https://chrome.google.com/webstore/detail/polkadot%7Bjs%7D-extension/mopnmbcafieddcagagdcbnhejhlodfdd).
2. Connect to a Karmachain testnet via polkadot.js. TN1 is available [here](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Ftestnet.karmaco.in%2Ftestnet%2Fws#/explorer)
3. Follow the instructions in this guide. [Create an Account using Polkadot JS Extension](https://www.youtube.com/watch?v=sy7lvAqyzkY)

## Signup

To register you account on-chain, go to `Developer` > `Extrinsics`.
Next, choose from drop down menu `identity` and `newUser`. 

![newUserTx](./images/basic-transactions/new-user-tx.png)

- using the selected account - account which will sign transaction and which you want to register, should match the `accountId` parameter.
- `accountId` - should be same as account used for signing tx
- `username` - requested username
- `phoneNumber` - requested phone number in international format. e.g. 972549805381

Press "Submit Transaction" to confirm sending transaction. After a few seconds, you should see an ExtrinsicSuccess message.

## Appreciation

To register you account onchain you need to go to `Developer` > `Extrinsics`.
Next you need to choose from drop down menu `appreciation` and `appreciation`. 

![appreciation](./images/basic-transactions/appreciation.png)

* using the selected account - account which send appreciation (your account)
* to - maybe AccountId, username or phone number of receiver account
* amount - amount of tip in `KCents`to send with appreciation
* communityId - for TN1 make no sense, because no community are configured in genesis
* charTraitId - id of char trait (for example 6 - `Awesome`, 7 - `Smart`)

Once everything is filled in properly, click "Submit Transaction". After a few seconds, you should see an ExtrinsicSuccess message.

## Available char traits

Go to `Developer` > `Chain state`. Here choose `appreciation` and `charTrait` and click on "+".
You should see list of available character trait below

![characterTraits](./images/basic-transactions/char-traits.png)

## Account balance

After registration, you can see you account balance in `Accounts` tab.

![accountBalance](./images/basic-transactions/account-balance.png)

## Account information

Go to `Developer` > `Chain state`. Here choose `identity` and `identityOf` and click on "+".
You should see basic information about account.

![accoutnInfo](./images/basic-transactions/account-info.png)
