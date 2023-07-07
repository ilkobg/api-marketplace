# NFT Marketplace API

## Overview

The API is build in Rust, using **reqwest_graphql** as GraphQL client for communicating with the subgraph, and Axum framework for thr web server part.

The connection with the subgraph is through the subgraph endpoint:
`pub static SUBGRAPH_ENDPOINT: &'static str =
    "https://api.studio.thegraph.com/query/48381/nftmarketplace/version/latest";`


## Requirements

See dependencies section in [Cargo.toml](https://github.com/ilkobg/api-marketplace/blob/main/Cargo.toml)

## Usage

Run
```
cargo build
cargo run
```
The server will start on localhost on port 8085:
`127.0.0.1:8085`

## Endpoints

- GET `/all-listings` - Returns a list with all active listings
- GET `/collection data/CONTRACT_ADDRESS` - Here CONTRACT_ADDRESS is address of the NFT collection(contract). The endpoint returns infromation about traded volume and floor price
- GET `/listings/OWNER` - Here OWNER is wallet address. The endpoint returns list with all active listings for the provided owner.
- GET `/purchases/BUYER` - Here BUYER is wallet address. The ednpoint returns all buyings for the provided address.

## Examples

- `http://127.0.0.1:8085/purchases/0x7f7A4D0A2167D5349b1254D5837ad12cb773a38E`

```
[
    {
        "id": "0x8d664549cd899bd21cf336defc1ab275e67eff09aa527345afb11f8a657a35884c000000",
        "buyer": "0x7f7a4d0a2167d5349b1254d5837ad12cb773a38e",
        "owner": "0xd4136b49415ad6ff8cd7281e1205d5fb1a10a5c0",
        "tokenId": "0",
        "nftContractAddress": "0xe0852ff3ade879b865a0685e0f1cf1beec8bce34",
        "price": "100",
        "blockNumber": "9299484",
        "blockTimestamp": "1688639196",
        "transactionHash": "0x8d664549cd899bd21cf336defc1ab275e67eff09aa527345afb11f8a657a3588"
    }
]
```

- `http://127.0.0.1:8085/all-listings`

```
[
    {
        "id": "0xe0852ff3ade879b865a0685e0f1cf1beec8bce34-2",
        "owner": "0xd4136b49415ad6ff8cd7281e1205d5fb1a10a5c0",
        "tokenId": "2",
        "nftContractAddress": "0xe0852ff3ade879b865a0685e0f1cf1beec8bce34",
        "price": "300",
        "isActive": true,
        "blockNumber": "9301239",
        "blockTimestamp": "1688666916",
        "transactionHash": "0x616450252cae2e40d23b32087fdbf2b9a21e9c4ee3b9d66c5a4f4a738ca24891"
    },
    {
        "id": "0xe0852ff3ade879b865a0685e0f1cf1beec8bce34-3",
        "owner": "0xd4136b49415ad6ff8cd7281e1205d5fb1a10a5c0",
        "tokenId": "3",
        "nftContractAddress": "0xe0852ff3ade879b865a0685e0f1cf1beec8bce34",
        "price": "400",
        "isActive": true,
        "blockNumber": "9305573",
        "blockTimestamp": "1688734584",
        "transactionHash": "0xfa15db6a06773359eacf514e97afc62d121d48e8ba469eee0686b9008172d7e2"
    }
]
```

- `http://127.0.0.1:8085/listings/0xd4136B49415Ad6FF8cd7281E1205d5Fb1a10a5c0`

```
[
    {
        "id": "0xe0852ff3ade879b865a0685e0f1cf1beec8bce34-2",
        "owner": "0xd4136b49415ad6ff8cd7281e1205d5fb1a10a5c0",
        "tokenId": "2",
        "nftContractAddress": "0xe0852ff3ade879b865a0685e0f1cf1beec8bce34",
        "price": "300",
        "isActive": true,
        "blockNumber": "9301239",
        "blockTimestamp": "1688666916",
        "transactionHash": "0x616450252cae2e40d23b32087fdbf2b9a21e9c4ee3b9d66c5a4f4a738ca24891"
    },
    {
        "id": "0xe0852ff3ade879b865a0685e0f1cf1beec8bce34-3",
        "owner": "0xd4136b49415ad6ff8cd7281e1205d5fb1a10a5c0",
        "tokenId": "3",
        "nftContractAddress": "0xe0852ff3ade879b865a0685e0f1cf1beec8bce34",
        "price": "400",
        "isActive": true,
        "blockNumber": "9305573",
        "blockTimestamp": "1688734584",
        "transactionHash": "0xfa15db6a06773359eacf514e97afc62d121d48e8ba469eee0686b9008172d7e2"
    }
]
```

- `http://127.0.0.1:8085/collection-data/0xe0852fF3Ade879B865a0685e0F1cF1BEec8bce34`

```
{
    "id": "0xe0852fF3Ade879B865a0685e0F1cF1BEec8bce34",
    "floorPrice": 100,
    "tradedVolume": 300
}
```