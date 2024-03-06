mod abi;
mod erc20maps;
mod erc20stores;
mod erc721maps;
mod erc721stores;
mod graphout;
mod helpers;
mod pb;
use helpers::erc20helpers::*;
use helpers::erc721helpers::*;
use pb::debbie::{Deployments, Erc20Deployment, Erc721Deployment};
use primitive_types::H256;
use std::collections::HashMap;
use std::str::FromStr;
use substreams::pb::substreams::Clock;
use substreams::scalar::BigInt;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::pb::sf::ethereum::r#type::v2 as eth;

pub use erc20maps::*;
pub use erc20stores::*;
pub use erc721maps::*;
pub use erc721stores::*;

pub struct ContractCreation {
    pub address: String,
    pub bytecode: String,
    pub abi: String,
}

#[substreams::handlers::map]
fn map_deployments(blk: Block, clk: Clock) -> Result<EntityChanges, substreams::errors::Error> {
    // // let mut erc20_deployments: Vec<Erc20Deployment> = Vec::new();
    // let mut erc721_deployments: Vec<Erc721Deployment> = Vec::new();
    let mut tables = Tables::new();

    let filtered_calls: Vec<_> = blk
        .transaction_traces
        .into_iter()
        .filter(|tx| tx.status == 1)
        .flat_map(|tx| {
            tx.calls
                .into_iter()
                .filter(|call| !call.state_reverted)
                .filter(|call| call.call_type == eth::CallType::Create as i32)
        })
        .collect();
    for call in filtered_calls {
        if call.call_type == eth::CallType::Create as i32 {
            let mut all_calls = Vec::new();
            all_calls.push(&call);
            if let Some(last_code_change) = call.code_changes.iter().last() {
                let code = &last_code_change.new_code;
                let address = &call.address.to_vec();
                let token_uri = get_token_uri(&call);
                let storage_changes: HashMap<H256, Vec<u8>> = call
                    .storage_changes
                    .iter()
                    .map(|s| (H256::from_slice(s.key.as_ref()), s.new_value.clone()))
                    .collect();
                if let Some(token) =
                    ERC20Creation::from_call(&address, code.to_vec(), storage_changes.clone())
                {
                    if let Some(deployment) = process_erc20_contract(token, clk.clone()) {
                        // erc20_deployments.push(deployment);
                        tables
                            .update_row("TokenDeployment", deployment.address)
                            .set("name", deployment.name)
                            .set("symbol", deployment.symbol)
                            .set(
                                "decimals",
                                BigInt::from_str(&deployment.decimals).unwrap_or(BigInt::zero()),
                            )
                            .set(
                                "totalSupply",
                                BigInt::from_str(&deployment.total_supply)
                                    .unwrap_or(BigInt::zero()),
                            )
                            .set(
                                "blocknumber",
                                BigInt::from_str(&deployment.blocknumber).unwrap(),
                            )
                            .set("timestamp", deployment.timestamp_seconds);
                    }
                } else if let Some(token) = ERC721Creation::from_call(all_calls, token_uri) {
                    if let Some(deployment) = process_erc721_contract(token, clk.clone()) {
                        // erc721_deployments.push(deployment);
                        tables
                            .update_row("NftDeployment", deployment.address)
                            .set("name", deployment.name)
                            .set("symbol", deployment.symbol)
                            .set("tokenUri", deployment.token_uri)
                            .set(
                                "blocknumber",
                                BigInt::from_str(&deployment.blocknumber).unwrap(),
                            )
                            .set("timestamp", deployment.timestamp_seconds);
                    }
                }
            }
        }
    }

    Ok(tables.to_entity_changes())
}

#[substreams::handlers::map]
fn g_out(deployments: Deployments) -> Result<EntityChanges, substreams::errors::Error> {
    let mut tables = Tables::new();

    for token_deployment in deployments.token_deployments {
        tables
            .update_row("Erc20Deployment", token_deployment.address)
            .set("name", token_deployment.name)
            .set("symbol", token_deployment.symbol)
            .set(
                "decimals",
                BigInt::from_str(&token_deployment.decimals).unwrap_or(BigInt::zero()),
            )
            .set(
                "totalSupply",
                BigInt::from_str(&token_deployment.total_supply).unwrap_or(BigInt::zero()),
            )
            .set(
                "blocknumber",
                BigInt::from_str(&token_deployment.blocknumber).unwrap(),
            )
            .set("timestamp", token_deployment.timestamp_seconds);
    }

    for nft_deployment in deployments.nft_deployments {
        tables
            .update_row("Erc721Deployment", nft_deployment.address)
            .set("name", nft_deployment.name)
            .set("symbol", nft_deployment.symbol)
            .set("tokenUri", nft_deployment.token_uri)
            .set(
                "blocknumber",
                BigInt::from_str(&nft_deployment.blocknumber).unwrap(),
            )
            .set("timestamp", nft_deployment.timestamp_seconds);
    }

    Ok(tables.to_entity_changes())
}
