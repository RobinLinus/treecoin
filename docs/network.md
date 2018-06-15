# Network

## Design Goals
- Scalability beyond a Million concurrent nodes.
- Get rid of the concept of  "[clients](https://en.bitcoin.it/wiki/Clients)" which rely on servers and can't benefit a peer to peer network.

## Node Family
- Full Node ( fully verifying nodes with costly setup )
  - Miner Node
  - High-Stake Node ( such as exchanges or any other entity with high security requirements )
- Nano Node ( balance proofs & transaction broadcasting. quick and simple )
  - Browser
  - IOT devices

## Network Overview
1. Mainnet ( Full Nodes )
2. Browser Bridge ( Full Node Hybrids )
3. Browser Network ( Nano Nodes )

### Backbone Network
Purpose:
- trust-less, censorship-resistant consensus
- simple, efficient and resilient
- high throughput of transactions

Full nodes are the fundamental anchor of trust-less security. A node which fully implements the protocol will always use the correct block chain and will never allow double-spends or invalid transactions to exist in the block chain under any circumstances.

### Browser Bridges
Required for browsers to communicate with the backbone.

Purpose:
- Initial seeding, in particular WebRTC signaling
- Relay blocks from the backbone network into the browser network.
- Relay transactions from the browser network into the backbone network.

Requirements:
- Open Port
- Domain
- SSL Certificate

Research Idea: Installation-free, browser-based WebSocket-to-WebRTC bridges via insecure origins/data URLs?

### Browser Network
Purpose:
- Decentralized hosting of the chain state.
- Decentralized updates of the state by applying new blocks received via the bridge.
- Serve all nano nodes' queries within the browser network.
- Route transactions to the bridge.

Nodes which only partially implement the protocol typically trust that 50% or more of the network's mining power is honest.


Requirements:
- Browser-based P2P Network
  - Browser-to-Browser _WebRTC signaling_
- _Distributed Hash Table_ to host the chain state
