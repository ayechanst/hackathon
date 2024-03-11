use crate::pb::debbie::{Erc20Deployment, Erc20Snapshot, Erc20Transfer};
use std::str::FromStr;
use substreams::scalar::BigInt;
use substreams_entity_change::tables::Tables;

pub fn deployments(tables: &mut Tables, erc20_deployments: Vec<Erc20Deployment>) {
    for erc20_deployment in erc20_deployments {
        tables
            .update_row("TokenDeployment", &erc20_deployment.address)
            .set(
                "blocknumber",
                BigInt::from_str(&erc20_deployment.blocknumber).unwrap_or(BigInt::from(0)),
            )
            .set("timestamp", erc20_deployment.timestamp_seconds)
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
}

pub fn transfers(tables: &mut Tables, erc20_transfers: Vec<Erc20Transfer>) {
    for (index, erc20_transfer) in erc20_transfers.iter().enumerate() {
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
            .set(
                "amount",
                BigInt::from_str(&erc20_transfer.amount).unwrap_or(BigInt::zero()),
            )
            .set(
                "count",
                BigInt::from_str(&erc20_transfer.count).unwrap_or(BigInt::zero()),
            )
            .set(
                "volume",
                BigInt::from_str(&erc20_transfer.volume).unwrap_or(BigInt::zero()),
            )
            .set("token", &erc20_transfer.address)
            .set(
                "blocknumber",
                BigInt::from_str(&erc20_transfer.blocknumber).unwrap_or(BigInt::zero()),
            )
            .set("timestamp", erc20_transfer.timestamp_seconds)
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
            .set(
                "volume",
                BigInt::from_str(&erc20_transfer.volume).unwrap_or(BigInt::zero()),
            )
            .set(
                "count",
                BigInt::from_str(&erc20_transfer.count).unwrap_or(BigInt::zero()),
            )
            .set("dayCount", erc20_transfer.day_count)
            .set(
                "dayVolume",
                BigInt::from_str(&erc20_transfer.day_volume).unwrap_or(BigInt::zero()),
            )
            .set("weekCount", erc20_transfer.week_count)
            .set(
                "weekVolume",
                BigInt::from_str(&erc20_transfer.week_volume).unwrap_or(BigInt::zero()),
            );
    }
}

pub fn daily_snapshots(tables: &mut Tables, daily_snapshots: Vec<Erc20Snapshot>) {
    for daily_token_snapshot in daily_snapshots {
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
}

pub fn weekly_snapshots(tables: &mut Tables, weekly_snapshots: Vec<Erc20Snapshot>) {
    for weekly_token_snapshot in weekly_snapshots {
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
    }
}
