mod abi;
mod erc20maps;
mod erc20stores;
mod erc721maps;
mod erc721stores;
mod helpers;
mod pb;
use abi::erc20::events::Transfer as Erc20TransferEvent;
use abi::erc20::functions::TotalSupply;
use abi::erc721::events::Transfer as Erc721TransferEvent;
use helpers::erc20helpers::*;
use helpers::erc721helpers::*;
use pb::debbie::{
    Erc20Deployment, Erc20Transfer, Erc20Transfers, Erc721Deployments, MasterProto, TokenHolders,
};
use pb::debbie::{Erc721Deployment, Erc721Transfer};
use primitive_types::H256;
use std::collections::HashMap;
use std::str::FromStr;
use substreams::pb::substreams::Clock;
use substreams::scalar::BigInt;
use substreams::Hex;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;
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
                } else if let Some(token) = ERC721Creation::from_call(all_calls) {
                    if let Some(deployment) = process_erc721_contract(token, clk.clone()) {
                        erc721_contracts.push(deployment);
                    }
                }
            }
        }
        let block_num = clk.number.to_string();
        for log in &call.logs {
            if let Some(erc20Transfer) = Erc20TransferEvent::match_and_decode(log) {
                erc20_transfers.push(Erc20Transfer {
                    address: Hex::encode(&log.address),
                    from: Hex::encode(erc20Transfer.from),
                    to: Hex::encode(erc20Transfer.to),
                    amount: erc20Transfer.value.to_string(),
                    count: String::from("1"),
                    volume: String::new(),
                    blocknumber: String::from(&block_num),
                });
            } else if let Some(erc721Transfer) = Erc721TransferEvent::match_and_decode(log) {
                erc721_transfers.push(Erc721Transfer {
                    address: Hex::encode(&log.address),
                    from: Hex::encode(erc721Transfer.from),
                    to: Hex::encode(erc721Transfer.to),
                    token_id: erc721Transfer.token_id.to_string(),
                    volume: String::new(),
                    blocknumber: String::from(&block_num),
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

pub fn erc20_test_data(contract: ERC20Creation, blocknumber: String) -> Erc20Deployment {
    Erc20Deployment {
        address: Hex::encode(contract.address),
        name: String::from("DebbieCoin"),
        symbol: String::from("DBC"),
        total_supply: String::from("100,000,000"),
        decimals: String::from("18"),
        blocknumber,
    }
}

pub fn erc721_test_data(contract: ERC721Creation) -> Erc721Deployment {
    Erc721Deployment {
        address: Hex::encode(contract.address),
        name: String::from("debbie road surf club"),
        symbol: String::from("DRSC"),
        blocknumber: String::new(),
    }
}

#[substreams::handlers::map]
fn map_delegates(blk: Block) -> Erc20Deployment {
    let delegates: Vec<_> = blk
        .transaction_traces
        .into_iter()
        .filter(|tx| tx.status == 1)
        .flat_map(|tx| {
            tx.calls
                .into_iter()
                .filter(|call| !call.state_reverted)
                .filter(|call| call.call_type == eth::CallType::Delegate as i32)
        })
        .collect();
    for delegatecall in delegates {
        let caller = Hex::encode(delegatecall.caller);
        let addy = Hex::encode(delegatecall.address);
        substreams::log::info!("caller {:?}", caller);
        substreams::log::info!("addy {:?}", addy);
    }
    Erc20Deployment {
        address: "".to_string(),
        name: "".to_string(),
        symbol: "".to_string(),
        total_supply: "".to_string(),
        decimals: "".to_string(),
        blocknumber: "".to_string(),
    }
}

#[substreams::handlers::map]
fn map_erc721_test(master: MasterProto) -> Erc721Deployments {
    let mut deployments = Vec::new();
    for deployment in master.erc721contracts {
        deployments.push(deployment);
    }
    Erc721Deployments {
        contracts: deployments,
    }
}

#[substreams::handlers::map]
fn graph_out(
    master: MasterProto,
    erc20_transfers: Erc20Transfers,
    user_erc20_data: TokenHolders,
) -> Result<EntityChanges, substreams::errors::Error> {
    let mut tables = Tables::new();

    for erc20_deployment in master.erc20contracts {
        let total_supply: BigInt;
        if let Some(supply) = BigInt::from_str(&erc20_deployment.total_supply).ok() {
            total_supply = supply;
        } else {
            total_supply = BigInt::from(0);
        }

        let decimals: BigInt;
        if let Some(dec) = BigInt::from_str(&erc20_deployment.decimals).ok() {
            decimals = dec;
        } else {
            decimals = BigInt::from(0);
        }

        let blocknumber: BigInt;
        if let Some(block) = BigInt::from_str(&erc20_deployment.blocknumber).ok() {
            blocknumber = block;
        } else {
            blocknumber = BigInt::from(0);
        }

        tables
            .create_row("TokenDeployment", erc20_deployment.address)
            .set("name", erc20_deployment.name)
            .set("symbol", erc20_deployment.symbol)
            .set("totalSupply", total_supply)
            .set("decimals", decimals)
            .set("blocknumber", blocknumber);
    }

    for (index, erc20_transfer) in erc20_transfers.transfers.iter().enumerate() {
        let amount: BigInt;
        if let Some(amt) = BigInt::from_str(&erc20_transfer.amount).ok() {
            amount = amt;
        } else {
            amount = BigInt::from(0);
        }

        let count: BigInt;
        if let Some(cnt) = BigInt::from_str(&erc20_transfer.count).ok() {
            count = cnt;
        } else {
            count = BigInt::from(0);
        }

        let volume: BigInt;
        if let Some(vol) = BigInt::from_str(&erc20_transfer.volume).ok() {
            volume = vol;
        } else {
            volume = BigInt::from(0);
        }

        let blocknumber: BigInt;
        if let Some(block) = BigInt::from_str(&erc20_transfer.blocknumber).ok() {
            blocknumber = block;
        } else {
            blocknumber = BigInt::from(0);
        }

        tables
            .create_row(
                "TokenTransfer",
                format!(
                    "{}:{}:{}:{}:{}:{}",
                    erc20_transfer.from,
                    erc20_transfer.to,
                    erc20_transfer.amount,
                    erc20_transfer.count,
                    erc20_transfer.blocknumber,
                    index
                ),
            )
            .set("from", String::from(erc20_transfer.from.clone()))
            .set("to", erc20_transfer.to.clone())
            .set("amount", amount)
            .set("count", count)
            .set("volume", volume)
            .set("blocknumber", blocknumber);
    }

    for token_holder in user_erc20_data.token_holders {
        let token_balance: BigInt;
        if let Some(balance) = BigInt::from_str(&token_holder.balance).ok() {
            token_balance = balance;
        } else {
            token_balance = BigInt::from(0);
        }

        let transfer_volume: BigInt;
        if let Some(volume) = BigInt::from_str(&token_holder.transfer_volume).ok() {
            transfer_volume = volume;
        } else {
            transfer_volume = BigInt::from(0);
        }

        let transfer_count: BigInt;
        if let Some(count) = BigInt::from_str(&token_holder.transfer_count).ok() {
            transfer_count = count;
        } else {
            transfer_count = BigInt::from(0);
        }

        tables
            .update_row(
                "TokenHolder",
                format!(
                    "{}:{}",
                    token_holder.holder_address, token_holder.token_address
                ),
            )
            .set("holderAddress", token_holder.holder_address)
            .set("tokenAddress", token_holder.token_address)
            .set("balance", token_balance)
            .set("transferVolume", transfer_volume)
            .set("transferCount", transfer_count);
    }

    Ok(tables.to_entity_changes())
}

//unique key generator
