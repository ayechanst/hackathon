use substreams::pb::substreams::Clock;

use crate::pb::debbie::{
    Erc20Transfer, Erc20Transfer, Erc20Transfers, MasterProto, TokenHolder, TokenHolders,
};
use substreams::store::{StoreGet, StoreGetBigInt, StoreGetInt64};

#[substreams::handlers::map]
pub fn map_erc20_transfers(
    transfers: MasterProto,
    store_vol: StoreGetBigInt,
    store_count: StoreGetInt64,
) -> Result<Erc20Transfers, substreams::errors::Error> {
    let mut erc20_transfers: Vec<Erc20Transfer> = Vec::new();
    for mut transfer in transfers.erc20transfers {
        if let Some(volume_bigint) = store_vol.get_at(0, &transfer.address) {
            transfer.volume = volume_bigint.to_string();
        }
        if let Some(count) = store_count.get_at(0, &transfer.address) {
            transfer.count = count.to_string();
        }
        erc20_transfers.push(transfer);
    }
    Ok(Erc20Transfers {
        transfers: erc20_transfers,
    })
}

// #[substreams::handlers::map]
// pub fn map_transfers_and_holders(
//     transfers: MasterProto,
//     store_vol: StoreGetBigInt,
//     store_count: StoreGetInt64,
//     store_user_balance: StoreGetBigInt,
//     store_user_vol: StoreGetBigInt,
//     store_user_count: StoreGetInt64,
// ) -> Result<Erc20Trans

#[substreams::handlers::map]
pub fn map_erc20_token_holders(
    transfers: MasterProto,
) -> Result<TokenHolders, substreams::errors::Error> {
    let mut token_holders = Vec::new();
    for transfer in &transfers.erc20transfers {
        if transfer.to != "00000000000000000000".to_string() {
            token_holders.push(TokenHolder {
                holder_address: transfer.to.to_string(),
                token_address: transfer.address.to_string(),
                balance: String::new(),
                transfer_volume: String::new(),
                transfer_count: String::new(),
                transfer_amount: transfer.amount.to_string(),
                transfer_from: false,
            });
        }
        token_holders.push(TokenHolder {
            holder_address: transfer.from.to_string(),
            token_address: transfer.address.to_string(),
            balance: String::new(),
            transfer_volume: String::new(),
            transfer_count: String::new(),
            transfer_amount: transfer.amount.to_string(),
            transfer_from: true,
        });
    }
    Ok(TokenHolders {
        token_holders: token_holders,
    })
}

#[substreams::handlers::map]
pub fn map_user_erc20_data(
    clock: Clock,
    token_holders: TokenHolders,
    store_user_vol: StoreGetBigInt,
    store_user_balance: StoreGetBigInt,
    store_user_count: StoreGetInt64,
) -> TokenHolders {
    let mut token_holders = token_holders;
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let month_id = timestamp_seconds / 2592000;

    for mut holder in &mut token_holders.token_holders {
        if let Some(volume) = store_user_vol.get_at(
            0,
            &format!(
                "UserErc20VolMonth:{}:{}:{}",
                month_id, holder.holder_address, holder.token_address
            ),
        ) {
            holder.transfer_volume = volume.to_string();
        }
        if let Some(balance) = store_user_balance.get_at(
            0,
            &format!(
                "UserErc20BalanceMonth:{}:{}:{}",
                month_id, holder.holder_address, holder.token_address
            ),
        ) {
            holder.balance = balance.to_string();
        }
        if let Some(count) = store_user_count.get_at(
            0,
            &format!(
                "UserErc20CountMonth:{}:{}:{}",
                month_id, holder.holder_address, holder.token_address
            ),
        ) {
            holder.transfer_count = count.to_string();
        }
    }
    token_holders
}
