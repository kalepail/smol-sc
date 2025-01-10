use soroban_sdk::{testutils::Address as _, Address, Env};

use std::println;
extern crate std;

use crate::{
    tests::utils::{initialize, mint, Init},
    Error, OfferBuy, OfferSellAsset,
};

#[test]
fn test_offer_glyph_for_glyph() {
    let env = Env::default();

    env.mock_all_auths();

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

    let user_1 = Address::generate(&env);
    let user_2 = Address::generate(&env);

    let glyph_1_hash = mint(&env, &client, &contract_id, &user_1, &user_1);
    let glyph_2_hash = mint(&env, &client, &contract_id, &user_2, &user_2);

    client.offer_sell_glyph(&glyph_1_hash, &OfferBuy::Glyph(glyph_2_hash.clone()));

    let offer_1 = client.offer_sell_glyph_get(&glyph_1_hash, &None);

    // check that offer is stored correctly
    assert_eq!(offer_1.is_some(), true);

    // match offer
    client.offer_sell_glyph(&glyph_2_hash, &OfferBuy::Glyph(glyph_1_hash.clone()));

    let offer_2 = client.offer_sell_glyph_get(&glyph_2_hash, &None);

    assert_eq!(offer_2.is_none(), true);

    let glyph_1_owner = client.glyph_owner_get(&glyph_1_hash);
    let glyph_2_owner = client.glyph_owner_get(&glyph_2_hash);

    // ensure ownership swapped
    assert_eq!(glyph_1_owner, user_2);
    assert_eq!(glyph_2_owner, user_1);

    let offer_1 = client.offer_sell_glyph_get(&glyph_1_hash, &None);

    // ensure no offers are open
    assert_eq!(offer_1.is_none(), true);
}

#[test]
fn test_offer_glyph_for_asset() {
    let env = Env::default();

    env.mock_all_auths();

    let mine_fee = 250_0000000;
    let color_owner_royalty_rate = 2;
    let glyph_author_royalty_rate = 5;

    let Init {
        contract_id,
        client,
        fee_sac_address,
        fee_sac_admin_client,
        fee_sac_client,
        ..
    } = initialize(
        &env,
        mine_fee,
        color_owner_royalty_rate,
        glyph_author_royalty_rate,
    );

    let amount = 100_0000000;

    let user_1 = Address::generate(&env);
    let user_2 = Address::generate(&env);

    fee_sac_admin_client.mint(&user_2, &amount);

    let glyph_1_hash = mint(&env, &client, &contract_id, &user_1, &user_1);

    client.offer_sell_glyph(
        &glyph_1_hash,
        &OfferBuy::Asset(fee_sac_address.clone(), amount),
    );

    let offer_1 = client.offer_sell_glyph_get(&glyph_1_hash, &None);

    // check that offer is stored correctly
    assert_eq!(offer_1.is_some(), true);

    // match offer
    client.offer_sell_asset(
        &OfferSellAsset(user_2.clone(), fee_sac_address.clone(), amount),
        &glyph_1_hash,
    );

    let offer_2 = client.offer_sell_asset_get(
        &crate::OfferSellAssetGet(None, fee_sac_address, amount),
        &glyph_1_hash,
    );

    assert_eq!(offer_2.is_none(), true);

    let glyph_1_owner = client.glyph_owner_get(&glyph_1_hash);
    let user_1_balance = fee_sac_client.balance(&user_1);

    // ensure user 2 has their glyph
    assert_eq!(glyph_1_owner, user_2);
    // ensure user 1 has their money
    assert_eq!(user_1_balance, amount);

    let offer_1 = client.offer_sell_glyph_get(&glyph_1_hash, &None);

    // ensure no offers are open
    assert_eq!(offer_1.is_none(), true);
}

#[test]
fn test_offer_asset_for_glyph() {
    let env = Env::default();

    env.mock_all_auths();

    let mine_fee = 250_0000000;
    let color_owner_royalty_rate = 2;
    let glyph_author_royalty_rate = 5;

    let Init {
        contract_id,
        client,
        fee_sac_address,
        fee_sac_admin_client,
        fee_sac_client,
        ..
    } = initialize(
        &env,
        mine_fee,
        color_owner_royalty_rate,
        glyph_author_royalty_rate,
    );

    let amount = 100_0000000;

    let user_1 = Address::generate(&env);
    let user_2 = Address::generate(&env);

    fee_sac_admin_client.mint(&user_2, &amount);

    let glyph_1_hash = mint(&env, &client, &contract_id, &user_1, &user_1);

    client.offer_sell_asset(
        &OfferSellAsset(user_2.clone(), fee_sac_address.clone(), amount),
        &glyph_1_hash,
    );

    let offer_1 = client.offer_sell_asset_get(
        &crate::OfferSellAssetGet(None, fee_sac_address.clone(), amount),
        &glyph_1_hash,
    );

    // check that offer is stored correctly
    assert_eq!(offer_1.is_some(), true);

    // match offer
    client.offer_sell_glyph(
        &glyph_1_hash,
        &OfferBuy::Asset(fee_sac_address.clone(), amount),
    );

    let offer_2 = client.offer_sell_glyph_get(&glyph_1_hash, &None);

    assert_eq!(offer_2.is_none(), true);

    let glyph_1_owner = client.glyph_owner_get(&glyph_1_hash);
    let user_1_balance = fee_sac_client.balance(&user_1);

    // ensure user 2 has their glyph
    assert_eq!(glyph_1_owner, user_2);
    // ensure user 1 has their money
    assert_eq!(user_1_balance, amount);

    let offer_1 = client.offer_sell_asset_get(
        &crate::OfferSellAssetGet(None, fee_sac_address.clone(), amount),
        &glyph_1_hash,
    );

    // ensure no offers are open
    assert_eq!(offer_1.is_none(), true);
}

#[test]
fn test_royalties() {

    // test author royalty

    // test color owner royalty

    // ensure leftover amount is sent to the seller
}
