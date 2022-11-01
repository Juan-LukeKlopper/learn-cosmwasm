# Queries

## Creating a message

You have a basic contract structure ready. It's time to add some life to the contract. In this lesson, you will implement a query message.

At first, we need an additional dependency - a `serde` crate with a `derive` feature to define serializable datatypes:

```bash
cargo add serde --features derive
```

Now create a new module modifying the `src/lib.rs`:

```rust
use cosmwasm_std::{
    DepsMut, Env, MessageInfo, Empty, StdResult, Response, entry_point
};

pub mod msg;

#[entry_point]
pub fn instantiate(
	_deps: DepsMut,
	_env: Env,
	_info: MessageInfo,
	_msg: Empty,
) -> StdResult<Response> {
	Ok(Response::new())
}
```

We want our messages module to be public - in the future, external contracts may wish to use it to communicate with it. The next step is to create a module file, `src/msg.rs`:

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Value {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct ValueResp {
    pub value: u64,
}
```

We created two messages here. The first one is the query message. When the contract is queried, it should be able to create a variety of queries. To do so, we typically create query messages as enum types, where every single variant is a separate query the contract understands.

There are a couple of things to discuss in this message. First, it has to derive at least `Deserialize`, but we also want it to implement `Serialize` - so we can use it to send this message from a different contract or tests. The `Clone` is kind of optional, but as it is a data transfer message, it is sometimes helpful to be able to clone it around. `Debug` and `PartialEq` are mostly here for testing purposes - we will talk about testing smart contracts in the next lesson. `Eq` is here only because it is a good practice to implement it if it is possible and `PartialEq` is implemented - which is warned by clippy. There is a good chance, it would be removed if we would add some variant without implementation of `Eq`.

What may be surprising is how the `Value {}` variant is defined - there are those curly braces, which are not strictly required by Rust syntax. It looks almost strange. However, it is here on purpose. It is related to how serde is serializing JSON values. If you would go with just `Value` here, when serializing `QueryMsg::Value` to JSON, it would end up as a string:

```
"Value"
```

There are two problems with that. First of all - the sole string is not a proper JSON object! And we want messages to be proper JSON objects. Another problem is inconsistency. Let's consider the following query message:

```rust
enum QueryMsg {
    Value, 
    PendingFunds { denom: Option<String> },
}
```

The `Value` variant is still (de)serialized as a string, but the `PendingFunds` becomes a proper JSON object:
```json
{
    "PendingFunds":  {}
},
```

or with denom provided:

```json
{
    "PendingFunds:": {
        "denom": "uatom"
    }
}
```

This inconsistency is itchy, and we want to avoid it. Also, most other frameworks deserialize empty enums as objects with a single field with an empty value. To achieve it with serde, you just add those curly braces. Doing so, you make serde serialize this variant to better form:

```json
{
    "Value": {}
}
```

The other thing to mention is the `#[serde(rename_all = "snake_case)]` attribute. As you probably notice, the fields serialize to JSON by serde match the names of types in Rust. However, it is not a very JSONish way to name things. In Rust, we tend to use a "CamelCase" to name our types, while in JSON, we expect our fields to be "snake_case". We could obviously just rename variants to be "snake_case," but it would not be a rusty way to do so. Instead, we use the `rename_all` serde attribute to make it handle name conversion for us. Now, our `QueryMsg::Value` variant is serialize to nice JSON value:

```json
{
    "value": {}
}
```
The second message I created is a response to the `Value` query. It is very similar to the `Query` message, but it is a `struct` this time - we would always know what kind of response we are sending or expecting. Also, you can argue that in this very message, the `rename_all` attribute is unnecessary as nothing is renamed, and you would be right. However, it is a good habit to have it there for consistency.

## Query implementation
Now we need to create yet another module to have a place for logic implementation. Let's call it `contract`, and keep this one private as it would contain internal contract logic:

```rust
use cosmwasm_std::{
    DepsMut, Env, MessageInfo, Empty, StdResult, Response, entry_point
};

mod contract;
pub mod msg;

#[entry_point]
pub fn instantiate(
	_deps: DepsMut,
	_env: Env,
	_info: MessageInfo,
	_msg: Empty,
) -> StdResult<Response> {
	Ok(Response::new())
}
```

And now let's implement the query handler in `src/contract.rs`:

```rust
use crate::msg::ValueResp;

pub mod query {
    use crate::msg::ValueResp;

    pub fn value() -> ValueResp {
        ValueResp { value: 0 }
    }
}
```

As you can see, I keep implementing my queries in dedicated submodules. I could move it to its own file, but I find it too fragmented. Also, I could just put everything in the `contract` module. Still, it would make it difficult to keep names not colliding between messages. Having this initial split for submodules makes extracting them later to own files easier - at the point when `contract.rs` overgrows.

The essential thing here is the `query::value()` function which would be called when `QueryMsg::Value {}` is received. The function returns an arbitrary object which would be serialized before sending as a response.

Lastly need to add an entry point for queries in `src/lib.rs`:

```rust
use cosmwasm_std::{
    Deps, DepsMut, Env, MessageInfo, Empty, StdResult, Response, entry_point
};

mod contract;
pub mod msg;

#[entry_point]
pub fn instantiate(
	_deps: DepsMut,
	_env: Env,
	_info: MessageInfo,
	_msg: Empty,
) -> StdResult<Response> {
	Ok(Response::new())
}

#[entry_point]
pub fn query(_deps: Deps, _env: Env, msg: msg::QueryMsg) -> StdResult<Binary> {
    use msg::QueryMsg::*;
    use contract::query;

    match msg {
        Value {} => to_binary(&query::value()),
    }
}
```

As you see, the entry point for query is very similar to instantiate. The differences are:

- The `deps` argument is of type `Deps` instead of `DepsMut` - it is because queries can never modify any blockchain state.
- There is no `MessageInfo` argument - queries can never depend on who sends the message or on any query circumstances besides the blockchain state itself. Queries also never have funds sent with them.
- The returned type is not a `Response`, but a `Binary` type instead - it is because the query returns arbitrary data to the querier instead of processing a full actor flow which is handled with `Response` type.
To implement my `query` entry point, we typically just dispatch on message received to proper message handlers. I like to import all dispatched message variants to function scope to make it easier to read, but it depends strongly on your taste. I also import the `query` module in the function scope as I use it here often.

An essential thing to do is to call the `to_binary` function on the result of `value()` call. As you remember, we returned arbitrary types from the handler, but we need a uniform `Binary` type returned from the entry point. The `Binary` type represents any base64-encoded binary data. In this case, this would be JSON encoded query response. The `to_binary` function is a helper serializing and encoding serializable types to the `Binary` type. The function already returns `StdResult`, so you don't have to do any error management - just return the result directly from a final function.

After all, don't forget if the contract is still valid smart contract using `cosmwasm-check` utility:

```bash
cargo wasm
cosmwasm-check ./target/wasm32-unknown-unknown/release/counting_contract.wasm 
Supported features: {"stargate", "staking", "iterator"}
contract checks passed.
```

At this point, you have a contract, which can be queried. It always returns a static response, but we will soon work on having an internal state in it. But before that happens, as we have some things done, it is an excellent time to learn how to test them.