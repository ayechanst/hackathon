use crate::pb::debbie::MasterProto;
use substreams::scalar::BigInt;
use substreams::store::{StoreAdd, StoreAddBigInt, StoreNew};

#[substreams::handlers::store]
pub fn store_erc721_transfer_vol(transfers: MasterProto, store: StoreAddBigInt) {
    for transfer in transfers.erc721transfers {
        store.add(0, transfer.address, BigInt::from(1))
    }
}

#[substreams::handlers::store]
pub fn store_user_erc721_balance(transfers: MasterProto, store: StoreAddBigInt) {
    for transfer in transfers.erc721transfers {
        if transfer.to != "00000000000000000000".to_string() {
            store.add(
                0,
                format!("{}:{}", transfer.to, transfer.address),
                BigInt::from(1),
            )
        }
        if transfer.from != "00000000000000000000".to_string() {
            store.add(
                0,
                format!("{}:{}", transfer.from, transfer.address),
                BigInt::from(-1),
            )
        }
    }
}

#[substreams::handlers::store]
pub fn store_erc721_token_vol(transfers: MasterProto, store: StoreAddBigInt) {
    for transfer in transfers.erc721transfers {
        store.add(
            0,
            format!("{}:{}", transfer.address, transfer.token_id),
            BigInt::from(1),
        )
    }
}
