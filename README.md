# Bitfinity EVM SDK

[![license-mit](https://img.shields.io/badge/License-MIT-teal.svg)](https://opensource.org/licenses/MIT)
[![Build Test](https://github.com/bitfinity-network/bitfinity-evm-sdk/actions/workflows/build-test.yml/badge.svg)](https://github.com/bitfinity-network/bitfinity-evm-sdk/actions/workflows/build-test.yml)

![github](https://github.com/bitfinity-network/bitfinity-evm-sdk/assets/25309184/4775bc4b-1033-4528-ab4b-64ed05b6dcbf)

## Components

- [did](./src/did): Data types for [evm-canister](https://github.com/bitfinity-network/evm-canister)
- [eth-signer](./src/eth-signer/): A library which provides a trait for signing transactions and messages.
- [evm-block-extractor](./src/evm-block-extractor/): It is made up of two components:
  - [evm-block-extractor](./src/evm-block-extractor/): A library for extracting blocks from the Bitfinity EVM and storing them in a PostgresSQL DB
  - [evm-block-extractor-server](./src/evm-block-extractor/bin/server): A JSON-RPC server for the EVM block extractor
- [evm-canister-client](./src/evm-canister-client/): A library for interacting with the Bitfinity EVM
- [register-evm-agent](./src/register-evm-agent/): A Cli tool for generating an ETH Wallet & reserving a canister to the Bitfinity EVM

## License

bitfinity-evm-sdk is licensed under the MIT license.

You can read the entire license [HERE](./LICENSE)
