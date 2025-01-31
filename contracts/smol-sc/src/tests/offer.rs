use soroban_sdk::{testutils::Address as _, Address, Env};

use std::println;
extern crate std;

use crate::{
    tests::utils::{initialize, mint, Init},
    OfferBuy, OfferSellAsset, OfferSellAssetGet,
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
        &OfferSellAssetGet(None, fee_sac_address.clone(), amount),
        &glyph_1_hash,
    );

    assert_eq!(offer_2.is_none(), true);

    client.royalties_claim(&user_1, &fee_sac_address);

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
        &OfferSellAssetGet(None, fee_sac_address.clone(), amount),
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

    client.royalties_claim(&user_1, &fee_sac_address);

    let glyph_1_owner = client.glyph_owner_get(&glyph_1_hash);
    let user_1_balance = fee_sac_client.balance(&user_1);

    // ensure user 2 has their glyph
    assert_eq!(glyph_1_owner, user_2);
    // ensure user 1 has their money
    assert_eq!(user_1_balance, amount);

    let offer_1 = client.offer_sell_asset_get(
        &OfferSellAssetGet(None, fee_sac_address.clone(), amount),
        &glyph_1_hash,
    );

    // ensure no offers are open
    assert_eq!(offer_1.is_none(), true);
}

#[test]
fn test_multiple_offers_and_royalties() {
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
    let user_3 = Address::generate(&env);

    fee_sac_admin_client.mint(&user_2, &amount);
    fee_sac_admin_client.mint(&user_3, &(amount + mine_fee + mine_fee));

    client.color_claim(&user_3, &user_3, &0);
    client.color_claim(&user_3, &user_3, &16777215);

    let glyph_1_hash = mint(&env, &client, &contract_id, &user_1, &user_1);

    // Make 2 offers from user 2 and user 3 for the same glyph_1
    client.offer_sell_asset(
        &OfferSellAsset(user_2.clone(), fee_sac_address.clone(), amount),
        &glyph_1_hash,
    );
    client.offer_sell_asset(
        &OfferSellAsset(user_3.clone(), fee_sac_address.clone(), amount),
        &glyph_1_hash,
    );

    // ensure there are 2 offers
    let offer_1 = client.offer_sell_asset_get(
        &OfferSellAssetGet(None, fee_sac_address.clone(), amount),
        &glyph_1_hash,
    );

    assert_eq!(offer_1.unwrap(), 2);

    // match offer 1
    client.offer_sell_glyph(
        &glyph_1_hash,
        &OfferBuy::Asset(fee_sac_address.clone(), amount),
    );

    // ensure user 2 owns the glyph
    let glyph_1_owner = client.glyph_owner_get(&glyph_1_hash);

    assert_eq!(glyph_1_owner, user_2);

    // ensure there's only one offer left
    let offer_1 = client.offer_sell_asset_get(
        &OfferSellAssetGet(None, fee_sac_address.clone(), amount),
        &glyph_1_hash,
    );

    assert_eq!(offer_1.unwrap(), 1);

    // match offer 2
    client.offer_sell_glyph(
        &glyph_1_hash,
        &OfferBuy::Asset(fee_sac_address.clone(), amount),
    );

    // ensure user 3 owns the glyph
    let glyph_1_owner = client.glyph_owner_get(&glyph_1_hash);

    assert_eq!(glyph_1_owner, user_3);

    // ensure there are no offers left
    let offer_1 = client.offer_sell_asset_get(
        &OfferSellAssetGet(None, fee_sac_address.clone(), amount),
        &glyph_1_hash,
    );

    assert_eq!(offer_1.is_none(), true);

    client.royalties_claim(&user_1, &fee_sac_address);
    client.royalties_claim(&user_2, &fee_sac_address);
    client.royalties_claim(&user_3, &fee_sac_address);

    // ensure user 1 has their money
    let user_1_balance = fee_sac_client.balance(&user_1);

    assert_eq!(user_1_balance, amount + 5_0000000 - 2_0000000 + 1); // 100 XLM + 5% author royalty from user 3 - 2% color owner royalty + 1 for rounding invariant

    // ensure user 2 has their money
    let user_2_balance = fee_sac_client.balance(&user_2);

    assert_eq!(user_2_balance, amount - 5_0000000 - 2_0000000 + 1); // 100 XLM - 5% author royalty for user 1 - 2% color owner royalty + 1 for rounding invariant

    // ensure user 3 has their money
    let user_3_balance = fee_sac_client.balance(&user_3);

    assert_eq!(user_3_balance, 2_0000000 + 2_0000000 - 2); // 2 glyph sales at 2% for owning all of the colors (-2 for rounding invariant)

    // ensure contract has no money
    let contract_balance = fee_sac_client.balance(&contract_id);

    assert_eq!(contract_balance, 0);
}

