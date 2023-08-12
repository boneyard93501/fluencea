use marine_rs_sdk::{get_call_parameters, marine};

pub fn is_owner() -> bool {
    let params = get_call_parameters();
    params.init_peer_id == params.service_creator_peer_id
}

#[marine]
pub fn am_i_owner() -> bool {
    is_owner()
}
