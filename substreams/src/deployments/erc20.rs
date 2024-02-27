use primitive_types::H256;
use std::collections::HashMap;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::Call;
pub struct ERC20Creation {
    address: Vec<u8>,
    code: Vec<u8>,
    storage_changes: HashMap<H256, Vec<u8>>,
}

impl ERC20Creation {
    pub fn new(address: Vec<u8>, code: Vec<u8>, storage_changes: HashMap<H256, Vec<u8>>) -> Self {
        Self {
            address,
            code,
            storage_changes,
        }
    }

    pub fn is_erc20(call: &Call) -> bool {
        let code = Hex::encode(&call.code_changes.iter().last().unwrap().new_code);

        if code.contains("07ba2a17") {
            return true;
        }

        return false;
        // substreams::log::info!("code: {:?}", code);
        // None
    }
}
