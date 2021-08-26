# Marketplace_Substrate
A marketplace on blockchain using rust programming and substrate framework.
<p align="center">
<img src="https://miro.medium.com/max/1400/1*Bf0Jt8OIE40taVntJFqhVw.png" width="500">
</p>

## Introduction

### Blockchain
A blockchain is a decentralized, distributed, and oftentimes public, digital ledger consisting of records called blocks that is used to record transactions across many computers so that any involved block cannot be altered retroactively, without the alteration of all subsequent blocks. This allows the participants to verify and audit transactions independently and relatively inexpensively. A blockchain database is managed autonomously using a peer-to-peer network and a distributed timestamping server. They are authenticated by mass collaboration powered by collective self-interests. Such a design facilitates robust workflow where participants' uncertainty regarding data security is marginal. The use of a blockchain removes the characteristic of infinite reproducibility from a digital asset. It confirms that each unit of value was transferred only once, solving the long-standing problem of double spending. A blockchain has been described as a value-exchange protocol. A blockchain can maintain title rights because, when properly set up to detail the exchange agreement, it provides a record that compels offer and acceptance.

Logically, a blockchain can be seen as consisting of several layers:
* infrastructure (hardware)
* networking (node discovery, information propagation and verification)
* consensus (proof of work, proof of stake)
* data (blocks, transactions)
* application (smart contracts/decentralized applications, if applicable)

### Substrate
<p align="center">
  <img src="https://cdn.hackernoon.com/hn-images/1*OQP5QAtLtrVCtNCKwB6GkQ.png" height="250" >
  </p>
  
<p>
Substrate is a modular and an open-source framework that enables you to create purpose-built blockchains by composing custom or pre-built components and, build entire, configurable blockchains in minimal time.
  </p>
  
#### Overview 
<p align="center">
  <img src="https://i.imgur.com/GZaJN3T.png" width="450" >
  </p>

## Description

* Built a marketplace on blockchain using substrate framework and rust programming.
* Here the seller can store the product using its productid to be sold and the buyer can buy the product using its productid.

<p align="center">
  <img src="https://braincode.xyz/wp-content/uploads/main_12-730x346.png" width="400">
  </p>

## Prerequisites:
  
<p>
You will probably need to do some set-up to prepare your computer for Substrate development.
</p>

* First we need to install and configure rustup and rust toolchain to default to the latest stable version
<a href="https://substrate.dev/docs/en/knowledgebase/getting-started">Click here</a> to go through installation guide.

* Once the prerequisites are installed, you can use Git to clone the Substrate Developer Hub Node Template, which serves as a good starting point for building on Substrate.
<a href="https://substrate.dev/docs/en/tutorials/create-your-first-substrate-chain/setup">Click here</a> to setup your computer and interact with your node.

## Working ⚒ 

### To clone the repo:
To clone the repo on Linux based system, use the command mentioned below 
```
git clone https://github.com/anish63342/marketplace_substrate
```

### To compile the node template:
To compile the node template on Linux based system, use the command mentioned below 
```
cd marketplace_substrate
```
```
cargo build --release 
```
This process may take a while to complete...

### Interacting with your node:
To start your node, use the command mentioned below
```
# Run a temporary node in development mode
./target/release/node-template --dev --tmp
```

## Output

  
  

