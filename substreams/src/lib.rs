mod abi;
mod helpers;
mod pb;
use helpers::erc20helpers::*;
use helpers::erc721helpers::*;
use primitive_types::H256;
use std::collections::HashMap;
use std::str::FromStr;
use substreams::pb::substreams::Clock;
use substreams::scalar::BigInt;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::pb::sf::ethereum::r#type::v2 as eth;

pub struct ContractCreation {
    pub address: String,
    pub bytecode: String,
    pub abi: String,
}

#[substreams::handlers::map]
fn map_deployments(blk: Block, clk: Clock) -> Result<EntityChanges, substreams::errors::Error> {
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
