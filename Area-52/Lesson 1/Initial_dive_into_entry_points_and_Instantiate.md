# An initial dive into entry points and instantiate

- [An initial dive into entry points and instantiate](#an-initial-dive-into-entry-points-and-instantiate)
  - [<u>Entry points </u>](#uentry-points-u)
  - [<u>Returning the value of instantiate</u>](#ureturning-the-value-of-instantiateu)
  - [<u>Entry Point Parameters</u>](#uentry-point-parametersu)
  - [<u>Storing State</u>](#ustoring-stateu)
    - [Serialize and Deserialize](#serialize-and-deserialize)
  - [<u>Storing the Struct</u>](#ustoring-the-structu)
    - [CosmWasm Storage](#cosmwasm-storage)


## <u>Entry points </u>

You can think of smart contract code as the blueprint for the contract itself.
It does not truly become a useful part of a blockchain until it has been instantiated.
Once it is established on the blockchain it can be interacted with.

In Rust (and other C/C++ and Java-like languages), there is the concept of an "entry point". 
This is typically called main and is the first function called when a compiled application is executed.
When compiling a traditional Rust app, the compiler expects there to be a main function from which the rest of the application will start and run.

There is no main function in CosmWasm - so how does it know instantiate is an entry point?

We instruct the code of that fact with a function attribute, using entry_point from the cosmwasm_std library:

```rust
use cosmwasm_std::entry_point;

#[entry_point]
pub fn instantiate(...) -> ... {...} 
```

Instantiate is one of three such entry points into a smart contract. 
It's used during deployment to initialize settings for a contract that's been stored on-chain, and create an address on the blockchain where the contract can be called.
Maybe you recognize this pattern, as it's called a Constructor in many programming languages.

## <u>Returning the value of instantiate</u>

The instantiate entry point returns a Rust `Result`, taking a CosmWasm `Response` struct and an error type.

`Response`
The Response struct returns a few items that allow the contract to communicate back to the caller.
It has several types in it for sending messages and data back.

`Messages`
CosmWasm is based on the `Actor Model` design pattern. 
In this pattern, Actors do not talk directly to one another (i.e., do not call functions directly) but rather send messages to one another. 
Here's a basic interface for the Actor model:

```rust
pub trait Actor {
  fn handle(msgPayload: &[u8]) -> Vec<Msg>;
}

pub struct Msg {
  pub destination: Vec<u8>,
  pub payload: Vec<u8>,
}
```

## <u>Entry Point Parameters</u>

Our instantiate function takes four parameters: DepsMut, Env, MessageInfo and InstantiateMsg.

The first three parameters are from the cosmwasm_std library, and the last one, InstantiateMsg, is a custom struct we will write ourselves.

- `Deps`, short for _dependencies_, provides access to CW utilities such as persistent storage; 
which, as you might have deduced, allows you to read from storage. For writing to storage you'll want the mutable version of this package, called `DepsMut`

- `Env`, short for _environment_, provides information about the block and the transaction the message was executed in.

- `MessageInfo` contains the sender's (that is, the caller of the contract method) address and funds sent along with the transaction.

## <u>Storing State</u>

While a smart contract can store information on the blockchain, that comes at a cost in the form of `gas and fees`, but this ability to store and compute on open data, that everybody can access, is what allows it to be decentralized.

we create a `State` struct to store our basic information when the contract is initialized. This is typically done in a file named `state.rs` in your CosmWasm project.

### Serialize and Deserialize

We make use of the `Serde` framework to serialize and deserialize the Rust data structures. In our case, we need our upcoming `State` struct to store data in such a way that it's prepared for storing on the blockchain.

`Serialize` and `Deserialize` From the Serde crate, are traits that need to be applied to the struct using the `derive` attribute.

## <u>Storing the Struct</u>

### CosmWasm Storage

One of the parameters passed to `instantiate` is `deps: DepsMut;` it contains a member called `storage: Storage`. This is what we will be updating on the blockchain. We'll do it using the `Singleton` implementation from the `cosmwasm_storage` library.

We'll have two separate functions — a write function, called `config`, and a read-only function called `config_read`. Conveniently, `cosmwasm_storage` provides two matching versions called `Singleton` and `ReadonlySingleton`.