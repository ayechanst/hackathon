mod abi;
mod erc20maps;
mod erc20stores;
mod erc721maps;
mod erc721stores;
mod helpers;
mod pb;
use abi::erc20::events::Transfer as Erc20TransferEvent;
use abi::erc721::events::Transfer as Erc721TransferEvent;
use helpers::erc20helpers::*;
use helpers::erc721helpers::*;

use pb::debbie::Erc20HoldersTransfers;
use pb::debbie::Erc721TransfersHoldersTokens;
use pb::debbie::{Erc20Deployment, Erc20Transfer, Erc721Deployments, MasterProto};
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
use uuid::timestamp;

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

// #[substreams::handlers::map]
// fn graph_out(
//     clock: Clock,
//     master: MasterProto,
//     transfers_and_holders: Erc20HoldersTransfers,
//     erc721_transfers_holders_tokens: Erc721TransfersHoldersTokens,
// ) -> Result<EntityChanges, substreams::errors::Error> {
//     let mut tables = Tables::new();
//     let timestamp_seconds = clock.timestamp.unwrap().seconds;
//     for erc20_deployment in master.erc20contracts {
//         let total_supply: BigInt;
//         if let Some(supply) = BigInt::from_str(&erc20_deployment.total_supply).ok() {
//             total_supply = supply;
//         } else {
//             total_supply = BigInt::from(0);
//         }

//         let decimals: BigInt;
//         if let Some(dec) = BigInt::from_str(&erc20_deployment.decimals).ok() {
//             decimals = dec;
//         } else {
//             decimals = BigInt::from(0);
//         }

//         let blocknumber: BigInt;
//         if let Some(block) = BigInt::from_str(&erc20_deployment.blocknumber).ok() {
//             blocknumber = block;
//         } else {
//             blocknumber = BigInt::from(0);
//         }

//         tables
//             .create_row("TokenDeployment", erc20_deployment.address)
//             .set("name", erc20_deployment.name)
//             .set("symbol", erc20_deployment.symbol)
//             .set("totalSupply", total_supply)
//             .set("decimals", decimals)
//             .set("blocknumber", blocknumber)
//             .set("timestamp", BigInt::from(timestamp_seconds));

//         // tables.create_row("Token", erc20_deployment.address)
//     }

//     for erc721_deployment in master.erc721contracts {
//         let blocknumber: BigInt;
//         if let Some(block) = BigInt::from_str(&erc721_deployment.blocknumber).ok() {
//             blocknumber = block;
//         } else {
//             blocknumber = BigInt::from(0);
//         }
//         tables
//             .create_row("NftDeployment", erc721_deployment.address)
//             .set("name", erc721_deployment.name)
//             .set("symbol", erc721_deployment.symbol)
//             .set("blocknumber", blocknumber)
//             .set("tokenUri", erc721_deployment.token_uri)
//             .set("timestamp", BigInt::from(timestamp_seconds));
//     }

//     for (index, erc20_transfer) in transfers_and_holders.erc20transfers.iter().enumerate() {
//         let amount: BigInt;
//         if let Some(amt) = BigInt::from_str(&erc20_transfer.amount).ok() {
//             amount = amt;
//         } else {
//             amount = BigInt::from(0);
//         }

//         let count: BigInt;
//         if let Some(cnt) = BigInt::from_str(&erc20_transfer.count).ok() {
//             count = cnt;
//         } else {
//             count = BigInt::from(0);
//         }

//         let volume: BigInt;
//         if let Some(vol) = BigInt::from_str(&erc20_transfer.volume).ok() {
//             volume = vol;
//         } else {
//             volume = BigInt::from(0);
//         }

//         let blocknumber: BigInt;
//         if let Some(block) = BigInt::from_str(&erc20_transfer.blocknumber).ok() {
//             blocknumber = block;
//         } else {
//             blocknumber = BigInt::from(0);
//         }

//         tables
//             .create_row(
//                 "TokenTransfer",
//                 format!(
//                     "{}:{}:{}:{}:{}:{}",
//                     erc20_transfer.from,
//                     erc20_transfer.to,
//                     erc20_transfer.amount,
//                     erc20_transfer.count,
//                     erc20_transfer.blocknumber,
//                     index
//                 ),
//             )
//             .set("from", String::from(erc20_transfer.from.clone()))
//             .set("to", erc20_transfer.to.clone())
//             .set("amount", amount)
//             .set("count", count)
//             .set("volume", volume)
//             .set("blocknumber", blocknumber)
//             .set("timestamp", BigInt::from(timestamp_seconds));
//     }

//     for (index, erc721_transfer) in erc721_transfers_holders_tokens.transfers.iter().enumerate() {
//         let volume: BigInt;
//         if let Some(vol) = BigInt::from_str(&erc721_transfer.volume).ok() {
//             volume = vol;
//         } else {
//             volume = BigInt::from(0);
//         }

