use crate::pb::debbie::{Erc20HoldersTransfers, Erc721TransfersHoldersTokens};

use crate::pb::debbie::MasterProto;

use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;

pub use crate::erc20_gout;
pub use crate::erc20maps::*;
pub use crate::erc20stores::*;
pub use crate::erc721_gout;
pub use crate::erc721maps::*;
pub use crate::erc721stores::*;

#[substreams::handlers::map]
fn graph_out(
    master: MasterProto,
    transfers_and_holders: Erc20HoldersTransfers,
    erc721_transfers_holders_tokens: Erc721TransfersHoldersTokens,
) -> Result<EntityChanges, substreams::errors::Error> {
    let mut tables = Tables::new();

    erc20_gout::deployments(&mut tables, master.erc20contracts);
    erc20_gout::transfers(&mut tables, transfers_and_holders.erc20transfers);
    erc20_gout::daily_snapshots(&mut tables, transfers_and_holders.token_daily_snapshots);
    erc20_gout::weekly_snapshots(&mut tables, transfers_and_holders.token_weekly_snapshots);

    erc721_gout::deployments(&mut tables, master.erc721contracts);
    erc721_gout::transfers(&mut tables, erc721_transfers_holders_tokens.transfers);
    erc721_gout::token_transfers(&mut tables, erc721_transfers_holders_tokens.erc721_tokens);

    Ok(tables.to_entity_changes())
}
