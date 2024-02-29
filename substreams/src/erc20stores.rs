use std::str::FromStr;

use crate::pb::debbie::{Erc20Deployment, Erc20Transfer, MasterProto, TokenHolders};
use crate::pb::debbie::{Erc721Deployment, Erc721Transfer};
use substreams::scalar::BigInt;
use substreams::store::{StoreAdd, StoreAddBigInt, StoreAddInt64, StoreNew};
use substreams::Hex;

#[substreams::handlers::store]
pub fn store_erc20_transfer_vol(transfers: MasterProto, store: StoreAddBigInt) {
    for transfer in transfers.erc20transfers {
        //add check for these
        store.add(
            0,
            transfer.address,
            BigInt::from_str(&transfer.amount).unwrap(),
        )
    }
}

#[substreams::handlers::store]
pub fn store_erc20_count(transfers: MasterProto, store: StoreAddInt64) {
    for transfer in transfers.erc20transfers {
        store.add(0, transfer.address, 1)
    }
}

#[substreams::handlers::store]
pub fn store_user_erc20_vol(transfers: TokenHolders, store: StoreAddBigInt) {
    for transfer in transfers.token_holders {
        //add check for these
        store.add(
            0,
            format!("{}:{}", transfer.holder_address, transfer.token_address),
            BigInt::from_str(&transfer.transfer_amount).unwrap(),
        );
    }
}

#[substreams::handlers::store]
pub fn store_user_erc20_balance(transfer: TokenHolders, store: StoreAddBigInt) {
    for transfer in transfer.token_holders {
        //add if let some check??
        let amount: substreams::scalar::BigInt;
        if !transfer.transfer_from {
            amount = BigInt::from_str(&transfer.transfer_amount).unwrap();
        } else {
            amount = BigInt::from_str(&transfer.transfer_amount).unwrap() * -1;
        }

        store.add(
            0,
            format!("{}:{}", transfer.holder_address, transfer.token_address),
            amount,
        );
    }
}
