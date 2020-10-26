# message-zome

[![hc-institute-japan](https://circleci.com/gh/hc-institute-japan/p2pmessage-zome.svg?style=svg)](https://circleci.com/gh/hc-institute-japan/p2pmessage-zome)

Small zome for allowing synchronous (soon asynchronous) messaging between two agents, in Holochain RSM.


This module is designed to be included in other DNAs, assuming as little as possible from those. It is packaged as a holochain zome and no built-in UI is provided 
for it.

## Assumptions

These are the things you need to know to decide if you can use this module in your happ:

- Zome:
  - This zome doesn't provide any capability grant/claim structure. Which means that, you as the consumer of this zome in your project, must provide another zome that will grant capabilities to each of the functions that this zome provides. This is done so that consumer of this zome can decide for themselves who gets to send and receive message from whom. 
  
### Including the zome in your DNA

1. Create a new folder in the `zomes` of the consuming DNA, with the name you want to give to this zome in your DNA.
2. Add a new `Cargo.toml` in that folder. In its content, paste the `Cargo.toml` content from any zome.
3. Change the `name` properties of the `Cargo.toml` file to the name you want to give to this zome in your DNA.
4. Add this zome as a dependency in the `Cargo.toml` file:
```toml
[dependencies]
p2pmessage = {git = "https://github.com/hc-institute-japan/p2pmessage-zome", package = "p2pmessage"}
```
5. Create a `src` folder besides the `Cargo.toml` with this content:
```rust
extern crate p2pmessage;
```
6. Add the zome into your `*.dna.workdir/dna.json` file.
7. Compile the DNA with the usual `CARGO_TARGET=target cargo build --release --target wasm32-unknown-unknown`.

## Developer setup

This respository is structured in the following way:

- `zome/`: example DNA with the `p2pmessage` code.
- Top level `Cargo.toml` is a virtual package necessary for other DNAs to include this zome by pointing to this git repository.

Read the [UI developer setup](/ui/README.md) and the [Zome developer setup](/zome/README.md).

## Contributions
We would like to thank [@guillemcordoba](https://github.com/guillemcordoba) and [holochain-open-dev](https://github.com/holochain-open-dev) for providing a reusable module template to easily create zomes that can be reusable in other Holochain pojects. If you are interested in using the same template, check it out [here](https://github.com/holochain-open-dev/reusable-module-template)
