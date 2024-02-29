use crate::pb::debbie::{Erc20Transfer, Erc20Transfers, MasterProto, TokenHolder, TokenHolders};
use substreams::scalar::BigInt;
use substreams::store::{StoreGet, StoreGetBigInt, StoreGetInt64};

#[substreams::handlers::map]
pub fn map_erc20_transfer_vol(
    transfers: MasterProto,
    store: StoreGetBigInt,
) -> Result<Erc20Transfers, substreams::errors::Error> {
    let mut erc20_transfers = Vec::new();
    for mut transfer in transfers.erc20transfers {
        if let Some(volume_bigint) = store.get_at(0, &transfer.address) {
            transfer.volume = volume_bigint.to_string();
            erc20_transfers.push(transfer);
        }
    }
    Ok(Erc20Transfers {
        transfers: erc20_transfers,
    })
}

#[substreams::handlers::map]
pub fn map_erc20_count(
    transfers: MasterProto,
    store: StoreGetInt64,
) -> Result<Erc20Transfers, substreams::errors::Error> {
    let mut erc20_transfers = Vec::new();
    for mut transfer in transfers.erc20transfers {
        if let Some(count) = store.get_at(0, &transfer.address) {
            transfer.count = count.to_string();
        }
        erc20_transfers.push(transfer);
    }
    Ok(Erc20Transfers {
        transfers: erc20_transfers,
    })
}

#[substreams::handlers::map]
pub fn map_erc20_token_holders(
    transfers: MasterProto,
    store: StoreGetInt64,
) -> Result<TokenHolders, substreams::errors::Error> {
    let mut token_holders = Vec::new();
    for transfer in &transfers.erc20transfers {
        if transfer.to != "00000000000000000000".to_string() {
            token_holders.push(TokenHolder {
                holder_address: transfer.to.to_string(),
                token_address: transfer.address.to_string(),
                balance: String::new(),
                transfer_volume: String::new(),
                transfer_count: String::new(),
                transfer_amount: transfer.amount.to_string(),
                transfer_from: false,
            });
        }
        token_holders.push(TokenHolder {
            holder_address: transfer.from.to_string(),
            token_address: transfer.address.to_string(),
            balance: String::new(),
            transfer_volume: String::new(),
            transfer_count: String::new(),
            transfer_amount: transfer.amount.to_string(),
            transfer_from: true,
        });
    }
    Ok(TokenHolders {
        token_holders: token_holders,
    })
}
