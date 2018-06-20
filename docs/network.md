# Network

## Design Goals
- Scalability beyond a Million concurrent users.
- Get rid of dumb "[clients](https://en.bitcoin.it/wiki/Clients)" which rely on servers and imply _costs_ rather than _resources_. A true _peer to peer network_ consists of [__*nodes*__](https://en.wikipedia.org/wiki/Peer-to-peer#Architecture) and scales inherently well.

## Node Family
- Full Node ( fully verifying nodes )
  - Miner Nodes ( with expensive hardware )
  - High-Stake Nodes ( such as exchanges or any other entity with high security requirements )
  - Benevolent Nodes ( users donating resources to the network )
- Nano Node ( balance proofs & transaction broadcasting )
  - Browsers ( one-click setup )
  - IOT devices

## Network Overview
-  Mainnet ( Full Nodes, consensus critical )
  - Bridges ( into the Nano Network )
- Nano Network ( Nano Nodes, infinitely scalable hosting of the chain state )

### Mainnet
**Purpose:** The mainnet and its full nodes are the fundamental anchor of trust-less security. A node which fully implements the protocol will always use the correct block chain and will never allow double-spends or invalid transactions to exist in the block chain under any circumstances.
- trust-less, censorship-resistant consensus
- simple, efficient and resilient
- high throughput of transactions

#### Bridges
**Purpose:** Enable nano nodes to communicate with the mainnet.
- Initial seeding, in particular WebRTC signaling
- Light-weight communication:
  - Relay blocks from the mainnet into the nanonet.
  - Relay transactions from the nanonet into the mainnet.

#### Incoming Connections
// Todo: more than bullet points. Address the general problem of p2p and NAT traversal
- TCP + Open Port
- UDP, UPnP, hole-punching
- Domain + SSL Certificate
- Research Idea: Installation-free, browser-based WebSocket-to-WebRTC bridges via insecure origins/data URLs?



### Nanonet
**Purpose:** Self-serving network of nano nodes.
- Don't stress the mainnet! Answer all balance queries within the nanonet.
- Decentralized hosting of the chain state.
- Decentralized updates of the state by applying new blocks received via the bridges.
- Route transactions into the mainnet via the bridges.
- Browser-based for insanely simple usability.


Requirements:
- Browser-based P2P Network
  - Browser-to-Browser _WebRTC signaling_
- _Distributed Hash Table_ to host the chain state

#### Limitation
Nano Nodes trust that 50% or more of the network's mining power is honest. In general this is a valid assumption for low-value transactions because double-spend attacks are usually expensive. Recipients of high-value transactions must default to a full node.
