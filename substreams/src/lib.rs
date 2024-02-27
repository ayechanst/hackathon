mod abi;
mod deployments;
mod pb;
use pb::debbie;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::pb::eth::v2::Call;
use substreams_ethereum::pb::sf::ethereum::r#type::v2 as eth;
use primitive_types::H256;
use std::collections::HashMap;
use std::string;

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
    pub fn from_call(address: Vec<u8>, code: Vec<u8>, storage_changes: HashMap<H256, Vec<u8>>) -> Option<Self> {
        let code_string =  Hex::encode(code);
        if code_string.contains("06fdde03")
        && code_string.contains("95d89b41")
        && code_string.contains("313ce567")
        && code_string.contains("18160ddd") {
            Some(Self {
                address,
                code,
                storage_changes,
            })
        } else {
            return None
        }
        
    }
}


#[substreams::handlers::map]
fn map_erc20_deployments(
    blk: Block,
) -> Result<Option<debbie::Erc20Deployments>, substreams::errors::Error> {
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
        let code = &call.code_changes.iter().last().unwrap().new_code;
        let address = call.address;
        let storage_changes = call.storage_changes.into_iter()
        .map(|s| (H256::from_slice(s.key.as_ref()), s.new_value))
        .collect();
        if let Some(token) = ERC20Creation::from_call(address, code.to_vec(), storage_changes) {
        
    }
    }
Ok(None)
}

pub fn is_erc20(call: &&Call) -> bool {
    let code = Hex::encode(&call.code_changes.iter().last().unwrap().new_code);
    // substreams::log::info!("code: {:?}", code);

    if code.contains("06fdde03")
        && code.contains("95d89b41")
        && code.contains("313ce567")
        && code.contains("18160ddd")
    {
        substreams::log::info!("name found");
        substreams::log::info!("symbol found");
        substreams::log::info!("decimals found");
        substreams::log::info!("total supply found");
        substreams::log::info!("call addresss {:?}", Hex::encode(&call.address));
        return true;
    }
    return false;
}

pub fn substream_test_data(contract: ERC20Creation) -> Option<ERC20Deployment> {

}