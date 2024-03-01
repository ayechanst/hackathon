use std::str::FromStr;

use crate::pb::debbie::{MasterProto, TokenHolders};
use substreams::scalar::BigInt;
use substreams::store::{StoreAdd, StoreAddBigInt, StoreAddInt64, StoreNew};

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
pub fn store_user_erc20_vol(transfers: TokenHolders, store: StoreAddBigInt) {
    for transfer in transfers.token_holders {
        if let Some(amount) = BigInt::from_str(&transfer.transfer_amount).ok() {
            store.add(
                0,
                format!("{}:{}", transfer.holder_address, transfer.token_address),
                amount,
            )
        } else {
            continue;
        }
    }
}

#[substreams::handlers::store]
pub fn store_user_erc20_balance(transfer: TokenHolders, store: StoreAddBigInt) {
    for transfer in transfer.token_holders {
        if let Some(amount) = BigInt::from_str(&transfer.transfer_amount).ok() {
            if transfer.transfer_from {
                store.add(
                    0,
                    format!("{}:{}", transfer.holder_address, transfer.token_address),
                    amount * -1,
                )
            } else {
                // subtract transfer amount from user balance
                store.add(
                    0,
                    format!("{}:{}", transfer.holder_address, transfer.token_address),
                    amount,
                )
            }
        } else {
            continue;
        }
    }
}

#[substreams::handlers::store]
pub fn store_user_erc20_count(transfer: TokenHolders, store: StoreAddInt64) {
    for transfer in transfer.token_holders {
        store.add(
            0,
            format!("{}:{}", transfer.holder_address, transfer.token_address),
            1,
        )
    }
}
