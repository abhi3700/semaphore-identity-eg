# CLI commands

Listed down are some commonly used commands for the CLI other than build, deploy, test in this case.

> - Mostly chain oriented rather than smart contracts.
> - Before running any command, ensure the deployer public key and RPC URL are set as environment variables using `$ source .env`.

## Get balance

```sh
$ cast balance $DEPLOYER_PUBLIC_KEY --rpc-url $SUBSPACE_EVM_RPC_URL
```

## Get nonce

```sh
$ cast nonce $DEPLOYER_PUBLIC_KEY --rpc-url $SUBSPACE_EVM_RPC_URL
```

## Get contract state

```sh
$ cast call $COUNTER "number()" --rpc-url $SUBSPACE_EVM_RPC_URL
0x00000000000000000000000000000000000000000000000000000000000038eb
```

> This value is in hex, convert it to decimal to get the actual value.

```sh
$ cast --to-dec 38EB -i 16
14571
```

## Send transaction

Contract: Semaphore

### createGroup

```
Sighash: 9c112141
Function Signature: createGroup(uint256,uint256,address)
contract: 0x6F53B339987f2B7347B889a3FEFc9d6793ACCcdA
```

```sh
$ cast send $SEMAPHORE_CONTRACT_ADDRESS "createGroup(uint256,uint256,address)" 3154 30 $DEPLOYER_PUBLIC_KEY --private-key $DEPLOYER_PRIVATE_KEY --rpc-url $SEPOLIA_RPC_URL
```

```
tx url: https://sepolia.etherscan.io/tx/0x49f850843be2dc813322a1938142cdc2d6e58dbe00e41b854b32e4ebe2e325f5
```

<!-- TODO: Add more results -->
