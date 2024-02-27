mod abi;
mod deployments;
mod pb;
use pb::debbie;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::pb::eth::v2::Call;
use substreams_ethereum::pb::sf::ethereum::r#type::v2 as eth;

pub struct ContractCreation {
    pub address: String,
    pub bytecode: String,
    pub abi: String,
}

#[substreams::handlers::map]
fn map_erc20_deployments(
    blk: Block,
) -> Result<Option<debbie::Erc20Deployments>, substreams::errors::Error> {
    let _erc20_deployments: Vec<_> = blk
        .transaction_traces
        .into_iter()
        .filter(|tx| tx.status == 1)
        .flat_map(|tx| {
            tx.calls
                .into_iter()
                .filter(|call| !call.state_reverted)
                .filter(|call| call.call_type == eth::CallType::Create as i32)
                .filter(|call| is_erc20(&call))
        })
        .collect();
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
