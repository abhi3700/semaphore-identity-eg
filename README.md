# Semaphore Identity Example

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

```sh
# Generate a random identity
$ yarn idn
```

```sh
# Generate an identity from a secret message
$ yarn idd
```
