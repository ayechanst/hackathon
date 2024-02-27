mod abi;
mod helpers;
mod pb;
use pb::debbie::{Erc20Deployment, Erc20Deployments, MasterProto};
use primitive_types::H256;
use std::collections::HashMap;
use std::string;
use substreams::pb::substreams::Clock;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::pb::eth::v2::Call;
use substreams_ethereum::pb::sf::ethereum::r#type::v2 as eth;
use abi::erc20::events as erc20;
use abi::erc721::events as erc721;

pub struct ContractCreation {
    pub address: String,
    pub bytecode: String,
    pub abi: String,
}

pub struct ERC20Creation {
    address: Vec<u8>,
    code: Vec<u8>,
    storage_changes: HashMap<H256, Vec<u8>>,
}

impl ERC20Creation {
    pub fn from_call(
        address: Vec<u8>,
        code: Vec<u8>,
        storage_changes: HashMap<H256, Vec<u8>>,
    ) -> Option<Self> {
        let code_string = Hex::encode(&code);
        if code_string.contains("06fdde03")
            && code_string.contains("95d89b41")
            && code_string.contains("313ce567")
            && code_string.contains("18160ddd")
        {
            Some(Self {
                address,
                code,
                storage_changes,
            })
        } else {
            None
        }
    }
}

#[substreams::handlers::map]
fn map_erc20_deployments(
    blk: Block,
    clk: Clock,
) -> Result<Erc20Deployments, substreams::errors::Error> {
    let mut deployment_contracts: Vec<Erc20Deployment> = Vec::new();
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
            let storage_changes = call
                .storage_changes
                .into_iter()
                .map(|s| (H256::from_slice(s.key.as_ref()), s.new_value))
                .collect();
            if let Some(token) = ERC20Creation::from_call(address, code.to_vec(), storage_changes) {
                let deployment = substream_test_data(token, clk.number.to_string());
                deployment_contracts.push(deployment);
            }
        }
    }
    Ok(Erc20Deployments {
        contracts: deployment_contracts,
    })
}



pub fn substream_test_data(contract: ERC20Creation, blocknumber: String) -> Erc20Deployment {
    Erc20Deployment {
        address: Hex::encode(contract.address),
        name: String::from("DebbieCoin"),
        symbol: String::from("DBC"),
        total_supply: String::from("100,000,000"),
        blocknumber,
    }
}
