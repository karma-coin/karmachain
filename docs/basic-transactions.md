# Basic transaction

## Creating account

Also, you need and some prepared you account [polkadot.js extension](https://chrome.google.com/webstore/detail/polkadot%7Bjs%7D-extension/mopnmbcafieddcagagdcbnhejhlodfdd).
[How to create account.](https://www.youtube.com/watch?v=sy7lvAqyzkY)

## Signup

To register you account onchain you need to go to `Developer` > `Extrinsics`.
Next you need to choose from drop down menu `identity` and `newUser`. 

![newUserTx](./images/basic-transactions/new-user-tx.png)

* using the selected account - account which will sign transaction and which you want to register, should match `accountId` parameter
* accountId - should be same as account used for signing tx
* username - requested username
* phoneNumber - requested phone number

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