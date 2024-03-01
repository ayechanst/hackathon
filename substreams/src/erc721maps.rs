use crate::pb::debbie::{Erc721Token, Erc721Tokens, MasterProto};
use crate::pb::debbie::{Erc721Transfer, Erc721Transfers, NftHolder, NftHolders};
use substreams::store::{StoreGet, StoreGetBigInt};

#[substreams::handlers::map]
pub fn map_erc721_transfers(
    transfers: MasterProto,
    store_vol: StoreGetBigInt,
) -> Result<Erc721Transfers, substreams::errors::Error> {
    let mut erc721_transfers: Vec<Erc721Transfer> = Vec::new();
    for mut transfer in transfers.erc721transfers {
        if let Some(volume_bigint) = store_vol.get_at(0, &transfer.address) {
            transfer.volume = volume_bigint.to_string();
        }
        erc721_transfers.push(transfer);
    }
    Ok(Erc721Transfers {
        transfers: erc721_transfers,
    })
}

#[substreams::handlers::map]
pub fn map_erc721_token_holders(
    transfers: Erc721Transfers,
) -> Result<NftHolders, substreams::errors::Error> {
    let mut erc721_holders: Vec<NftHolder> = Vec::new();
    for transfer in transfers.transfers {
        if transfer.to != "00000000000000000000".to_string() {
            erc721_holders.push(NftHolder {
                holder_address: transfer.to,
                token_address: transfer.address.clone(),
                token_balance: String::new(),
                transfer_from: false,
            });
        }
        erc721_holders.push(NftHolder {
            holder_address: transfer.from,
            token_address: transfer.address,
            token_balance: String::new(),
            transfer_from: true,
        })
    }
    Ok(NftHolders {
        erc721_token_holders: erc721_holders,
    })
}

#[substreams::handlers::map]
pub fn map_user_erc721_data(erc721_holders: NftHolders, store: StoreGetBigInt) -> NftHolders {
    let mut nft_holders = erc721_holders;
    for mut holder in &mut nft_holders.erc721_token_holders {
        if let Some(balance) = store.get_at(
            0,
            format!("{}:{}", &holder.holder_address, &holder.token_address),
        ) {
            holder.token_balance = balance.to_string();
        }
    }
    nft_holders
}

#[substreams::handlers::map]
pub fn map_erc721_token_vol(
    transfers: Erc721Transfers,
    store: StoreGetBigInt,
) -> Result<Erc721Tokens, substreams::errors::Error> {
    let mut erc721_tokens: Vec<Erc721Token> = Vec::new();
    for transfer in transfers.transfers {
        if let Some(volume) =
            store.get_at(0, &format!("{}:{}", transfer.address, transfer.token_id))
        {
            erc721_tokens.push(Erc721Token {
                token_address: transfer.address,
                token_id: transfer.token_id,
                transfer_volume: volume.to_string(),
            });
        }
    }
    Ok(Erc721Tokens {
        erc721_tokens: erc721_tokens,
    })
}
