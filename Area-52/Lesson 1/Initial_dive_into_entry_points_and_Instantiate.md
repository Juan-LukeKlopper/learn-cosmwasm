# An initial dive into entry points and instantiate

- [An initial dive into entry points and instantiate](#an-initial-dive-into-entry-points-and-instantiate)
  - [<u>Entry points </u>](#uentry-points-u)
  - [<u>Returning the value of instantiate</u>](#ureturning-the-value-of-instantiateu)


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