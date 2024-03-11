use crate::pb::debbie::{Erc721Token, Erc721Transfer, Erc721TransfersHoldersTokens, MasterProto};
use substreams::pb::substreams::Clock;
use substreams::scalar::BigInt;
use substreams::store::{StoreGet, StoreGetBigInt};

#[substreams::handlers::map]
pub fn map_erc721_transfers_tokens(
    clock: Clock,
    transfers: MasterProto,
    store_vol: StoreGetBigInt,
    store_token_vol: StoreGetBigInt,
    store_monthly_vol: StoreGetBigInt,
    store_weekly_vol: StoreGetBigInt,
    store_daily_vol: StoreGetBigInt,
) -> Result<Erc721TransfersHoldersTokens, substreams::errors::Error> {
    let mut erc721_transfers: Vec<Erc721Transfer> = Vec::new();
    let mut erc721_tokens: Vec<Erc721Token> = Vec::new();

    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let month_id = timestamp_seconds / 2592000;

    for transfer in transfers.erc721transfers {
        let volume;
        if let Some(volume_bigint) = store_token_vol.get_at(
            0,
            &format!(
                "Erc721TokenVolMonth:{}:{}:{}",
                month_id, transfer.address, transfer.token_id
            ),
        ) {
            volume = volume_bigint.to_string();
        } else {
            volume = String::new();
        }

        erc721_tokens.push(Erc721Token {
            token_address: String::from(&transfer.address),
            token_id: String::from(&transfer.token_id),
            transfer_volume: volume,
            timestamp_seconds: transfer.timestamp_seconds,
        });

        let volume;
        if let Some(volume_bigint) = store_vol.get_at(0, &transfer.address) {
            volume = volume_bigint.to_string();
        } else {
            volume = String::new();
        }

        let daily_volume = store_daily_vol
            .get_at(
                0,
                &format!(
                    "Erc721ContractVolDay:{}:{}",
                    timestamp_seconds / 86400,
                    &transfer.address
                ),
            )
            .unwrap_or(BigInt::zero());

        let weekly_volume = store_weekly_vol
            .get_at(
                0,
                &format!(
                    "Erc721ContractVolWeek:{}:{}",
                    timestamp_seconds / 604800,
                    &transfer.address
                ),
            )
            .unwrap_or(BigInt::zero());

        let monthly_volume = store_monthly_vol
            .get_at(
                0,
                &format!(
                    "Erc721ContractVolMonth:{}:{}",
                    timestamp_seconds / 2592000,
                    &transfer.address
                ),
            )
            .unwrap_or(BigInt::zero());

        erc721_transfers.push(Erc721Transfer {
            address: transfer.address,
            from: transfer.from,
            to: transfer.to,
            token_id: transfer.token_id,
            volume: volume,
            week_volume: weekly_volume.to_i32() as i64,
            day_volume: daily_volume.to_i32() as i64,
            month_volume: monthly_volume.to_i32() as i64,
            blocknumber: transfer.blocknumber,
            timestamp_seconds: transfer.timestamp_seconds,
        });
    }
    Ok(Erc721TransfersHoldersTokens {
        transfers: erc721_transfers,
        erc721_tokens: erc721_tokens,
    })
}
