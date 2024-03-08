use crate::pb::debbie::{
    Erc20HoldersTransfers, Erc20Snapshot, Erc20Transfer, MasterProto, TokenHolder, TokenHolders,
};

use substreams::scalar::BigInt;

use substreams::pb::substreams::module::input::store;
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
    store_token_weekly_count: StoreGetInt64,
    store_token_daily_count: StoreGetInt64,
    store_token_weekly_vol: StoreGetBigInt,
    store_token_daily_vol: StoreGetBigInt,
) -> Result<Erc20HoldersTransfers, substreams::errors::Error> {
    let mut erc20_transfers: Vec<Erc20Transfer> = Vec::new();
    let mut token_holders: Vec<TokenHolder> = Vec::new();
    let mut token_daily_snapshots: Vec<Erc20Snapshot> = Vec::new();
    let mut token_weekly_snapshots: Vec<Erc20Snapshot> = Vec::new();

    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    // let month_id = timestamp_seconds / 2592000;
    let month_id = timestamp_seconds / 604800;
    let day_id = timestamp_seconds / 86400;

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

        let daily_volume = store_token_daily_vol
            .get_at(
                0,
                &format!("Erc20TokenVolDay:{}:{}", day_id, &transfer.address),
            )
            .unwrap_or(BigInt::zero());

        let weekly_volume = store_token_weekly_vol
            .get_at(
                0,
                &format!("Erc20TokenVolWeek:{}:{}", month_id, &transfer.address),
            )
            .unwrap_or(BigInt::zero())
            .to_string();

        let daily_count = store_token_daily_count
            .get_at(
                0,
                &format!("Erc20TokenCountDay:{}:{}", day_id, &transfer.address),
            )
            .unwrap_or(0);

        let weekly_count = store_token_weekly_count
            .get_at(
                0,
                &format!("Erc20TokenCountWeek:{}:{}", month_id, &transfer.address),
            )
            .unwrap_or(0);

        token_daily_snapshots.push(Erc20Snapshot {
            token: String::from(&transfer.address),
            volume: daily_volume.to_string(),
            tx_count: daily_count,
            timestamp_seconds,
            snapshot_id: day_id,
        });

        token_weekly_snapshots.push(Erc20Snapshot {
            token: String::from(&transfer.address),
            volume: weekly_volume.to_string(),
            tx_count: weekly_count,
            timestamp_seconds,
            snapshot_id: month_id,
        });

        erc20_transfers.push(Erc20Transfer {
            address: String::from(&transfer.address),
            from: String::from(&transfer.from),
            to: String::from(&transfer.to),
            amount: String::from(&transfer.amount),
            count,
            volume,
            blocknumber: String::from(&blocknumber),
            timestamp_seconds: transfer.timestamp_seconds,
        });

        if transfer.from != "0000000000000000000000000000000000000000" {
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
            // if transfer.to != "00000000000000000000".to_string() {
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

        if transfer.to != "0000000000000000000000000000000000000000" {
            // }
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
    }
    Ok(Erc20HoldersTransfers {
        erc20transfers: erc20_transfers,
        token_daily_snapshots,
        token_weekly_snapshots,
        token_holders,
    })
}