#[test]
fn test_get_offers() {
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
        ..
    } = initialize(
        &env,
        mine_fee,
        color_owner_royalty_rate,
        glyph_author_royalty_rate,
    );

    let user_1 = Address::generate(&env);

    fee_sac_admin_client.mint(&user_1, &100);

    let glyph_1_hash = mint(&env, &client, &contract_id, &user_1, &user_1);
    let glyph_2_hash = mint(&env, &client, &contract_id, &user_1, &user_1);

    client.offer_sell_glyph(&glyph_1_hash, &OfferBuy::Asset(fee_sac_address.clone(), 0));

    client.offer_sell_glyph(&glyph_1_hash, &OfferBuy::Glyph(glyph_2_hash.clone()));

    client.offer_sell_asset(
        &OfferSellAsset(user_1.clone(), fee_sac_address.clone(), 100),
        &glyph_1_hash,
    );

    let offer_1 = client.offer_sell_glyph_get(&glyph_1_hash, &None);
    let offer_2 = client.offer_sell_glyph_get(
        &glyph_1_hash,
        &Some(OfferBuy::Asset(fee_sac_address.clone(), 0)),
    );

    let offer_3 = client.offer_sell_glyph_get(&glyph_1_hash, &Some(OfferBuy::Glyph(glyph_2_hash)));

    let offer_4 = client.offer_sell_asset_get(
        &OfferSellAssetGet(None, fee_sac_address.clone(), 100),
        &glyph_1_hash,
    );
    let offer_5 = client.offer_sell_asset_get(
        &OfferSellAssetGet(Some(user_1), fee_sac_address, 100),
        &glyph_1_hash,
    );

    assert_eq!(offer_1.unwrap(), 2); // 2 total buy my glyph offers
    assert_eq!(offer_2.unwrap(), 0); // index 0 on the buy my glyph with an asset offer
    assert_eq!(offer_3.unwrap(), 1); // index 1 on the by my glyph with this glyph offer
    assert_eq!(offer_4.unwrap(), 1); // 1 total buy this glyph with this asset offer
    assert_eq!(offer_5.unwrap(), 0); // 0 index on the buy this glyph with my asset offer
}

#[test]
fn test_remove_offers() {
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

    let user_1 = Address::generate(&env);

    fee_sac_admin_client.mint(&user_1, &100);

    let glyph_1_hash = mint(&env, &client, &contract_id, &user_1, &user_1);
    let glyph_2_hash = mint(&env, &client, &contract_id, &user_1, &user_1);

    client.offer_sell_glyph(&glyph_1_hash, &OfferBuy::Asset(fee_sac_address.clone(), 0));

    client.offer_sell_glyph(&glyph_1_hash, &OfferBuy::Glyph(glyph_2_hash.clone()));

    client.offer_sell_asset(
        &OfferSellAsset(user_1.clone(), fee_sac_address.clone(), 100),
        &glyph_1_hash,
    );

    // ensure user funds were withdrawn
    let user_1_balance = fee_sac_client.balance(&user_1);

    assert_eq!(user_1_balance, 0);

    client.offer_sell_glyph_remove(
        &glyph_1_hash,
        &Some(OfferBuy::Asset(fee_sac_address.clone(), 0)),
    );

    client.offer_sell_glyph_remove(&glyph_1_hash, &Some(OfferBuy::Glyph(glyph_2_hash.clone())));

    client.offer_sell_asset_remove(
        &OfferSellAsset(user_1.clone(), fee_sac_address.clone(), 100),
        &glyph_1_hash,
    );

    let offer_1 = client.offer_sell_glyph_get(&glyph_1_hash, &None);
    let offer_2 = client.offer_sell_asset_get(
        &OfferSellAssetGet(None, fee_sac_address.clone(), 100),
        &glyph_1_hash,
    );

    assert_eq!(offer_1.is_none(), true);
    assert_eq!(offer_2.is_none(), true);

    // ensure user's funds were returned
    let user_1_balance = fee_sac_client.balance(&user_1);

    assert_eq!(user_1_balance, 100);
}

#[test]
fn test_remove_all_buy_me_now_offers() {
    let env = Env::default();

    env.mock_all_auths();

    let mine_fee = 250_0000000;
    let color_owner_royalty_rate = 2;
    let glyph_author_royalty_rate = 5;

    let Init {
        contract_id,
        client,
        fee_sac_address,
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

    for index in 0..10 {
        client.offer_sell_glyph(
            &glyph_1_hash,
            &OfferBuy::Asset(fee_sac_address.clone(), index),
        );
    }

    let offers = client.offer_sell_glyph_get(&glyph_1_hash, &None);

    assert_eq!(offers.unwrap(), 10);

    client.offer_sell_asset(
        &OfferSellAsset(user_2, fee_sac_address.clone(), 0),
        &glyph_1_hash,
    );

    let offers = client.offer_sell_glyph_get(&glyph_1_hash, &None);

    assert_eq!(offers.is_none(), true);

    for index in 0..10 {
        client.offer_sell_glyph(
            &glyph_1_hash,
            &OfferBuy::Asset(fee_sac_address.clone(), index),
        );
    }

    client.offer_sell_glyph_remove(&glyph_1_hash, &None);

    let offers = client.offer_sell_glyph_get(&glyph_1_hash, &None);

    assert_eq!(offers.is_none(), true);
}
