use crate::pb::debbie::Erc20HoldersTransfers;

use crate::pb::debbie::MasterProto;

use std::str::FromStr;
use substreams::pb::substreams::Clock;
use substreams::scalar::BigInt;

use substreams::store::{StoreGet, StoreGetBigInt, StoreGetInt64};
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;

pub use crate::erc20maps::*;
pub use crate::erc20stores::*;
pub use crate::erc721maps::*;
pub use crate::erc721stores::*;

#[substreams::handlers::map]
fn graph_out(
    clock: Clock,
    master: MasterProto,
    transfers_and_holders: Erc20HoldersTransfers,
    // erc721_transfers_holders_tokens: Erc721TransfersHoldersTokens,
) -> Result<EntityChanges, substreams::errors::Error> {
    let mut tables = Tables::new();
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let month_id = timestamp_seconds / 2592000;
    for erc20_deployment in master.erc20contracts {
        let total_supply: BigInt;
        if let Some(supply) = BigInt::from_str(&erc20_deployment.total_supply).ok() {
            total_supply = supply;
        } else {
            total_supply = BigInt::from(0);
        }

        tables
            .update_row("TokenDeployment", &erc20_deployment.address)
            .set(
                "blocknumber",
                BigInt::from_str(&erc20_deployment.blocknumber).unwrap_or(BigInt::from(0)),
            )
            .set("timestamp", BigInt::from(timestamp_seconds))
            .set("token", &erc20_deployment.address);

        tables
            .update_row("Token", &erc20_deployment.address)
            .set("name", erc20_deployment.name)
            .set("symbol", erc20_deployment.symbol)
            .set(
                "decimals",
                BigInt::from_str(&erc20_deployment.decimals).unwrap_or(BigInt::zero()),
            )
            .set(
                "totalSupply",
                BigInt::from_str(&erc20_deployment.total_supply).unwrap_or(BigInt::zero()),
            )
            .set("volume", BigInt::zero())
            .set("count", BigInt::zero());
    }

    for daily_token_snapshot in transfers_and_holders.token_daily_snapshots {
        tables
            .update_row(
                "TokenDailySnapshot",
                &format!(
                    "{}:{}",
                    &daily_token_snapshot.token, &daily_token_snapshot.snapshot_id
                ),
            )
            .set("tokenAddress", &daily_token_snapshot.token)
            .set("token", &daily_token_snapshot.token)
            .set(
                "volume",
                BigInt::from_str(&daily_token_snapshot.volume).unwrap_or(BigInt::zero()),
            )
            .set("count", daily_token_snapshot.tx_count)
            .set(
                "timestamp",
                BigInt::from(daily_token_snapshot.timestamp_seconds),
            );
    }

    for weekly_token_snapshot in transfers_and_holders.token_weekly_snapshots {
        tables
            .update_row(
                "TokenWeeklySnapshot",
                &format!(
                    "{}:{}",
                    &weekly_token_snapshot.token, &weekly_token_snapshot.snapshot_id
                ),
            )
            .set("tokenAddress", &weekly_token_snapshot.token)
            .set("token", &weekly_token_snapshot.token)
            .set(
                "volume",
                BigInt::from_str(&weekly_token_snapshot.volume).unwrap_or(BigInt::zero()),
            )
            .set("count", weekly_token_snapshot.tx_count)
            .set(
                "timestamp",
                BigInt::from(weekly_token_snapshot.timestamp_seconds),
            );
        tables
            .update_row(
                "TokenWeeklySnapshot",
                &format!(
                    "{}:{}",
                    &weekly_token_snapshot.token, weekly_token_snapshot.snapshot_id
                ),
            )
            .set("tokenAddress", &weekly_token_snapshot.token)
            .set("token", &weekly_token_snapshot.token)
            .set(
                "volume",
                BigInt::from_str(&weekly_token_snapshot.volume).unwrap_or(BigInt::zero()),
            )
            .set("count", weekly_token_snapshot.tx_count)
            .set(
                "timestamp",
                BigInt::from(weekly_token_snapshot.timestamp_seconds),
            );
    }

    // for erc721_deployment in master.erc721contracts {
    //     tables
    //         .create_row("NftDeployment", &erc721_deployment.address)
    //         .set(
    //             "blocknumber",
    //             BigInt::from_str(&erc721_deployment.blocknumber).unwrap_or(BigInt::from(0)),
    //         )
    //         .set(
    //             "timestamp",
    //             BigInt::from(erc721_deployment.timestamp_seconds),
    //         )
    //         .set("nft", &erc721_deployment.address);

    //     tables
    //         .update_row("Nft", &erc721_deployment.address)
    //         .set("name", erc721_deployment.name)
    //         .set("symbol", erc721_deployment.symbol)
    //         .set("tokenUri", erc721_deployment.token_uri)
    //         .set("volume", BigInt::zero());
    // }

    for (index, erc20_transfer) in transfers_and_holders.erc20transfers.iter().enumerate() {
        let amount: BigInt;
        if let Some(amt) = BigInt::from_str(&erc20_transfer.amount).ok() {
            amount = amt;
        } else {
            amount = BigInt::from(0);
        }

        let count: BigInt;
        if let Some(cnt) = BigInt::from_str(&erc20_transfer.count).ok() {
            count = cnt;
        } else {
            count = BigInt::from(0);
        }

        let volume: BigInt;
        if let Some(vol) = BigInt::from_str(&erc20_transfer.volume).ok() {
            volume = vol;
        } else {
            volume = BigInt::from(0);
        }

        let blocknumber: BigInt;
        if let Some(block) = BigInt::from_str(&erc20_transfer.blocknumber).ok() {
            blocknumber = block;
        } else {
            blocknumber = BigInt::from(0);
        }

        tables
            .create_row(
                "TokenTransfer",
                format!(
                    "{}:{}:{}:{}:{}:{}",
                    erc20_transfer.from,
                    erc20_transfer.to,
                    erc20_transfer.amount,
                    erc20_transfer.count,
                    erc20_transfer.blocknumber,
                    index
                ),
            )
            .set("from", &erc20_transfer.from)
            .set("to", &erc20_transfer.to)
            .set("amount", &amount)
            .set("count", &count)
            .set("volume", &volume)
            .set("token", &erc20_transfer.address)
            .set("blocknumber", &blocknumber)
            .set("timestamp", BigInt::from(timestamp_seconds))
            .set(
                "tokenHolderTo",
                &format!("{}:{}", &erc20_transfer.to, erc20_transfer.address),
            )
            .set(
                "tokenHolderFrom",
                &format!("{}:{}", &erc20_transfer.from, erc20_transfer.address),
            );

        tables
            .update_row("Token", &erc20_transfer.address)
            .set("volume", &volume)
            .set("count", &count);
    }

    // for (index, erc721_transfer) in erc721_transfers_holders_tokens.transfers.iter().enumerate() {
    //     let volume: BigInt;
    //     if let Some(vol) = BigInt::from_str(&erc721_transfer.volume).ok() {
    //         volume = vol;
    //     } else {
    //         volume = BigInt::from(0);
    //     }

    //     let blocknumber: BigInt;
    //     if let Some(block) = BigInt::from_str(&erc721_transfer.blocknumber).ok() {
    //         blocknumber = block;
    //     } else {
    //         blocknumber = BigInt::from(0);
    //     }

    //     tables
    //         .create_row(
    //             "NftTransfer",
    //             format!(
    //                 "{}:{}:{}:{}:{}:{}",
    //                 erc721_transfer.from,
    //                 erc721_transfer.to,
    //                 erc721_transfer.token_id,
    //                 erc721_transfer.volume,
    //                 erc721_transfer.blocknumber,
    //                 index
    //             ),
    //         )
    //         .set("from", erc721_transfer.from.clone())
    //         .set("to", erc721_transfer.to.clone())
    //         .set("tokenId", erc721_transfer.token_id.clone())
    //         .set("volume", &volume)
    //         .set("blocknumber", blocknumber)
    //         .set("timestamp", BigInt::from(timestamp_seconds))
    //         .set(
    //             "nftHolderTo",
    //             &format!("{}:{}", &erc721_transfer.to, &erc721_transfer.address),
    //         )
    //         .set(
    //             "nftHolderFrom",
    //             &format!("{}:{}", &erc721_transfer.from, &erc721_transfer.address),
    //         );

    //     tables
    //         .update_row("Nft", &erc721_transfer.address)
    //         .set("volume", BigInt::from(volume));
    // }

    for token_holder in transfers_and_holders.token_holders {
        let token_balance: BigInt;
        if let Some(balance) = BigInt::from_str(&token_holder.balance).ok() {
            token_balance = balance;
        } else {
            token_balance = BigInt::from(0);
        }

        let transfer_volume: BigInt;
        if let Some(volume) = BigInt::from_str(&token_holder.transfer_volume).ok() {
            transfer_volume = volume;
        } else {
            transfer_volume = BigInt::from(0);
        }

        let transfer_count: BigInt;
        if let Some(count) = BigInt::from_str(&token_holder.transfer_count).ok() {
            transfer_count = count;
        } else {
            transfer_count = BigInt::from(0);
        }

        tables
            .update_row(
                "TokenHolder",
                format!(
                    "{}:{}",
                    &token_holder.holder_address, &token_holder.token_address
                ),
            )
            .set("holderAddress", &token_holder.holder_address)
            .set("token", &token_holder.token_address)
            .set("timestamp", BigInt::from(timestamp_seconds));

        tables
            .update_row(
                "TokenHolderSnapshot",
                format!(
                    "{}:{}:{}",
                    &token_holder.holder_address, &token_holder.token_address, month_id
                ),
            )
            .set(
                "tokenHolder",
                &format!(
                    "{}:{}",
                    &token_holder.holder_address, &token_holder.token_address
                ),
            )
            .set("balance", token_balance)
            .set("transferVolume", transfer_volume)
            .set("transferCount", transfer_count)
            .set("timestamp", BigInt::from(token_holder.timestamp_seconds))
            .set(
                "blocknumber",
                BigInt::from_str(&token_holder.blocknumber).unwrap_or(BigInt::from(0)),
            )
            .set("monthId", BigInt::from(token_holder.month_id));
    }

    // for token_holder in erc721_transfers_holders_tokens.erc721_token_holders {
    //     tables
    //         .update_row(
    //             "NftHolder",
    //             format!(
    //                 "{}:{}",
    //                 token_holder.holder_address, token_holder.token_address
    //             ),
    //         )
    //         .set("holderAddress", &token_holder.holder_address)
    //         .set("nft", &token_holder.token_address)
    //         .set("timestamp", BigInt::from(token_holder.timestamp_seconds));

    //     tables
    //         .update_row(
    //             "NftHolderSnapshot",
    //             format!(
    //                 "{}:{}:{}",
    //                 &token_holder.holder_address, &token_holder.token_address, month_id
    //             ),
    //         )
    //         .set(
    //             "nftHolder",
    //             format!(
    //                 "{}:{}",
    //                 &token_holder.holder_address, &token_holder.token_address
    //             ),
    //         )
    //         .set(
    //             "tokenBalance",
    //             BigInt::from_str(&token_holder.token_balance).unwrap_or(BigInt::one()),
    //         )
    //         .set("timestamp", BigInt::from(token_holder.timestamp_seconds))
    //         .set(
    //             "blocknumber",
    //             BigInt::from_str(&token_holder.blocknumber).unwrap_or(BigInt::from(0)),
    //         )
    //         .set("monthId", BigInt::from(token_holder.month_id));
    // }

    // for token in erc721_transfers_holders_tokens.erc721_tokens {
    //     tables
    //         .create_row(
    //             "NftToken",
    //             format!("{}:{}", token.token_address, token.token_id),
    //         )
    //         .set("address", &token.token_address)
    //         .set("tokenId", &token.token_id)
    //         .set(
    //             "volume",
    //             BigInt::from_str(&token.transfer_volume).unwrap_or(BigInt::one()),
    //         )
    //         .set("nft", &token.token_address)
    //         .set("timestamp", BigInt::from(token.timestamp_seconds));
    // }

    Ok(tables.to_entity_changes())
}
