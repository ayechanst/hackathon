use std::str::FromStr;

use crate::pb::debbie::{MasterProto, TokenHolders};
use substreams::pb::substreams::Clock;
use substreams::scalar::BigInt;
use substreams::store::{
    StoreAdd, StoreAddBigInt, StoreAddInt64, StoreDelete, StoreNew, StoreSet, StoreSetBigInt,
};

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
pub fn store_user_erc20_vol(clock: Clock, transfers: TokenHolders, store: StoreAddBigInt) {
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    // let week_id = timestamp_seconds / 604800;
    let month_id = timestamp_seconds / 2592000;
    let prev_month_id = month_id - 1;

    store.delete_prefix(0, &format!("UserErc20VolMonth:{}", prev_month_id));

    for transfer in transfers.token_holders {
        if let Some(amount) = BigInt::from_str(&transfer.transfer_amount).ok() {
            store.add(
                0,
                format!(
                    "UserErc20VolMonth:{}:{}:{}",
                    month_id, transfer.holder_address, transfer.token_address
                ),
                amount,
            )
        } else {
            continue;
        }
    }
}

#[substreams::handlers::store]
pub fn store_user_erc20_balance(clock: Clock, transfer: TokenHolders, store: StoreAddBigInt) {
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let month_id = timestamp_seconds / 2592000;
    let prev_month_id = month_id - 1;

    store.delete_prefix(0, &format!("UserErc20BalanceMonth:{}", prev_month_id));

    for transfer in transfer.token_holders {
        if let Some(amount) = BigInt::from_str(&transfer.transfer_amount).ok() {
            if transfer.transfer_from {
                store.add(
                    0,
                    format!(
                        "UserErc20BalanceMonth:{}:{}:{}",
                        month_id, transfer.holder_address, transfer.token_address
                    ),
                    amount * -1,
                )
            } else {
                // subtract transfer amount from user balance
                store.add(
                    0,
                    format!(
                        "UserErc20BalanceMonth:{}:{}:{}",
                        month_id, transfer.holder_address, transfer.token_address
                    ),
                    amount,
                )
            }
        } else {
            continue;
        }
    }
}

#[substreams::handlers::store]
pub fn store_user_erc20_count(clock: Clock, transfer: TokenHolders, store: StoreAddInt64) {
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let month_id = timestamp_seconds / 2592000;
    let prev_month_id = month_id - 1;

    store.delete_prefix(0, &format!("UserErc20CountMonth:{}", prev_month_id));
    for transfer in transfer.token_holders {
        store.add(
            0,
            format!(
                "UserErc20CountMonth:{}:{}:{}",
                month_id, transfer.holder_address, transfer.token_address
            ),
            1,
        )
    }
}
