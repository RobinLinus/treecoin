# Network

## Client Overview
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
- simple, efficient and resilient blockchain architecture
- high tx throughput

### Browser Bridges
Required for browsers to communicate with the backbone.

Purpose:
- Initial WebRTC Signaling
- Initial Seeding

Requirements:
- Open Port
- Domain
- SSL Certificate

Research Idea: Installation-free, browser-based websocket-to-webrtc bridges via insecure origins/data URLs?

### Browser Network
Purpose:
- distributed storage of the state
- distributed updates of the state by applying new blocks from the bridge
- serve all nano clients' queries within the browser network

Requirements:
- P2P Network
- WebRTC Signaling
- Distributed Hash Table
