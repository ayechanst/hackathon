use crate::pb::debbie::MasterProto;
use std::str::FromStr;
use substreams::pb::substreams::Clock;
use substreams::scalar::BigInt;
use substreams::store::{StoreAdd, StoreAddBigInt, StoreAddInt64, StoreDelete, StoreNew};

#[substreams::handlers::store]
pub fn store_erc20_transfer_vol(transfers: MasterProto, store: StoreAddBigInt) {
    for transfer in transfers.erc20transfers {
        if let Some(amount) = BigInt::from_str(&transfer.amount).ok() {
            store.add(0, transfer.address, amount)
        } else {
            continue;
        }
    }
}

#[substreams::handlers::store]
pub fn store_erc20_transfer_count(transfers: MasterProto, store: StoreAddInt64) {
    for transfer in transfers.erc20transfers {
        store.add(0, transfer.address, 1)
    }
}

#[substreams::handlers::store]
pub fn store_erc20_transfer_count_weekly(
    clock: Clock,
    transfers: MasterProto,
    store: StoreAddInt64,
) {
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let week_id = timestamp_seconds / 604800;
    let prev_week_id = week_id - 1;

    store.delete_prefix(0, &format!("Erc20TokenCountWeek:{}", prev_week_id));

    for transfer in transfers.erc20transfers {
        store.add(
            0,
            &format!("Erc20TokenCountWeek:{}:{}", week_id, &transfer.address),
            1,
        )
    }
}

#[substreams::handlers::store]
pub fn store_erc20_transfer_vol_weekly(
    clock: Clock,
    transfers: MasterProto,
    store: StoreAddBigInt,
) {
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let week_id = timestamp_seconds / 604800;
    let prev_week_id = week_id - 1;

    store.delete_prefix(0, &format!("Erc20TokenVolWeek:{}", prev_week_id));

    for transfer in transfers.erc20transfers {
        if let Some(amount) = BigInt::from_str(&transfer.amount).ok() {
            store.add(
                0,
                &format!("Erc20TokenVolWeekly:{}:{}", week_id, &transfer.address),
                amount,
            )
        } else {
            continue;
        }
    }
}

#[substreams::handlers::store]
pub fn store_erc20_transfer_count_daily(
    clock: Clock,
    transfers: MasterProto,
    store: StoreAddInt64,
) {
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let day_id = timestamp_seconds / 86400;
    let prev_day_id = day_id - 1;

    store.delete_prefix(0, &format!("Erc20TokenCountDay:{}", prev_day_id));

    for transfer in transfers.erc20transfers {
        store.add(
            0,
            &format!("Erc20TokenCountDay:{}:{}", day_id, &transfer.address),
            1,
        )
    }
}

#[substreams::handlers::store]
pub fn store_erc20_transfer_vol_daily(clock: Clock, transfers: MasterProto, store: StoreAddBigInt) {
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let day_id = timestamp_seconds / 86400;
    let prev_day_id = day_id - 1;

    store.delete_prefix(0, &format!("Erc20TokenVolDay:{}", prev_day_id));

    for transfer in transfers.erc20transfers {
        if let Some(amount) = BigInt::from_str(&transfer.amount).ok() {
            store.add(
                0,
                &format!("Erc20TokenVolDay:{}:{}", day_id, &transfer.address),
                amount,
            )
        } else {
            continue;
        }
    }
}

#[substreams::handlers::store]
pub fn store_erc20_users_count(clock: Clock, transfer: MasterProto, store: StoreAddInt64) {
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    // let month_id = timestamp_seconds / 2592000;
    // let prev_month_id = month_id - 1;
    let week_id = timestamp_seconds / 604800;
    let prev_week_id = week_id - 1;

    store.delete_prefix(0, &format!("UserErc20CountMonth:{}", prev_week_id));

    for transfer in transfer.erc20transfers {
        if transfer.to != "0000000000000000000000000000000000000000".to_string() {
            store.add(
                0,
                format!(
                    "UserErc20CountMonth:{}:{}:{}",
                    week_id, transfer.to, transfer.address
                ),
                1,
            );
        }
        if transfer.from != "0000000000000000000000000000000000000000".to_string() {
            store.add(
                0,
                format!(
                    "UserErc20CountMonth:{}:{}:{}",
                    week_id, transfer.from, transfer.address
                ),
                1,
            );
        }
    }
}

#[substreams::handlers::store]
pub fn store_erc20_users_vol(clock: Clock, transfer: MasterProto, store: StoreAddBigInt) {
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    // let month_id = timestamp_seconds / 2592000;
    // let prev_month_id = month_id - 1;
    let week_id = timestamp_seconds / 604800;
    let prev_week_id = week_id - 1;

    store.delete_prefix(0, &format!("Erc20TokenVolMonth:{}", prev_week_id));

    for transfer in transfer.erc20transfers {
        if transfer.to != "0000000000000000000000000000000000000000".to_string() {
            store.add(
                0,
                format!(
                    "Erc20TokenVolMonth:{}:{}:{}",
                    week_id, transfer.to, transfer.address
                ),
                BigInt::from_str(&transfer.amount).unwrap_or(BigInt::zero()),
            );
        }
        if transfer.from != "0000000000000000000000000000000000000000".to_string() {
            store.add(
                0,
                format!(
                    "Erc20TokenVolMonth:{}:{}:{}",
                    week_id, transfer.from, transfer.address
                ),
                BigInt::from_str(&transfer.amount).unwrap_or(BigInt::zero()) * -1,
            );
        }
    }
}

#[substreams::handlers::store]
pub fn store_erc20_users_balance(clock: Clock, transfer: MasterProto, store: StoreAddBigInt) {
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    // let month_id = timestamp_seconds / 2592000;
    let week_id = timestamp_seconds / 604800;
    let prev_week_id = week_id - 1;
    // let prev_month_id = month_id - 1;

    store.delete_prefix(0, &format!("Erc20TokenBalanceMonth:{}", prev_week_id));

    for transfer in transfer.erc20transfers {
        if transfer.to != "0000000000000000000000000000000000000000".to_string() {
            store.add(
                0,
                format!(
                    "Erc20TokenBalanceMonth:{}:{}:{}",
                    week_id, transfer.to, transfer.address
                ),
                BigInt::from_str(&transfer.amount).unwrap_or(BigInt::zero()),
            );
        }
        if transfer.from != "0000000000000000000000000000000000000000".to_string() {
            store.add(
                0,
                format!(
                    "Erc20TokenBalanceMonth:{}:{}:{}",
                    week_id, transfer.from, transfer.address
                ),
                BigInt::from_str(&transfer.amount).unwrap_or(BigInt::zero()) * -1,
            );
        }
    }
}