//         let blocknumber: BigInt;
//         if let Some(block) = BigInt::from_str(&erc721_transfer.blocknumber).ok() {
//             blocknumber = block;
//         } else {
//             blocknumber = BigInt::from(0);
//         }

//         tables
//             .create_row(
//                 "NftTransfer",
//                 format!(
//                     "{}:{}:{}:{}:{}:{}",
//                     erc721_transfer.from,
//                     erc721_transfer.to,
//                     erc721_transfer.token_id,
//                     erc721_transfer.volume,
//                     erc721_transfer.blocknumber,
//                     index
//                 ),
//             )
//             .set("from", erc721_transfer.from.clone())
//             .set("to", erc721_transfer.to.clone())
//             .set("tokenId", erc721_transfer.token_id.clone())
//             .set("volume", volume)
//             .set("blocknumber", blocknumber)
//             .set("timestamp", BigInt::from(timestamp_seconds));
//     }

//     for token_holder in transfers_and_holders.token_holders {
//         let token_balance: BigInt;
//         if let Some(balance) = BigInt::from_str(&token_holder.balance).ok() {
//             token_balance = balance;
//         } else {
//             token_balance = BigInt::from(0);
//         }

//         let transfer_volume: BigInt;
//         if let Some(volume) = BigInt::from_str(&token_holder.transfer_volume).ok() {
//             transfer_volume = volume;
//         } else {
//             transfer_volume = BigInt::from(0);
//         }

//         let transfer_count: BigInt;
//         if let Some(count) = BigInt::from_str(&token_holder.transfer_count).ok() {
//             transfer_count = count;
//         } else {
//             transfer_count = BigInt::from(0);
//         }

//         tables
//             .update_row(
//                 "TokenHolder",
//                 format!(
//                     "{}:{}",
//                     token_holder.holder_address, token_holder.token_address
//                 ),
//             )
//             .set("holderAddress", token_holder.holder_address)
//             .set("tokenAddress", token_holder.token_address)
//             .set("balance", token_balance)
//             .set("transferVolume", transfer_volume)
//             .set("transferCount", transfer_count)
//             .set("timestamp", BigInt::from(timestamp_seconds));
//     }

//     for token_holder in erc721_transfers_holders_tokens.erc721_token_holders {
//         let token_balance: BigInt;
//         if let Some(balance) = BigInt::from_str(&token_holder.token_balance).ok() {
//             token_balance = balance;
//         } else {
//             token_balance = BigInt::from(0);
//         }

//         tables
//             .update_row(
//                 "NftHolder",
//                 format!(
//                     "{}:{}",
//                     token_holder.holder_address, token_holder.token_address
//                 ),
//             )
//             .set("holderAddress", token_holder.holder_address)
//             .set("nftAddress", token_holder.token_address)
//             .set("tokenBalance", token_balance)
//             .set("timestamp", BigInt::from(timestamp_seconds));
//     }

//     for token in erc721_transfers_holders_tokens.erc721_tokens {
//         let volume: BigInt;
//         if let Some(vol) = BigInt::from_str(&token.transfer_volume).ok() {
//             volume = vol;
//         } else {
//             volume = BigInt::from(0);
//         }

//         tables
//             .create_row(
//                 "NftToken",
//                 format!("{}:{}", token.token_address, token.token_id),
//             )
//             .set("address", token.token_address)
//             .set("tokenId", token.token_id)
//             .set("volume", volume)
//             .set("timestamp", BigInt::from(timestamp_seconds));
//     }

//     Ok(tables.to_entity_changes())
// }

