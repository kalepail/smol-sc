use soroban_sdk::{testutils::Address as _, Address, Env};

use std::println;
extern crate std;

use crate::tests::utils::{initialize, Init};

#[test]
fn test_color_claim() {
    let env = Env::default();

    env.mock_all_auths();

    let mine_fee = 250_0000000;
    let glyph_fee = 1_0000000;
    let color_owner_royalty_rate = 2;
    let glyph_author_royalty_rate = 5;

    let Init {
        client,
        fee_sac_admin_client,
        ..
    } = initialize(
        &env,
        mine_fee,
        glyph_fee,
        color_owner_royalty_rate,
        glyph_author_royalty_rate,
    );

    let user = Address::generate(&env);

    fee_sac_admin_client.mint(&user, &mine_fee);

    client.color_claim(&user, &user, &0x0000FF);
}

#[test]
fn test_color_owner_transfer() {
    // TODO
}
