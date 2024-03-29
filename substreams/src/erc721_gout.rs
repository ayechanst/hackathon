use crate::pb::debbie::{Erc721Deployment, Erc721Token, Erc721Transfer};
use std::str::FromStr;
use substreams::scalar::BigInt;
use substreams_entity_change::tables::Tables;

pub fn deployments(tables: &mut Tables, erc721_deployments: Vec<Erc721Deployment>) {
    for erc721_deployment in erc721_deployments {
        tables
            .update_row("NftDeployment", &erc721_deployment.address)
            .set(
                "blocknumber",
                BigInt::from_str(&erc721_deployment.blocknumber).unwrap_or(BigInt::from(0)),
            )
            .set("timestamp", erc721_deployment.timestamp_seconds)
            .set("nft", &erc721_deployment.address);

        tables
            .update_row("Nft", &erc721_deployment.address)
            .set("name", erc721_deployment.name)
            .set("symbol", erc721_deployment.symbol)
            .set("tokenUri", erc721_deployment.token_uri)
            .set("deploymentTimestamp", erc721_deployment.timestamp_seconds);
    }
}

pub fn transfers(tables: &mut Tables, erc721_transfers: Vec<Erc721Transfer>) {
    for (index, erc721_transfer) in erc721_transfers.iter().enumerate() {
        let volume = BigInt::from_str(&erc721_transfer.volume).unwrap_or(BigInt::zero());
        let blocknumber = BigInt::from_str(&erc721_transfer.blocknumber).unwrap_or(BigInt::zero());

        tables
            .update_row(
                "NftTransfer",
                &format!(
                    "{}:{}:{}:{}:{}:{}",
                    &erc721_transfer.from,
                    &erc721_transfer.to,
                    &erc721_transfer.token_id,
                    &erc721_transfer.volume,
                    &erc721_transfer.blocknumber,
                    &index
                ),
            )
            .set("from", &erc721_transfer.from)
            .set("to", &erc721_transfer.to)
            .set("tokenId", &erc721_transfer.token_id)
            .set("volume", &volume)
            .set("blocknumber", blocknumber)
            .set("timestamp", erc721_transfer.timestamp_seconds);

        tables
            .update_row("Nft", &erc721_transfer.address)
            .set("volume", volume)
            .set("dayVolume", erc721_transfer.day_volume)
            .set("weekVolume", erc721_transfer.week_volume)
            .set("monthVolume", erc721_transfer.month_volume);
    }
}

pub fn token_transfers(tables: &mut Tables, token_transfers: Vec<Erc721Token>) {
    for token in token_transfers {
        tables
            .create_row(
                "NftToken",
                format!("{}:{}", token.token_address, token.token_id),
            )
            .set("address", &token.token_address)
            .set("tokenId", &token.token_id)
            .set("tokenUri", &token.token_uri)
            .set(
                "volume",
                BigInt::from_str(&token.transfer_volume).unwrap_or(BigInt::one()),
            )
            .set("nft", &token.token_address)
            .set("timestamp", token.timestamp_seconds);
    }
}
