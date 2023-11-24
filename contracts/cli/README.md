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

Addresses (on networks): [deployments](../deployments.json)

### createGroup

```sh
$ cast send $SEMAPHORE_CONTRACT_ADDRESS "createGroup(uint256,uint256,address)" 3111 20 $DEPLOYER_PUBLIC_KEY --private-key $DEPLOYER_PRIVATE_KEY --rpc-url $SEPOLIA_RPC_URL
```

```
tx url: https://sepolia.etherscan.io/tx/0x5b011c511d6d3ae6a533eba97515cfdd0d22a71538da65592adde2051fa7bae6
```

### createGroup (with duration to verify proof)

```sh
$ cast send $SEMAPHORE_CONTRACT_ADDRESS "createGroup(uint256,uint256,address,uint256)" 3112 20 $DEPLOYER_PUBLIC_KEY 86400 --private-key $DEPLOYER_PRIVATE_KEY --rpc-url $SEPOLIA_RPC_URL
```

```
tx url: https://sepolia.etherscan.io/tx/0x9fa8fbc8ebbdd35685a51cee88524e774d00cf202f900b6cbb40a8630deea1c2
```

### addMember

```sh
$ cast send $SEMAPHORE_CONTRACT_ADDRESS "addMember(uint256,uint256)" 3111 11216909772611159987761092911642618205426065353080584263434245612434546457738 $DEPLOYER_PUBLIC_KEY --private-key $DEPLOYER_PRIVATE_KEY --rpc-url $SEPOLIA_RPC_URL
```

```
tx url: https://sepolia.etherscan.io/tx/0xb7b86317e35f90eb04476df9df7748caf08b6009dc1e84ba120484abafe95103
```

### addMembers

```sh
$ cast send $SEMAPHORE_CONTRACT_ADDRESS "addMembers(uint256,uint256[])" 3111 [14873715770595485563260312142424404454075221624479988126831304707589940083175,18269309615270290411027305068632481324321098674725859679913224414606738740685]  $DEPLOYER_PUBLIC_KEY --private-key $DEPLOYER_PRIVATE_KEY --rpc-url $SEPOLIA_RPC_URL
```

```
tx url: https://sepolia.etherscan.io/tx/0xaaeb36e26e6e1934a56237f0caa4e4cd02e864c27e3951d2596c010a16ee756a
```

<!-- TODO: Add more results -->
