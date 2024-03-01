use crate::pb::debbie::MasterProto;
use substreams::pb::substreams::Clock;
use substreams::scalar::BigInt;
use substreams::store::{StoreAdd, StoreAddBigInt, StoreDelete, StoreNew};

#[substreams::handlers::store]
pub fn store_erc721_transfer_vol(transfers: MasterProto, store: StoreAddBigInt) {
    for transfer in transfers.erc721transfers {
        store.add(0, transfer.address, BigInt::one())
    }
}

#[substreams::handlers::store]
pub fn store_user_erc721_balance(clock: Clock, transfers: MasterProto, store: StoreAddBigInt) {
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let month_id = timestamp_seconds / 2592000;
    let prev_month_id = month_id - 1;

    store.delete_prefix(0, &format!("UserErc721BalanceMonth:{}", prev_month_id));

    for transfer in transfers.erc721transfers {
        if transfer.to != "00000000000000000000".to_string() {
            store.add(
                0,
                format!(
                    "UserErc721BalanceMonth:{}:{}:{}",
                    month_id, transfer.to, transfer.address
                ),
                BigInt::one(),
            )
        }
        if transfer.from != "00000000000000000000".to_string() {
            store.add(
                0,
                format!(
                    "UserErc721BalanceMonth:{}:{}:{}",
                    month_id, transfer.from, transfer.address
                ),
                BigInt::from(-1),
            )
        }
    }
}

#[substreams::handlers::store]
pub fn store_erc721_token_vol(clock: Clock, transfers: MasterProto, store: StoreAddBigInt) {
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let month_id = timestamp_seconds / 2592000;
    let prev_month_id = month_id - 1;

    store.delete_prefix(0, &format!("Erc721TokenVolMonth:{}", prev_month_id));

    for transfer in transfers.erc721transfers {
        store.add(
            0,
            format!(
                "Erc721TokenVolMonth:{}:{}:{}",
                month_id, transfer.address, transfer.token_id
            ),
            BigInt::one(),
        )
    }
}
