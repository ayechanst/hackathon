use crate::pb::debbie::{Erc721Token, Erc721Tokens, Erc721TransfersHoldersTokens, MasterProto};
use crate::pb::debbie::{Erc721Transfer, Erc721Transfers, NftHolder, NftHolders};
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

// #[substreams::handlers::map]
// pub fn map_erc721_transfers(
//     transfers: MasterProto,
//     store_vol: StoreGetBigInt,
// ) -> Result<Erc721Transfers, substreams::errors::Error> {
//     let mut erc721_transfers: Vec<Erc721Transfer> = Vec::new();
//     for mut transfer in transfers.erc721transfers {
//         if let Some(volume_bigint) = store_vol.get_at(0, &transfer.address) {
//             transfer.volume = volume_bigint.to_string();
//         }
//         erc721_transfers.push(transfer);
//     }
//     Ok(Erc721Transfers {
//         transfers: erc721_transfers,
//     })
// }

// #[substreams::handlers::map]
// pub fn map_erc721_token_holders(
//     transfers: Erc721Transfers,
// ) -> Result<NftHolders, substreams::errors::Error> {
//     let mut erc721_holders: Vec<NftHolder> = Vec::new();
//     for transfer in transfers.transfers {
//         if transfer.to != "00000000000000000000".to_string() {
//             erc721_holders.push(NftHolder {
//                 holder_address: transfer.to,
//                 token_address: transfer.address.clone(),
//                 token_balance: String::new(),

//             });
//         }
//         erc721_holders.push(NftHolder {
//             holder_address: transfer.from,
//             token_address: transfer.address,
//             token_balance: String::new(),
//             transfer_from: true,
//         })
//     }
//     Ok(NftHolders {
//         erc721_token_holders: erc721_holders,
//     })
// }

// #[substreams::handlers::map]
// pub fn map_user_erc721_data(
//     clock: Clock,
//     erc721_holders: NftHolders,
//     store: StoreGetBigInt,
// ) -> NftHolders {
//     let mut nft_holders = erc721_holders;
//     let timestamp_seconds = clock.timestamp.unwrap().seconds;
//     let month_id = timestamp_seconds / 2592000;

//     for mut holder in &mut nft_holders.erc721_token_holders {
//         if let Some(balance) = store.get_at(
//             0,
//             format!(
//                 "UserErc721BalanceMonth:{}:{}:{}",
//                 month_id, &holder.holder_address, &holder.token_address
//             ),
//         ) {
//             holder.token_balance = balance.to_string();
//         }
//     }
//     nft_holders
// }

// #[substreams::handlers::map]
// pub fn map_erc721_token_vol(
//     clock: Clock,
//     transfers: Erc721Transfers,
//     store: StoreGetBigInt,
// ) -> Result<Erc721Tokens, substreams::errors::Error> {
//     let mut erc721_tokens: Vec<Erc721Token> = Vec::new();
//     let timestamp_seconds = clock.timestamp.unwrap().seconds;
//     let month_id = timestamp_seconds / 2592000;

//     for transfer in transfers.transfers {
//         if let Some(volume) = store.get_at(
//             0,
//             &format!(
//                 "Erc721TokenVolMonth:{}:{}:{}",
//                 month_id, transfer.address, transfer.token_id
//             ),
//         ) {
//             erc721_tokens.push(Erc721Token {
//                 token_address: transfer.address,
//                 token_id: transfer.token_id,
//                 transfer_volume: volume.to_string(),
//             });
//         }
//     }
//     Ok(Erc721Tokens {
//         erc721_tokens: erc721_tokens,
//     })
// }
