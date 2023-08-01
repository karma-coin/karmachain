
# Name
Name of the chain
```yaml
{
  "name": "Development"
  # ...
}
 ```

# Id
Id of the chain
```yaml
{
  # ...
  "id": "dev",
  # ...
}
```

# chainType
Type of the chain. Possible values are `Live`, `Development`, `Local`.
```yaml
{
  # ...
  "chainType": "Development",
  # ...
}
```

# bootNodes
Determine set of nodes to connect to on start up. Required in order to connect nodes into one network.
If no bootnode will be found, node will write a log and run as standalone chain.

Node identity can be found in node logs, for example:
```shell
> Local node identity is: 12D3KooWMDGwUWHY5BdYhy3bu1yfVFRbpUh4Z5jkTGizQvrfBq9e 
```

```yaml
{
  # ...
  "bootNodes": [
    "/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWH4svWxuSF8ZGUfSBgXLrKZmgqS8mbYfDL7zMpuqqJ2Jm"
  ],
  # ...
}
```

# telemetryEndpoints
Endpoint to send telemetry data such as block time, etc. Can be connected to [polkadot telemetry](https://telemetry.polkadot.io/#/0x91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3)

```yaml
{
  # ...
  "telemetryEndpoints": null,
  # ...
}
```

# protocolId

```yaml
{
  # ...
  "protocolId": null,
  # ...
}
```

# properties
Additional information for polkadot-ui. Used to properly display some information

```yaml
{
  # ...
  "properties": {
    "tokenDecimals": 0,
    "tokenSymbol": "KCent"
  },
  # ... 
}
```

# Genesis

## Runtime

### System
Just blob of code

```yaml
{
  # ...
  "code": "..."
  # ... 
}
```

### Babe

```yaml
{
  # ...
  "babe": {
    "authorities": [],
    "epochConfig": {
      "c": [
        1,
        4
      ],
    "allowed_slots": "PrimaryAndSecondaryVRFSlots"
    }
  },
  # ...
}
```

### Balances
Initial balances of accounts. 

```yaml
{
  # ...
  "balances": {
    "balances": [
      [
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        1000000000000000
      ],
      # ...
     ]
  },
  # ...
}
```


### transactionPayment
Determine transaction fee size

```yaml
{
  # ...
  "transactionPayment": {
    "multiplier": "1000000000000000000"
  }
  # ...
}
```

### Sudo
`AccountId` of sudo account

```yaml
{
  # ...
  "sudo": {
    "key": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
  }
  # ...
}
```

### Staking

```yaml
{
  # ...
  "staking": {
    # Target number of validators
    "validatorCount": 1,
    # Minimal number of validators
    "minimumValidatorCount": 1,
    # Validators that may never be slashed or forcibly kicked
    "invulnerables": [
      "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY"
    ],
    # Mode to handle era errors
    "forceEra": "NotForcing",
    "slashRewardFraction": 100000000,
    "canceledPayout": 0,
    "stakers": [
      [
        # `AccountId`
        "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY",
        # Controller `AccountId`
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        # Amount 
        2500000000000,
        # Role
        "Validator"
      ]
    ],
    "minNominatorBond": 0,
    "minValidatorBond": 0,
    "maxValidatorCount": null,
    "maxNominatorCount": null
  }
  # ...
}
```

### Nomination pools

#### Chain spec configuration

```yaml
"nominationPools": {
  # Minimum amount to bond to join a pool
  "minJoinBond": 1000000,
  # Minimum bond required to create a pool
  "minCreateBond": 1000000,
  # Maximum number of nomination pools that can exist
  "maxPools": 16,
  # Maximum number of members that may belong to pool
  "maxMembersPerPool": 32,
  # Maximum number of members that can exist in the system
  "maxMembers": 512,
  # The maximum commission that can be charged by a pool
  "globalMaxCommission": null
},
```

### Session
A session key is actually several keys kept together that provide the various
signing functions required by network authorities/validators in pursuit of their duties.

```yaml
{
  # ...
  "session": {
    "keys": [
      [
        "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY",
        "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY",
        {
          "babe": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
          "grandpa": "5FA9nQDVg267DEd8m1ZypXLBnvN7SFxYwV7ndqSYGiN9TTpu"
        }
      ]
    ]
  },
  # ...
}
```  

### Grandpa
Provides initial genesis initialization for `grandpa`, if empty `grandpa` will be initialized
through `on_genesis_session`

```yaml
{
  # ...
  "grandpa": {
    "authorities": []
  },
  # ...
}
```

### Identity
```yaml
{
  # ...
  "identity": {
    # AccountId's of phone verifier accounts
    "phoneVerifiers": [
      "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
    ]
  },
  # ...
}
```

### Appreciation
```yaml
{
  # ...
  "appreciation": {
    # List of character traits with id, name and emoji
    "chatTraits": [
      [
        1,
        "a Karma Grower",
        "💚"
      ],
      # ...
    ],
    "noCharTraitId": 0,
    "signupCharTraitId": 1,
    "spenderCharTraitId": 2,
    "ambassadorCharTraitId": 41,
    "noCommunityId": 0,
    # Communities with information about
    "communities": [
      [
        1,
        "Grateful Giraffes",
        "A global community of of leaders that come together for powerful wellness experiences",
        "🦒",
        "https://www.gratefulgiraffes.com",
        "https://twitter.com/TheGratefulDAO",
        "https://www.instagram.com/gratefulgiraffes",
        "",
        "https://discord.gg/7FMTXavy8N",
        # List of available character traits
        [10,4,3,11,15,18,39,42,60],
        true
      ],
      # ...
    ],
    # Community membership of Accounts [AccountId, CommunityId, CommunityRole]
    # possible values for `CommunityRoles`: `Admin`, `Member`
     
    "communityMembership": [
      [
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        1,
        "Admin"
      ],
      # ...
    ],
  }
  # ...
}
```
