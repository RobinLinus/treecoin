# Network

## Design Goals
- Scalability beyond a Million concurrent nodes.
- Get rid of the concept of  "[clients](https://en.bitcoin.it/wiki/Clients)" which rely on servers and can't benefit a peer to peer network.

## Node Family
1. Full Node ( fully verifying nodes with costly setup )
  - Miner Node
  - High-Stake Node ( such as exchanges or any other entity with high security requirements )
2. Nano Node ( balance proofs & transaction broadcasting. quick and simple )
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

Research Idea: Installation-free, browser-based websocket-to-webrtc bridges via insecure origins/data URLs?

### Browser Network
Purpose:
- Decentralized hosting of the state.
- Decentralized updates of the state by applying new blocks received via the bridge.
- Serve all nano nodes' queries within the browser network.
- Route transactions to the bridge.

Requirements:
- Browser-based P2P Network
  - Browser-to-Browser WebRTC signaling
- Distributed Hash Table to store the state
