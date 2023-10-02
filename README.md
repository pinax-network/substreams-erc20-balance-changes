# Substreams ERC20 Balance Changes
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)


This is a fork of the original github https://github.com/streamingfast/substreams-erc20-balance-changes

This fork have InitialBalance = 0 to allow more flexibility on blockchain.
The goal of this Substreams project is to extract all ERC20/ERC721 transfers from Ethereum events for the full chain.

The `map_balance_changes` module will output messages of type `erc20.types.v1.BalanceChange` defined by: 

```proto
message BalanceChange {
string contract = 1;
string owner = 2;
string old_balance = 3;
string new_balance = 4;

string transaction = 5;
string storage_key = 6;
string call_index = 7;

string transfer_value = 8;

BalanceChangeType change_type = 9;
}
```

## Known issues:

Tracking balance changes requires tracking state changes on chain.  However, different contracts have different ways of storing balances.  

We have implemented the following strategies for tracking balance changes:

### Type 1: Storage change is in the same call as the transfer

example:
https://etherscan.io/tx/0xf490320cff087d82747fcb0e6ed797f899ff887bcd15162933ea051c94c596ea#eventlog

Here is the relevant section from the Firehose block for this transaction:

```json
  {
    "index": 1,
    "callType": "CALL",
    "caller": "45225d3536ac02928f16071ab05066bce95c2cd5",
    "address": "dac17f958d2ee523a2206206994597c13d831ec7",
    "gasLimit": "104810",
    "gasConsumed": "41601",
    "input": "a9059cbb000000000000000000000000caf7ce56598e8588c9bf471e08b53e8a8d9541b300000000000000000000000000000000000000000000000000000000c84cfb23",
    "executedCode": true,
    "keccakPreimages": {
      "3cacfdf5e3a27369ea8efd976a1d467ed2ce08586e22e7366aa4d82943439fa7": "00000000000000000000000045225d3536ac02928f16071ab05066bce95c2cd50000000000000000000000000000000000000000000000000000000000000006",
      "d116b96c704431079cf20227b36d5f02fea21af673489300fe1ae3229e0c0d74": "000000000000000000000000caf7ce56598e8588c9bf471e08b53e8a8d9541b30000000000000000000000000000000000000000000000000000000000000002",
      "ec2750738b8e716c607ab9d95b2d48bc4d6b8eacc278d1510c490ab2c788884d": "00000000000000000000000045225d3536ac02928f16071ab05066bce95c2cd50000000000000000000000000000000000000000000000000000000000000002"
    },
    "storageChanges": [
      {
        "address": "dac17f958d2ee523a2206206994597c13d831ec7",
        "key": "ec2750738b8e716c607ab9d95b2d48bc4d6b8eacc278d1510c490ab2c788884d",
        "oldValue": "000000000000000000000000000000000000000000000000000000355ed4c80e",
        "newValue": "000000000000000000000000000000000000000000000000000000349687cceb",
        "ordinal": "1154"
      },
      {
        "address": "dac17f958d2ee523a2206206994597c13d831ec7",
        "key": "d116b96c704431079cf20227b36d5f02fea21af673489300fe1ae3229e0c0d74",
        "oldValue": "0000000000000000000000000000000000000000000000000000000000000000",
        "newValue": "00000000000000000000000000000000000000000000000000000000c84cfb23",
        "ordinal": "1155"
      }
    ],
    "logs": [
      {
        "address": "dac17f958d2ee523a2206206994597c13d831ec7",
        "topics": [
          "ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
          "00000000000000000000000045225d3536ac02928f16071ab05066bce95c2cd5",
          "000000000000000000000000caf7ce56598e8588c9bf471e08b53e8a8d9541b3"
        ],
        "data": "00000000000000000000000000000000000000000000000000000000c84cfb23",
        "blockIndex": 49,
        "ordinal": "1157"
      }
    ]
  }
```

The correctness of the `old_balance` and `new_balance` values in this case is easily determined.

These types of transfers will result in a BalanceChange message with `change_type` set to `TYPE_1`.

### Type 2: Storage change is in a different call than the transfer

In this case, the Transfer but this results in storage changes in different child calls, where often the amount sent will be split to multiple accounts.

example:
https://etherscan.io/tx/0x5a31fb5d3f5bbb95023438f017ad6cd501ce70e445f31c2660c784e5a7eb5d83#eventlog

```json
{
  "index": 4,
  "logs": [
    {
      "address": "225bc3affc1da39bd3cb2100c74a41c62310d1e1",
      "topics": [
        "ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
        "000000000000000000000000541f52216afdfeef6851eea9772b17d3cafd9438",
        "000000000000000000000000b30acc73814d34941d71a1dfa5c2a5e618a062fe"
      ],
      "data": "0000000000000000000000000000000000000000000000000000000000451f50",
      "index": 2,
      "blockIndex": 2,
      "ordinal": "68"
    }
  ]
},
{
  "index": 10,
  "keccakPreimages": {
    "c0309ad5a3dcaf0d46cab6102b742e914f7ff8447190f509bf80a0f0b60c452c": "000000000000000000000000b30acc73814d34941d71a1dfa5c2a5e618a062fe0000000000000000000000000000000000000000000000000000000000000002"
  },
  "storageChanges": [
    {
      "address": "276c5c6ca8507ed7bac085fc9b9521f4f54b58d3",
      "key": "c0309ad5a3dcaf0d46cab6102b742e914f7ff8447190f509bf80a0f0b60c452c",
      "oldValue": "000000000000000000000000000000000000000000000000000000012d03e73e",
      "newValue": "000000000000000000000000000000000000000000000000000000012d48915e",
      "ordinal": "61"
    }
  ],
}
```

In this example, the Transfer call is made in call index 4.  Then in the subsequent child calls, the transfer of 4,530,000 tokens is split into transfers by the contract:  One transfer of 4,500,000 to the original receiver and a transfer of 30,000 to another address.  Some work is required to track the balance changes in this case.

These types of transfers will result in a BalanceChange message with `change_type` set to `TYPE_2`.

### Others

There are other types of transfers where the balance of the accounts before and after is not clear.

example:
https://etherscan.io/tx/0x5a31fb5d3f5bbb95023438f017ad6cd501ce70e445f31c2660c784e5a7eb5d83#eventlog

These transfers will result in a BalanceChange message with `change_type` set to `null`.

These should currently be discarded by the consumer of the substream as they are guaranteed to be incorrect.

### Notes

As of block 18005744, the sum of type 1 and type 2 matches accounts for approximately 96.7% of the total balance changes.

```json
{
  "type0Count": "88809770",
  "type1Count": "2608546600",
  "type2Count": "5195308",
  "totalCount": "2702551678",
  "validRate": "0.9671385488303694890529305171717793142603506581308747872905614795055919001005685856860791544131205324",
  "blockNumber": "18005744"
}
```

## Running

### Generate protos

```bash
make protogen
```

### Build substreams

```bash
make build
```

### Build spkg

```bash
make pack
```

### Example run

```bash
substreams gui ./erc20-balance-changes-v0.0.4.spkg map_valid_balance_changes -e mainnet.eth.streamingfast.io:443 -s 17000000 -t +10 --production-mode
```
