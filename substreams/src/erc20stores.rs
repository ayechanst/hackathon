use std::str::FromStr;

use crate::pb::debbie::{Erc20Deployment, Erc20Transfer, MasterProto};
use crate::pb::debbie::{Erc721Deployment, Erc721Transfer};
use substreams::scalar::BigInt;
use substreams::store::{StoreAdd, StoreAddBigInt, StoreNew};
use substreams::Hex;

#[substreams::handlers::store]
pub fn store_erc20_transfer_vol(transfers: MasterProto, store: StoreAddBigInt) {
    for transfer in transfers.erc20transfers {
        store.add(
            0,
            transfer.address,
            BigInt::from_str(&transfer.amount).unwrap(),
        )
    }
}
