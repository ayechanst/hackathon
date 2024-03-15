use crate::deployments;
use crate::get_token_uri;
use crate::helpers;
use crate::pb::debbie::Erc721Tokens;
use crate::pb::debbie::{
    Erc721Deployment, Erc721Deployments, Erc721Token, Erc721Transfer, Erc721TransfersHoldersTokens,
    MasterProto,
};
use substreams::pb::substreams::Clock;
use substreams::store::{StoreGet, StoreGetInt64, StoreGetProto};

#[substreams::handlers::map]
pub fn map_token_uri_test(
    transfers: MasterProto,
    store: StoreGetProto<Erc721Deployment>,
) -> Erc721Tokens {
    let mut tokens = Vec::new();
    for transfer in transfers.erc721transfers.iter() {
        if let Some(contract) = store.get_at(0, &transfer.address) {
            //substreams::log::info!("contract in store");
            let token_uri = match get_token_uri(&contract, &transfer.token_id) {
                Ok(token_uri) => token_uri,

                Err(e) => String::new(),
            };
            let token = Erc721Token {
                token_id: transfer.token_id.clone(),
                token_address: transfer.address.clone(),
                transfer_volume: String::new(),
                timestamp_seconds: 1,
                token_uri,
            };
            if token.token_uri != String::new() {
                tokens.push(token);
            }
        }
    }
    Erc721Tokens {
        erc721_tokens: tokens,
    }
}

#[substreams::handlers::map]
pub fn map_erc721_transfers_tokens(
    clock: Clock,
    transfers: MasterProto,
    store_vol: StoreGetInt64,
    store_token_vol: StoreGetInt64,
    store_monthly_vol: StoreGetInt64,
    store_weekly_vol: StoreGetInt64,
    store_daily_vol: StoreGetInt64,
    store_contracts: StoreGetProto<Erc721Deployment>,
) -> Result<Erc721TransfersHoldersTokens, substreams::errors::Error> {
    let mut erc721_transfers: Vec<Erc721Transfer> = Vec::new();
    let mut erc721_tokens: Vec<Erc721Token> = Vec::new();

    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let day_id = timestamp_seconds / 86400;
    let week_id = timestamp_seconds / 604800;
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
        let mut token_uri = String::new();
        if let Some(contract) = store_contracts.get_at(0, &transfer.address) {
            token_uri = match get_token_uri(&contract, &transfer.token_id) {
                Ok(uri) => uri,

                Err(e) => String::new(),
            };
        }
        if token_uri != String::new() {
            erc721_tokens.push(Erc721Token {
                token_address: String::from(&transfer.address),
                token_id: String::from(&transfer.token_id),
                transfer_volume: volume,
                timestamp_seconds: transfer.timestamp_seconds,
                token_uri,
            });
        }

        let volume;
        if let Some(volume_bigint) = store_vol.get_at(0, &transfer.address) {
            volume = volume_bigint.to_string();
        } else {
            volume = String::new();
        }

        let daily_volume = store_daily_vol
            .get_at(
                0,
                &format!("Erc721ContractVolDay:{}:{}", day_id, &transfer.address),
            )
            .unwrap_or(0);

        let weekly_volume = store_weekly_vol
            .get_at(
                0,
                &format!("Erc721ContractVolWeek:{}:{}", week_id, &transfer.address),
            )
            .unwrap_or(0);

        let monthly_volume = store_monthly_vol
            .get_at(
                0,
                &format!("Erc721ContractVolMonth:{}:{}", month_id, &transfer.address),
            )
            .unwrap_or(0);

        erc721_transfers.push(Erc721Transfer {
            address: transfer.address,
            from: transfer.from,
            to: transfer.to,
            token_id: transfer.token_id,
            volume: volume,
            week_volume: weekly_volume,
            day_volume: daily_volume,
            month_volume: monthly_volume,
            blocknumber: transfer.blocknumber,
            timestamp_seconds: transfer.timestamp_seconds,
        });
    }
    Ok(Erc721TransfersHoldersTokens {
        transfers: erc721_transfers,
        erc721_tokens: erc721_tokens,
    })
}
