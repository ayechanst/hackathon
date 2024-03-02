use crate::pb::debbie::{
    Erc20HoldersTransfers, Erc20Transfer, MasterProto, TokenHolder, TokenHolders,
};
use substreams::pb::substreams::Clock;
use substreams::store::{StoreGet, StoreGetBigInt, StoreGetInt64};

#[substreams::handlers::map]
pub fn map_transfers_and_holders(
    clock: Clock,
    transfers: MasterProto,
    store_vol: StoreGetBigInt,
    store_count: StoreGetInt64,
    store_user_vol: StoreGetBigInt,
    store_user_count: StoreGetInt64,
    store_user_balance: StoreGetBigInt,
) -> Result<Erc20HoldersTransfers, substreams::errors::Error> {
    let mut erc20_transfers: Vec<Erc20Transfer> = Vec::new();
    let mut token_holders: Vec<TokenHolder> = Vec::new();
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let month_id = timestamp_seconds / 2592000;

    for transfer in transfers.erc20transfers {
        let volume;
        if let Some(volume_bigint) = store_vol.get_at(0, &transfer.address) {
            volume = volume_bigint.to_string();
        } else {
            volume = String::new();
        }
        let count;
        if let Some(count_int) = store_count.get_at(0, &transfer.address) {
            count = count_int.to_string();
        } else {
            count = String::new();
        }

        let blocknumber = transfer.blocknumber;

        erc20_transfers.push(Erc20Transfer {
            address: String::from(&transfer.address),
            from: String::from(&transfer.from),
            to: String::from(&transfer.to),
            amount: String::from(&transfer.amount),
            count,
            volume,
            blocknumber: String::from(&blocknumber),
        });
        let user_balance;
        if let Some(user_balance_bigint) = store_user_balance.get_at(
            0,
            format!(
                "Erc20TokenBalanceMonth:{}:{}:{}",
                month_id, &transfer.from, &transfer.address
            ),
        ) {
            user_balance = user_balance_bigint.to_string();
        } else {
            user_balance = String::new();
        }
        let user_vol;
        if let Some(user_vol_bigint) = store_user_vol.get_at(
            0,
            format!(
                "Erc20TokenVolMonth:{}:{}:{}",
                month_id, &transfer.from, &transfer.address
            ),
        ) {
            user_vol = user_vol_bigint.to_string();
        } else {
            user_vol = String::new();
        }
        let user_count;
        if let Some(user_count_int) = store_user_count.get_at(
            0,
            format!(
                "UserErc20CountMonth:{}:{}:{}",
                month_id, &transfer.from, &transfer.address
            ),
        ) {
            user_count = user_count_int.to_string();
        } else {
            user_count = String::new();
        }
        if transfer.to != "00000000000000000000".to_string() {
            token_holders.push(TokenHolder {
                holder_address: String::from(&transfer.from),
                token_address: String::from(&transfer.address),
                balance: user_balance,
                transfer_volume: user_vol,
                transfer_count: user_count,
                transfer_amount: String::from(&transfer.amount),
                blocknumber: String::from(&blocknumber),
                month_id,
                timestamp_seconds,
            });
        }
        let user_balance;
        if let Some(user_balance_bigint) = store_user_balance.get_at(
            0,
            format!(
                "Erc20TokenBalanceMonth:{}:{}:{}",
                month_id, transfer.to, transfer.address
            ),
        ) {
            user_balance = user_balance_bigint.to_string();
        } else {
            user_balance = String::new();
        }
        let user_vol;
        if let Some(user_vol_bigint) = store_user_vol.get_at(
            0,
            format!(
                "Erc20TokenVolMonth:{}:{}:{}",
                month_id, transfer.to, transfer.address
            ),
        ) {
            user_vol = user_vol_bigint.to_string();
        } else {
            user_vol = String::new();
        }
        let user_count;
        if let Some(user_count_int) = store_user_count.get_at(
            0,
            format!(
                "UserErc20CountMonth:{}:{}:{}",
                month_id, transfer.to, transfer.address
            ),
        ) {
            user_count = user_count_int.to_string();
        } else {
            user_count = String::new();
        }
        token_holders.push(TokenHolder {
            holder_address: transfer.to,
            token_address: transfer.address,
            balance: user_balance,
            transfer_volume: user_vol,
            transfer_count: user_count,
            transfer_amount: transfer.amount,
            blocknumber: String::from(&blocknumber),
            month_id,
            timestamp_seconds,
        });
    }
    Ok(Erc20HoldersTransfers {
        erc20transfers: erc20_transfers,
        token_holders,
    })
}
