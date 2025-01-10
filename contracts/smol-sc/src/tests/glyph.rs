use soroban_sdk::{testutils::Address as _, Address, Env};

use std::println;
extern crate std;

use crate::tests::utils::{initialize, mint, Init};

#[test]
fn test_glyph_mint() {
    let env = Env::default();

    let mine_fee = 250_0000000;
    let color_owner_royalty_rate = 2;
    let glyph_author_royalty_rate = 5;

    let Init {
        contract_id,
        client,
        ..
    } = initialize(
        &env,
        mine_fee,
        color_owner_royalty_rate,
        glyph_author_royalty_rate,
    );

    let user = Address::generate(&env);

    let glyph_hash = mint(&env, &client, &contract_id, &user, &user);

    // InvocationResources {
    //     instructions: 170098,
    //     mem_bytes: 11792,
    //     read_entries: 1,
    //     write_entries: 2,
    //     read_bytes: 500,
    //     write_bytes: 2480,
    //     contract_events_size_bytes: 0,
    //     persistent_rent_ledger_bytes: 10155600,
    //     persistent_entry_rent_bumps: 2,
    //     temporary_rent_ledger_bytes: 0,
    //     temporary_entry_rent_bumps: 0,
    // }

    println!("{:#?}", env.cost_estimate().resources());

    println!("{:?}", glyph_hash);

    let glyph = client.glyph_get(&glyph_hash);

    println!("{:?}", glyph);
}

#[test]
fn test_glyph_owner_transfer() {
    // TODO
}
