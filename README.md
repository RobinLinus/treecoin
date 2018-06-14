# Lovicash: A P2P Payment System

## Design Goals
- Trust-less, censorship-resistant payment network
- Scalability
  - Up to 25 tx/s ( 25% of Paypal's transaction volume )
  - Up to 1 000 000 concurrent clients
- Nano Nodes
  - Trust-less _nano sync_ ( "sync by downloading less than a song" ~ 3 MB )
  - Web-based for simple usability ( "click link to start syncing" )

## Architecture Overview
The architecture is based on bitcoin's design and introduces the following extensions:
- _State Tree_: an efficient and authenticated global state representation.
  -  _Headers tree_: an index for blocks in the chain.
    - _Proof of proofs of work_: compact proof of longest chain.
  - TXO-IDs: an index for transaction outputs.
  - UTXO bit vector: an index for unspent transaction outputs.
  - Address index: an index for unspent transaction output addresses.
  - Transactions: compact UTXOs and signatures.
- Network
  - Backbone Network: miners and full nodes.
  - Browser bridges: servers for block relay and seeding for nano nodes.
  - Nano Network: self-serving nano nodes.

## Implementation
- Rust
- WebAssembly
