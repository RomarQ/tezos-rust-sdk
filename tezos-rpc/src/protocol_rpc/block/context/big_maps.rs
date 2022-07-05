pub mod big_map;

use crate::models::block::BlockID;

fn path(chain_id: &String, block_id: &BlockID) -> String {
    format!("{}/big_maps", super::path(chain_id, block_id))
}
