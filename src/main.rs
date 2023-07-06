use anyhow::{anyhow, Result};
use axum::extract::Path;
use axum::http::Response;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{routing::get, Router};
use reqwest_graphql::Client;
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, str::FromStr};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Listing {
    pub id: String,
    pub owner: String,
    pub token_id: String,
    pub nft_contract_address: String,
    pub price: String,
    pub is_active: bool,
    pub block_number: String,
    pub block_timestamp: String,
    pub transaction_hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CollectionData {
    pub id: String,
    pub floor_price: i64,
    pub traded_volume: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Purchase {
    pub id: String,
    pub buyer: String,
    pub owner: String,
    pub token_id: String,
    pub nft_contract_address: String,
    pub price: String,
    pub block_number: String,
    pub block_timestamp: String,
    pub transaction_hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListingsData {
    lists: Vec<Listing>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PurchasesData {
    buys: Vec<Purchase>,
}

#[derive(Serialize, Debug)]
pub struct Vars {}

pub static SUBGRAPH_ENDPOINT: &'static str =
    "https://api.studio.thegraph.com/query/48381/nftmarketplace/version/latest";

pub async fn start_server(address: &str) {
    // Define the HTTP handlers
    let app = Router::new()
        .route("/all-listings", get(handle_all_listings))
        .route("/collection-data/:id", get(handle_collection_data))
        .route("/listings/:owner", get(handle_listings_for_owner))
        .route("/purchases/:address", get(handle_purchases_for_buyer))
        .fallback(get(handle_404))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        );

    // Create the server and start listening for requests
    let addr = SocketAddr::from_str(address).unwrap();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_404() -> impl IntoResponse {
    let response_body = "The requested resource could not be found".to_string();
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(response_body)
        .unwrap()
        .into_response()
}

async fn handle_all_listings() -> impl IntoResponse {
    let response_body;

    match listings(None).await {
        Ok(listings) => {
            let all_listing = listings;
            println!("All listings {:?}", all_listing);

            let response_body = serde_json::to_string(&all_listing).unwrap();

            Response::new(response_body).into_response()
        }
        Err(e) => {
            println!("An error occurred: {}", e);
            response_body = e.to_string();
            Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(response_body)
                .unwrap()
                .into_response()
        }
    }
}

async fn handle_collection_data(Path(id): Path<String>) -> impl IntoResponse {
    let traded_volume_by_contract_address = traded_volume(id.to_lowercase()).await;
    println!(
        "Traded volume by contract address {} - {:?}",
        id.to_lowercase(),
        traded_volume_by_contract_address
    );

    let response_body;

    match floor_price(id.to_lowercase()).await {
        Ok(price) => {
            println!(
                "Floor price by contract address {} - {:?}",
                id.to_lowercase(),
                price
            );
            let collection_data = CollectionData {
                id,
                floor_price: price,
                traded_volume: traded_volume_by_contract_address,
            };

            // 200 http code
            response_body = serde_json::to_string(&collection_data).unwrap();
            Response::new(response_body).into_response()
        }
        Err(e) => {
            println!("An error occurred: {}", e);
            response_body = e.to_string();
            Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(response_body)
                .unwrap()
                .into_response()
        }
    }
}

async fn handle_listings_for_owner(Path(owner): Path<String>) -> impl IntoResponse {
    let response_body;

    match listings(Some(owner.to_lowercase())).await {
        Ok(listings) => {
            let listings_by_owner = listings;
            println!(
                "Listings by owner {:?} - {:?}",
                listings_by_owner,
                owner.to_lowercase()
            );

            let response_body = serde_json::to_string(&listings_by_owner).unwrap();

            Response::new(response_body).into_response()
        }
        Err(e) => {
            println!("An error occurred: {}", e);
            response_body = e.to_string();
            Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(response_body)
                .unwrap()
                .into_response()
        }
    }
}

async fn handle_purchases_for_buyer(Path(buyer): Path<String>) -> impl IntoResponse {
    let response_body;

    match purchases(buyer.to_lowercase()).await {
        Ok(purchases) => {
            let purchases_by_address = purchases;
            println!(
                "Purchases by buyer {:?} - {:?}",
                purchases_by_address,
                buyer.to_lowercase()
            );

            let response_body = serde_json::to_string(&purchases_by_address).unwrap();

            Response::new(response_body).into_response()
        }
        Err(e) => {
            println!("An error occurred: {}", e);
            response_body = e.to_string();
            Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(response_body)
                .unwrap()
                .into_response()
        }
    }
}

async fn listings(by_owner: Option<String>) -> Result<Vec<Listing>> {
    let query = r#"
        query ListingsQuery {
                lists {
                    id
                    owner
                    tokenId
                    nftContractAddress
                    price
                    isActive
                    blockTimestamp
                    blockNumber
                    transactionHash
            }
        }
   "#;

    let vars = Vars {};
    let client = Client::new(SUBGRAPH_ENDPOINT);
    let data = client
        .query_with_vars::<ListingsData, Vars>(query, vars)
        .await
        .unwrap();

    if let Some(owner) = by_owner {
        let listings: Vec<Listing> = data
            .lists
            .into_iter()
            .filter(|list| list.owner == owner)
            .filter(|list| list.is_active == true)
            .collect();

        if listings.len() == 0 {
            return Err(anyhow!(format!("No active listings for owner {}", owner)));
        }

        Ok(listings)
    } else {
        if data.lists.len() == 0 {
            return Err(anyhow!("No active listings found"));
        } else {
            let listings: Vec<Listing> = data
            .lists
            .into_iter()
            .filter(|list| list.is_active == true)
            .collect();

            if listings.len() == 0 {
                return Err(anyhow!(format!("No active listings found",)));
            }

            Ok(listings)
        }
    }
}

async fn purchases(address: String) -> Result<Vec<Purchase>> {
    let query = r#"
        query BuysQuery {
                buys {
                    id
                    buyer
                    owner
                    tokenId
                    nftContractAddress
                    price
                    blockTimestamp
                    blockNumber
                    transactionHash
            }
        }
    "#;

    let vars = Vars {};
    let client = Client::new(SUBGRAPH_ENDPOINT);
    let data = client
        .query_with_vars::<PurchasesData, Vars>(query, vars)
        .await
        .unwrap();

    let buys_per_address: Vec<Purchase> = data
        .buys
        .into_iter()
        .filter(|buy| buy.buyer == address)
        .collect();

    if buys_per_address.len() == 0 {
        return Err(anyhow!(format!("No buys for address {}", address)));
    } else {
        Ok(buys_per_address)
    }
}

async fn traded_volume(nft_contract_address: String) -> i64 {
    let query = r#"
    query BuysQuery {
            buys {
                id
                buyer
                owner
                tokenId
                nftContractAddress
                price
                blockTimestamp
                blockNumber
                transactionHash
        }
    }
"#;

    let vars = Vars {};
    let client = Client::new(SUBGRAPH_ENDPOINT);
    let data = client
        .query_with_vars::<PurchasesData, Vars>(query, vars)
        .await
        .unwrap();

    let buys_per_contract_address: Vec<Purchase> = data
        .buys
        .into_iter()
        .filter(|buy| buy.nft_contract_address == nft_contract_address)
        .collect();

    let total_volume: i64 = buys_per_contract_address
        .iter()
        .map(|buy| buy.price.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
        .iter()
        .sum();

    total_volume
}

async fn floor_price(nft_contract_address: String) -> Result<i64> {
    let query = r#"
        query ListingsQuery {
                lists {
                    id
                    owner
                    tokenId
                    nftContractAddress
                    price
                    blockTimestamp
                    blockNumber
                    transactionHash
            }
        }
   "#;

    let vars = Vars {};
    let client = Client::new(SUBGRAPH_ENDPOINT);
    let data = client
        .query_with_vars::<ListingsData, Vars>(query, vars)
        .await
        .unwrap();

    let listings: Vec<Listing> = data
        .lists
        .into_iter()
        .filter(|list| list.nft_contract_address == nft_contract_address)
        .collect();

    println!("Listings: {:?}", listings);

    if listings.len() == 0 {
        return Err(anyhow!(format!(
            "No collection with id {} found",
            nft_contract_address
        )));
    }

    let lowest_price = listings
        .iter()
        .filter_map(|listing| listing.price.parse::<i64>().ok())
        .min()
        .unwrap();

    Ok(lowest_price)
}

#[tokio::main]
async fn main() {
    let host = "127.0.0.1:8085";
    start_server(host).await;
}
