mod abi;
mod erc20maps;
mod erc20stores;
mod erc721maps;
mod erc721stores;
mod graphout;
mod helpers;
mod pb;
use abi::erc20::events::Transfer as Erc20TransferEvent;
use abi::erc721::events::Transfer as Erc721TransferEvent;
use helpers::erc20helpers::*;
use helpers::erc721helpers::*;

use pb::debbie::{Erc20Deployment, Erc20Transfer, MasterProto};
use pb::debbie::{Erc721Deployment, Erc721Transfer};

use primitive_types::H256;
use std::collections::HashMap;
use substreams::pb::substreams::Clock;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::pb::sf::ethereum::r#type::v2 as eth;
use substreams_ethereum::Event;

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
fn map_blocks(blk: Block, clk: Clock) -> Result<MasterProto, substreams::errors::Error> {
    let mut erc20_contracts: Vec<Erc20Deployment> = Vec::new();
    let mut erc721_contracts: Vec<Erc721Deployment> = Vec::new();
    let mut erc20_transfers: Vec<Erc20Transfer> = Vec::new();
    let mut erc721_transfers: Vec<Erc721Transfer> = Vec::new();

    let filtered_calls: Vec<_> = blk
        .transaction_traces
        .into_iter()
        .filter(|tx| tx.status == 1)
        .flat_map(|tx| {
            tx.calls.into_iter().filter(|call| !call.state_reverted)
            // .filter(|call| call.call_type == eth::CallType::Create as i32)
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
                        erc20_contracts.push(deployment);
                    }
                } else if let Some(token) = ERC721Creation::from_call(all_calls, token_uri) {
                    if let Some(deployment) = process_erc721_contract(token, clk.clone()) {
                        erc721_contracts.push(deployment);
                    }
                }
            }
        }
        // let block_num = clk.number.to_string();
        for log in &call.logs {
            let clk = clk.clone();
            let block_num = clk.number.to_string();
            let timestamp_seconds = clk.timestamp.unwrap().seconds;

            if let Some(erc20_transfer) = Erc20TransferEvent::match_and_decode(log) {
                erc20_transfers.push(Erc20Transfer {
                    address: Hex::encode(&log.address),
                    from: Hex::encode(erc20_transfer.from),
                    to: Hex::encode(erc20_transfer.to),
                    amount: erc20_transfer.value.to_string(),
                    count: String::from("1"),
                    volume: String::new(),
                    blocknumber: String::from(block_num),
                    timestamp_seconds: timestamp_seconds.clone(),
                });
            } else if let Some(erc721_transfer) = Erc721TransferEvent::match_and_decode(log) {
                erc721_transfers.push(Erc721Transfer {
                    address: Hex::encode(&log.address),
                    from: Hex::encode(erc721_transfer.from),
                    to: Hex::encode(erc721_transfer.to),
                    token_id: erc721_transfer.token_id.to_string(),
                    volume: String::new(),
                    blocknumber: String::from(block_num),
                    timestamp_seconds: timestamp_seconds.clone(),
                });
            }
        }
    }

    Ok(MasterProto {
        erc20contracts: erc20_contracts,
        erc721contracts: erc721_contracts,
        erc20transfers: erc20_transfers,
        erc721transfers: erc721_transfers,
    })
}
