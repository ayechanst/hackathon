use crate::pb::debbie::{Erc20HoldersTransfers, Erc20Snapshot, Erc20Transfer, MasterProto};
use substreams::scalar::BigInt;

use substreams::pb::substreams::Clock;
use substreams::store::{StoreGet, StoreGetBigInt, StoreGetInt64};

#[substreams::handlers::map]
pub fn map_erc20_transfers(
    clock: Clock,
    transfers: MasterProto,
    store_vol: StoreGetBigInt,
    store_count: StoreGetInt64,
    store_token_monthly: StoreGetBigInt,
    store_token_weekly: StoreGetBigInt,
    store_token_daily: StoreGetBigInt,
) -> Result<Erc20HoldersTransfers, substreams::errors::Error> {
    let mut erc20_transfers: Vec<Erc20Transfer> = Vec::new();
    let mut token_daily_snapshots: Vec<Erc20Snapshot> = Vec::new();
    let mut token_weekly_snapshots: Vec<Erc20Snapshot> = Vec::new();

    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let month_id = timestamp_seconds / 2592000;
    let week_id = timestamp_seconds / 604800;
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

        let daily_volume = store_token_daily
            .get_at(
                0,
                &format!("Erc20TokenVolDay:{}:{}", day_id, &transfer.address),
            )
            .unwrap_or(BigInt::zero());

        let weekly_volume = store_token_weekly
            .get_at(
                0,
                &format!("Erc20TokenVolWeek:{}:{}", week_id, &transfer.address),
            )
            .unwrap_or(BigInt::zero())
            .to_string();

        let daily_count = store_token_daily
            .get_at(
                0,
                &format!("Erc20TokenCountDay:{}:{}", day_id, &transfer.address),
            )
            .unwrap_or(BigInt::zero());

        let weekly_count = store_token_weekly
            .get_at(
                0,
                &format!("Erc20TokenCountWeek:{}:{}", week_id, &transfer.address),
            )
            .unwrap_or(BigInt::zero());

        let monthly_count = store_token_monthly
            .get_at(
                0,
                &format!("Erc20TokenCountMonth:{}:{}", month_id, &transfer.address),
            )
            .unwrap_or(BigInt::zero());

        let monthly_vol = store_token_monthly
            .get_at(
                0,
                &format!("Erc20TokenVolMonth:{}:{}", month_id, &transfer.address),
            )
            .unwrap_or(BigInt::zero());

        token_daily_snapshots.push(Erc20Snapshot {
            token: String::from(&transfer.address),
            volume: daily_volume.to_string(),
            tx_count: daily_count.to_i32() as i64,
            timestamp_seconds,
            snapshot_id: day_id,
        });

        token_weekly_snapshots.push(Erc20Snapshot {
            token: String::from(&transfer.address),
            volume: weekly_volume.to_string(),
            tx_count: weekly_count.to_i32() as i64,
            timestamp_seconds,
            snapshot_id: week_id,
        });

        erc20_transfers.push(Erc20Transfer {
            address: String::from(&transfer.address),
            from: String::from(&transfer.from),
            to: String::from(&transfer.to),
            amount: String::from(&transfer.amount),
            count,
            volume,
            day_volume: daily_volume.to_string(),
            day_count: daily_count.to_i32() as i64,
            week_volume: weekly_volume.to_string(),
            week_count: weekly_count.to_i32() as i64,
            month_count: monthly_count.to_i32() as i64,
            month_volume: monthly_vol.to_string(),
            blocknumber: String::from(&blocknumber),
            timestamp_seconds: transfer.timestamp_seconds,
        });
    }
    Ok(Erc20HoldersTransfers {
        erc20transfers: erc20_transfers,
        token_daily_snapshots,
        token_weekly_snapshots,
    })
}
