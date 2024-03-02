use crate::pb::debbie::{Erc721Token, Erc721TransfersHoldersTokens, MasterProto};
use crate::pb::debbie::{Erc721Transfer, NftHolder};
use substreams::pb::substreams::Clock;
use substreams::store::{StoreGet, StoreGetBigInt};

#[substreams::handlers::map]
pub fn map_erc721_transfers_holders_tokens(
    clock: Clock,
    transfers: MasterProto,
    store_vol: StoreGetBigInt,
    store_user_balance: StoreGetBigInt,
    store_token_vol: StoreGetBigInt,
) -> Result<Erc721TransfersHoldersTokens, substreams::errors::Error> {
    let mut erc721_transfers: Vec<Erc721Transfer> = Vec::new();
    let mut erc721_holders: Vec<NftHolder> = Vec::new();
    let mut erc721_tokens: Vec<Erc721Token> = Vec::new();

    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let month_id = timestamp_seconds / 2592000;

    for transfer in transfers.erc721transfers {
        let balance;
        if let Some(balance_bigint) = store_user_balance.get_at(
            0,
            &format!(
                "UserErc721BalanceMonth:{}:{}:{}",
                month_id, &transfer.to, &transfer.address
            ),
        ) {
            balance = balance_bigint.to_string();
        } else {
            balance = String::new();
        }

        if transfer.to != "00000000000000000000".to_string() {
            erc721_holders.push(NftHolder {
                holder_address: String::from(&transfer.to),
                token_address: String::from(&transfer.address),
                token_balance: String::from(&balance),
                month_id: month_id,
                blocknumber: String::from(&transfer.blocknumber),
                timestamp_seconds: transfer.timestamp_seconds,
            });
        }
        if transfer.from != "00000000000000000000".to_string() {
            erc721_holders.push(NftHolder {
                holder_address: String::from(&transfer.from),
                token_address: String::from(&transfer.address),
                token_balance: balance,
                month_id: month_id,
                blocknumber: String::from(&transfer.blocknumber),
                timestamp_seconds: transfer.timestamp_seconds,
            });
        }
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
        erc721_transfers.push(Erc721Transfer {
            address: transfer.address,
            from: transfer.from,
            to: transfer.to,
            token_id: transfer.token_id,
            volume: volume,
            blocknumber: transfer.blocknumber,
            timestamp_seconds: transfer.timestamp_seconds,
        });
    }
    Ok(Erc721TransfersHoldersTokens {
        transfers: erc721_transfers,
        erc721_token_holders: erc721_holders,
        erc721_tokens: erc721_tokens,
    })
}

