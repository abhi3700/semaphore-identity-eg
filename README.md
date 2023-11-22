# Semaphore Playground

Play with Semaphore Protocol.

## Project setup

Steps (one-time):

```sh
$ yarn init -y
$ yarn add @semaphore-protocol/identity
$ yarn add typescript @types/node --dev
$ npx tsc --init
```

Write code in a file and run with `yarn`.

> To run a `.ts` file, use `$ npx tsc && node <path/to/file.ts>` as script in `package.json`.

```sh
$ yarn <script-command>
```

## Usage

### Identities

Generate a random identity:

```sh
$ yarn id:random
```

Generate a deterministic identity from a secret message:

```sh
$ yarn id:deterministic
```

### Groups

Create a group with id:

```sh
$ yarn group:offchain
```

Create a group with id and depth:

```sh
$ yarn group:offchain-depth
```

Create a group with id, depth and members (add, remove, update):

```sh
$ yarn group:offchain-members
```

### Contracts

This is for onchain activities.

**Setup**:

```sh
# download the contract package into the root of repo.
$ npm pack @semaphore-protocol/contracts

# unpack the tarball here
$ tar -xzf semaphore-protocol-contracts-3.15.1.tgz

# rename the "package" to "contracts"
$ mv package contracts

# remove the tarball
$ rm semaphore-protocol-contracts-3.15.1.tgz

# move everything in contracts to src folder
$ mkdir -p src && find . -maxdepth 1 ! -name 'src' -exec mv {} src \;
```
