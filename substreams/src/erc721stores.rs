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
pub fn store_erc721_transfer_weekly(clock: Clock, transfers: MasterProto, store: StoreAddBigInt) {
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let week_id = timestamp_seconds / 604800;
    let prev_week_id = week_id - 1;

    store.delete_prefix(0, &format!("Erc721ContractVolWeek:{}", prev_week_id));

    for transfer in transfers.erc721transfers {
        store.add(
            0,
            format!("Erc721ContractVolWeek:{}:{}", week_id, transfer.address),
            BigInt::one(),
        )
    }
}

#[substreams::handlers::store]
pub fn store_erc721_transfer_daily(clock: Clock, transfers: MasterProto, store: StoreAddBigInt) {
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let day_id = timestamp_seconds / 86400;
    let prev_day_id = day_id - 1;

    store.delete_prefix(0, &format!("Erc721ContractVolDay:{}", prev_day_id));

    for transfer in transfers.erc721transfers {
        store.add(
            0,
            format!("Erc721ContractVolDay:{}:{}", day_id, transfer.address),
            BigInt::one(),
        )
    }
}

#[substreams::handlers::store]
pub fn store_erc721_transfer_monthly(clock: Clock, transfers: MasterProto, store: StoreAddBigInt) {
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let month_id = timestamp_seconds / 2592000;
    let prev_month_id = month_id - 1;

    store.delete_prefix(0, &format!("Erc721ContractVolMonth:{}", prev_month_id));

    for transfer in transfers.erc721transfers {
        store.add(
            0,
            format!("Erc721ContractVolMonth:{}:{}", month_id, transfer.address),
            BigInt::one(),
        )
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
