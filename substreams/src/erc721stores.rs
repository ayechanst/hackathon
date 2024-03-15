use crate::deployments;
use crate::pb::debbie::{Erc721Deployment, MasterProto};
use substreams::pb::substreams::Clock;
use substreams::store::{StoreAdd, StoreAddInt64, StoreDelete, StoreNew, StoreSet, StoreSetProto};

#[substreams::handlers::store]
pub fn store_erc721_contract_data(contracts: MasterProto, store: StoreSetProto<Erc721Deployment>) {
    for contract in contracts.erc721contracts {
        store.set(0, &contract.address, &contract)
    }

}

#[substreams::handlers::store]
pub fn store_erc721_transfer_vol(transfers: MasterProto, store: StoreAddInt64) {
    for transfer in transfers.erc721transfers {
        store.add(0, transfer.address, 1)
    }
}

#[substreams::handlers::store]
pub fn store_erc721_transfer_weekly(clock: Clock, transfers: MasterProto, store: StoreAddInt64) {
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let week_id = timestamp_seconds / 604800;
    let prev_week_id = week_id - 1;

    store.delete_prefix(0, &format!("Erc721ContractVolWeek:{}", prev_week_id));

    for transfer in transfers.erc721transfers {
        store.add(
            0,
            format!("Erc721ContractVolWeek:{}:{}", week_id, transfer.address),
            1,
        )
    }
}

#[substreams::handlers::store]
pub fn store_erc721_transfer_daily(clock: Clock, transfers: MasterProto, store: StoreAddInt64) {
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let day_id = timestamp_seconds / 86400;
    let prev_day_id = day_id - 1;

    store.delete_prefix(0, &format!("Erc721ContractVolDay:{}", prev_day_id));

    for transfer in transfers.erc721transfers {
        store.add(
            0,
            format!("Erc721ContractVolDay:{}:{}", day_id, transfer.address),
            1,
        )
    }
}

#[substreams::handlers::store]
pub fn store_erc721_transfer_monthly(clock: Clock, transfers: MasterProto, store: StoreAddInt64) {
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let month_id = timestamp_seconds / 2592000;
    let prev_month_id = month_id - 1;

    store.delete_prefix(0, &format!("Erc721ContractVolMonth:{}", prev_month_id));

    for transfer in transfers.erc721transfers {
        store.add(
            0,
            format!("Erc721ContractVolMonth:{}:{}", month_id, transfer.address),
            1,
        )
    }
}

#[substreams::handlers::store]
pub fn store_erc721_token_vol(clock: Clock, transfers: MasterProto, store: StoreAddInt64) {
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let month_id = timestamp_seconds / 2592000;
    let prev_month_id = month_id - 1;

    store.delete_prefix(0, &format!("Erc721TokenVolMonth:{}", prev_month_id));

    for transfer in transfers.erc721transfers {
        store.add(
            0,
            format!(
                "Erc721TokenVolMonth:{}:{}:{}",
                month_id, transfer.address, transfer.token_id
            ),
            1,
        )
    }
}
