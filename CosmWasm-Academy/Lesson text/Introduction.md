# Setting up an environment

## Installing the toolset

Before we start building our CosmWasm smart contract, we need to set up an environment. The first thing you need is cargo, the Rust compiler. You can find its installation instructions on https://www.rust-lang.org/tools/install.

Additionally, you need a wasm target for cargo, which you can install with rustup target add wasm32-unknown-unknown. It would allow you to cross-compile Rust code to wasm bytecode instead of native, which we need for our smart contracts.

When your Rust is set up, you should install the cosmwasm-check utility. It is a tool verifying if the wasm binary is a valid CosmWasm smart contract ready to upload to the chain. We want all contracts we build to be valid, so cosmwasm-check would be very useful. You can install it using cargo, executing ```cargo install cosmwasm-check``` in your terminal.

## Verifying installation

Now it is time to verify the installation. Start with getting some ready-to-use smart contracts. You can find good examples in cw-plus repository. Check it out with ```git checkout git@github.com:CosmWasm/cw-plus.git```. Go to the cw-plus/contracts/cw1-whitelist directory, which contains a simple grouping smart contract. You should be able to build it with cargo wasm command. If build passes, your Rust is set up correctly.

Now go back to the root repository directory (the cw-plus), and look for your build artifacts - they would be in target directory, and would have .wasm extension. You can list all of them with ```find ./target -name "*.wasm"```. There should be a ./target/wasm32-unknown-unknown/release/cw1_whitelist.wasm file. Execute ```cosmwasm-check ./target/wasm32-unknown-unknown/release/cw1_whitelist.wasm```. If the contract check passes, you are ready for the next lesson - creating an empty but valid CosmWasm contract.