#[substreams::handlers::map]
fn graph_out(
    clock: Clock,
    master: MasterProto,
    transfers_and_holders: Erc20HoldersTransfers,
    erc721_transfers_holders_tokens: Erc721TransfersHoldersTokens,
) -> Result<EntityChanges, substreams::errors::Error> {
    let mut tables = Tables::new();
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let month_id = timestamp_seconds / 2592000;
    let blocknumber = BigInt::from_str(&clock.number.to_string()).unwrap();
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
            .create_row("TokenDeployment", &erc20_deployment.address)
            .set("blocknumber", blocknumber)
            .set("timestamp", BigInt::from(timestamp_seconds))
            .set("token", &erc20_deployment.address);

        tables
            .create_row("Token", &erc20_deployment.address)
            .set("name", erc20_deployment.name)
            .set("symbol", erc20_deployment.symbol)
            .set("decimals", decimals)
            .set("totalSupply", total_supply);
    }

    for erc721_deployment in master.erc721contracts {
        tables
            .create_row("NftDeployment", &erc721_deployment.address)
            .set(
                "blocknumber",
                BigInt::from_str(&erc721_deployment.blocknumber).unwrap_or(BigInt::from(0)),
            )
            .set(
                "timestamp",
                BigInt::from(erc721_deployment.timestamp_seconds),
            )
            .set("nft", &erc721_deployment.address);

        tables
            .create_row("Nft", &erc721_deployment.address)
            .set("name", erc721_deployment.name)
            .set("symbol", erc721_deployment.symbol)
            .set("tokenUri", erc721_deployment.token_uri);
    }

    for (index, erc20_transfer) in transfers_and_holders.erc20transfers.iter().enumerate() {
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
            .set("from", &erc20_transfer.from)
            .set("to", &erc20_transfer.to)
            .set("amount", &amount)
            .set("count", &count)
            .set("volume", &volume)
            .set("token", &erc20_transfer.address)
            .set("blocknumber", &blocknumber)
            .set("timestamp", BigInt::from(timestamp_seconds))
            .set(
                "tokenHolderTo",
                &format!("{}:{}", &erc20_transfer.to, erc20_transfer.address),
            )
            .set(
                "tokenHolderFrom",
                &format!("{}:{}", &erc20_transfer.from, erc20_transfer.address),
            );
    }

    for (index, erc721_transfer) in erc721_transfers_holders_tokens.transfers.iter().enumerate() {
        let volume: BigInt;
        if let Some(vol) = BigInt::from_str(&erc721_transfer.volume).ok() {
            volume = vol;
        } else {
            volume = BigInt::from(0);
        }

        let blocknumber: BigInt;
        if let Some(block) = BigInt::from_str(&erc721_transfer.blocknumber).ok() {
            blocknumber = block;
        } else {
            blocknumber = BigInt::from(0);
        }

        tables
            .create_row(
                "NftTransfer",
                format!(
                    "{}:{}:{}:{}:{}:{}",
                    erc721_transfer.from,
                    erc721_transfer.to,
                    erc721_transfer.token_id,
                    erc721_transfer.volume,
                    erc721_transfer.blocknumber,
                    index
                ),
            )
            .set("from", erc721_transfer.from.clone())
            .set("to", erc721_transfer.to.clone())
            .set("tokenId", erc721_transfer.token_id.clone())
            .set("volume", volume)
            .set("blocknumber", blocknumber)
            .set("timestamp", BigInt::from(timestamp_seconds))
            .set(
                "nftHolderTo",
                &format!("{}:{}", &erc721_transfer.to, &erc721_transfer.address),
            )
            .set(
                "nftHolderFrom",
                &format!("{}:{}", &erc721_transfer.from, &erc721_transfer.address),
            );
    }

    for token_holder in transfers_and_holders.token_holders {
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
                    &token_holder.holder_address, &token_holder.token_address
                ),
            )
            .set("holderAddress", &token_holder.holder_address)
            .set("token", &token_holder.token_address)
            .set("timestamp", BigInt::from(timestamp_seconds));

        tables
            .update_row(
                "TokenHolderSnapshot",
                format!(
                    "{}:{}:{}",
                    &token_holder.holder_address, &token_holder.token_address, month_id
                ),
            )
            .set(
                "tokenHolder",
                &format!(
                    "{}:{}",
                    &token_holder.holder_address, &token_holder.token_address
                ),
            )
            .set("balance", token_balance)
            .set("transferVolume", transfer_volume)
            .set("transferCount", transfer_count)
            .set("timestamp", BigInt::from(token_holder.timestamp_seconds))
            .set(
                "blocknumber",
                BigInt::from_str(&token_holder.blocknumber).unwrap_or(BigInt::from(0)),
            )
            .set("monthId", BigInt::from(token_holder.month_id));
    }

    for token_holder in erc721_transfers_holders_tokens.erc721_token_holders {
        tables
            .update_row(
                "NftHolder",
                format!(
                    "{}:{}",
                    token_holder.holder_address, token_holder.token_address
                ),
            )
            .set("holderAddress", &token_holder.holder_address)
            .set("nft", &token_holder.token_address)
            .set("timestamp", BigInt::from(token_holder.timestamp_seconds));

        tables
            .update_row(
                "NftHolderSnapshot",
                format!(
                    "{}:{}:{}",
                    &token_holder.holder_address, &token_holder.token_address, month_id
                ),
            )
            .set(
                "nftHolder",
                format!(
                    "{}:{}",
                    &token_holder.holder_address, &token_holder.token_address
                ),
            )
            .set(
                "tokenBalance",
                BigInt::from_str(&token_holder.token_balance).unwrap_or(BigInt::one()),
            )
            .set("timestamp", BigInt::from(token_holder.timestamp_seconds))
            .set(
                "blocknumber",
                BigInt::from_str(&token_holder.blocknumber).unwrap_or(BigInt::from(0)),
            )
            .set("monthId", BigInt::from(token_holder.month_id));
    }

    for token in erc721_transfers_holders_tokens.erc721_tokens {
        tables
            .create_row(
                "NftToken",
                format!("{}:{}", token.token_address, token.token_id),
            )
            .set("address", &token.token_address)
            .set("tokenId", &token.token_id)
            .set(
                "volume",
                BigInt::from_str(&token.transfer_volume).unwrap_or(BigInt::one()),
            )
            .set("nft", &token.token_address)
            .set("timestamp", BigInt::from(token.timestamp_seconds));
    }

    Ok(tables.to_entity_changes())
}
