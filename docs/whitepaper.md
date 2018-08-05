# TreeCoin

## Design Goals

TreeCoin is designed for mainstream adoption of trustless p2p payments. The core design targets are:

1. Usability
	- "Sync by downloading less than a song"
	- Installation-free because web-based.
2. Scalability
	- Trustless p2p payments at Paypal-scale: 
		- Up to 100 TX/s
		- Up to 100 Million Users

## Design Approach 

The general approach is to design a system consisting of mainly two groups:

1. Endusers with consumer level hardware on mobile connections
2. Miners with expensive setup of industrial operations

There are three core assumptions to the design:

- Endusers aren't expected to have enough resources to fully validate the chain, therefore the network has to provide them with compact proofs for the states they are interested in.
- Assuming a ratio between miners and endusers of 1 : 1000, Miners aren't expected to "waste" resources to serve a lot of endusers. Therefore we want endusers to serve themselves in a secondary p2p layer to minimize the load on the primary layer.
- On-boarding endusers should be as simple and frictionless as possible. Therefore the protocol needs to be native to the web such that endusers are able to connect to the network from their browsers without requiring any installation. The general idea is to port well-known user flows of centralized services such as Paypal or Venmo to a fully decentralized network. The core challenge is to achieve simplicity for the endusers in spite of the complexity of the underlying cryptocurrency.

