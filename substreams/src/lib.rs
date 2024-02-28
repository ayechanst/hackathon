mod abi;
mod helpers;
mod pb;
use abi::erc20::events as erc20;
use abi::erc721::events as erc721;
use helpers::erc20helpers::*;
use helpers::erc721helpers::*;
use pb::debbie::{Erc20Deployment, Erc20Transfer, MasterProto};
use pb::debbie::{Erc721Deployment, Erc721Transfer};
use primitive_types::H256;
use std::collections::HashMap;
use std::string;
use substreams::pb::substreams::Clock;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::Call;
use substreams_ethereum::pb::eth::v2::{Block, StorageChange};
use substreams_ethereum::pb::sf::ethereum::r#type::v2 as eth;

pub struct ContractCreation {
    pub address: String,
    pub bytecode: String,
    pub abi: String,
}

// pub struct ERC20Creation {
//     address: Vec<u8>,
//     code: Vec<u8>,
//     storage_changes: HashMap<H256, Vec<u8>>,
// }

// impl ERC20Creation {
//     pub fn from_call(
//         address: &Vec<u8>,
//         code: Vec<u8>,
//         storage_changes: HashMap<H256, Vec<u8>>,
//     ) -> Option<Self> {
//         let code_string = Hex::encode(&code);
//         if code_string.contains("06fdde03")
//             && code_string.contains("95d89b41")
//             && code_string.contains("313ce567")
//             && code_string.contains("18160ddd")
//         {
//             Some(Self {
//                 address: address.to_vec(),
//                 code,
//                 storage_changes,
//             })
//         } else {
//             None
//         }
//     }
// }

// pub struct ERC721Creation {
//     address: Vec<u8>,
//     code: Vec<u8>,
//     storage_changes: HashMap<H256, Vec<u8>>,
// }

// impl ERC721Creation {
//     pub fn from_call(
//         address: &Vec<u8>,
//         code: Vec<u8>,
//         storage_changes: HashMap<H256, Vec<u8>>,
//     ) -> Option<Self> {
//         let code_string = Hex::encode(&code);
//         if code_string.contains("06fdde03")
//             && code_string.contains("95d89b41")
//             && code_string.contains("c87b56dd")
//         {
//             Some(Self {
//                 address: address.to_vec(),
//                 code,
//                 storage_changes,
//             })
//         } else {
//             None
//         }
//     }
// }

#[substreams::handlers::map]
fn map_erc20_deployments(blk: Block, clk: Clock) -> Result<MasterProto, substreams::errors::Error> {
    let mut erc20_contracts: Vec<Erc20Deployment> = Vec::new();
    let mut erc721_contracts: Vec<Erc721Deployment> = Vec::new();
    let mut erc20_transfers: Vec<Erc20Transfer> = Vec::new();
    let mut erc721_transfers: Vec<Erc721Transfer> = Vec::new();
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
        if let Some(last_code_change) = call.code_changes.iter().last() {
            let code = &last_code_change.new_code;
            let address = call.address;
            let storage_changes: HashMap<H256, Vec<u8>> = call
                .storage_changes
                .into_iter()
                .map(|s| (H256::from_slice(s.key.as_ref()), s.new_value))
                .collect();
            if let Some(token) =
                ERC20Creation::from_call(&address, code.to_vec(), storage_changes.clone())
            {
                if let Some(deployment) = process_contract(token, clk.clone()) {
                    erc20_contracts.push(deployment);
                }
            } else if let Some(token) =
                ERC721Creation::from_call(&address, code.to_vec(), storage_changes.clone())
            {
                let deployment = erc721_test_data(token);
                erc721_contracts.push(deployment);
            }
        }
    }
    // Erc20Deployments {contracts: erc20_contracts};
    // Erc721Deployments {contracts: erc721_contracts};
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
