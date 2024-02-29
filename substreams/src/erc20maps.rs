use crate::pb::debbie::{Erc20Transfer, Erc20Transfers, MasterProto};
use substreams::scalar::BigInt;
use substreams::store::{StoreGet, StoreGetBigInt};

#[substreams::handlers::map]
pub fn map_erc20_transfer_vol(transfers: MasterProto,store: StoreGetBigInt) -> Result<Erc20Transfers, substreams::errors::Error> {
    let mut erc20_transfers = Vec::new();
    for mut transfer in transfers.erc20transfers {
        if let Some(volume_bigint) = store.get_at(0, &transfer.address) {
            transfer.volume = volume_bigint.to_string();
            erc20_transfers.push(transfer);
        }
    }
    Ok(Erc20Transfers {transfers: erc20_transfers})
}
