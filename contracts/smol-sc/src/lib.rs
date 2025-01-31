#![no_std]

use soroban_fixed_point_math::SorobanFixedPoint;
use soroban_sdk::{
    auth::{Context, CustomAccountInterface},
    contract, contracterror, contractimpl, contracttype,
    crypto::Hash,
    token, vec, Address, Bytes, BytesN, Env, String, Symbol, Val, Vec,
};

#[contracterror]
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    ColorOutOfRange = 3,
    ColorAlreadyClaimed = 4,
    ColorNotClaimed = 5,
    GlyphTooBig = 6,
    GlyphAlreadyMinted = 7,
    GlyphNotMinted = 8,
    GlyphIndex = 9,
    OfferDuplicate = 10,
    OfferNotFound = 11,
    NoRoyaltiesToClaim = 12,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum Storage {
    Admin,
    FeeSAC,
    FeeAddress,
    ColorClaimFee,
    ColorOwnerRoyaltyRate,
    GlyphAuthorRoyaltyRate,
    GlyphIndex,
    ColorOwner(u32),                           // Color : Owner
    Glyph(u32),                         // Glyph Index : Glyph
    GlyphIndexHashMap(BytesN<32>), // Glyph Hash : Glyph Index
    GlyphOwner(u32),                    // Glyph Index : Owner
    OfferSellGlyph(u32),                // Glyph Index : Vec<OfferBuy>
    OfferSellAsset(u32, Address, i128), // Glyph Index, SAC, Amount : Vec<Owner>
    Royalties(Address, Address),               // Owner, SAC : Amount
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum OfferBuy {
    Asset(Address, i128), // SAC, Amount
    Glyph(u32),    // Glyph Index
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct OfferSellAsset(Address, Address, i128); // Owner, SAC, Amount

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct OfferSellAssetGet(Option<Address>, Address, i128); // Owner, SAC, Amount

#[contract]
pub struct Contract;

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Glyph {
    pub author: Address,
    pub colors: Bytes,    // u8 colors (up to 256 unique colors)
    pub legend: Vec<u32>, // map of u8 color to u32 color
    pub width: u32,
}

mod tests;

// TODO
// add ttl extensions?
// break up contract into separate files and helpers

#[contractimpl]
impl Contract {
    pub fn __constructor(
        env: Env,
        admin: Address,
        fee_sac: Address,
        fee_address: Address,
        color_claim_fee: i128,
        color_owner_royalty_rate: i128,
        glyph_author_royalty_rate: i128,
    ) -> Result<(), Error> {
        if env.storage().instance().has(&Storage::Admin) {
            return Err(Error::AlreadyInitialized);
        }

        env.storage()
            .instance()
            .set::<Storage, Address>(&Storage::Admin, &admin);
        env.storage()
            .instance()
            .set::<Storage, Address>(&Storage::FeeSAC, &fee_sac);
        env.storage()
            .instance()
            .set::<Storage, Address>(&Storage::FeeAddress, &fee_address);
        env.storage()
            .instance()
            .set::<Storage, i128>(&Storage::ColorClaimFee, &color_claim_fee);
        env.storage()
            .instance()
            .set::<Storage, i128>(&Storage::ColorOwnerRoyaltyRate, &color_owner_royalty_rate);
        env.storage()
            .instance()
            .set::<Storage, i128>(&Storage::GlyphAuthorRoyaltyRate, &glyph_author_royalty_rate);

        Ok(())
    }
    pub fn update(
        env: Env,
        admin: Option<Address>,
        fee_sac: Option<Address>,
        fee_address: Option<Address>,
        color_claim_fee: Option<i128>,
        color_owner_royalty_rate: Option<i128>,
        glyph_author_royalty_rate: Option<i128>,
    ) -> Result<(), Error> {
        let current_admin = env
            .storage()
            .instance()
            .get::<Storage, Address>(&Storage::Admin)
            .ok_or(Error::NotInitialized)?;

        current_admin.require_auth();

        if let Some(admin) = admin {
            env.storage()
                .instance()
                .set::<Storage, Address>(&Storage::Admin, &admin);
        }
        if let Some(fee_sac) = fee_sac {
            env.storage()
                .instance()
                .set::<Storage, Address>(&Storage::FeeSAC, &fee_sac);
        }
        if let Some(fee_address) = fee_address {
            env.storage()
                .instance()
                .set::<Storage, Address>(&Storage::FeeAddress, &fee_address);
        }
        if let Some(color_claim_fee) = color_claim_fee {
            env.storage()
                .instance()
                .set::<Storage, i128>(&Storage::ColorClaimFee, &color_claim_fee);
        }
        if let Some(color_owner_royalty_rate) = color_owner_royalty_rate {
            env.storage()
                .instance()
                .set::<Storage, i128>(&Storage::ColorOwnerRoyaltyRate, &color_owner_royalty_rate);
        }
        if let Some(glyph_author_royalty_rate) = glyph_author_royalty_rate {
            env.storage()
                .instance()
                .set::<Storage, i128>(&Storage::GlyphAuthorRoyaltyRate, &glyph_author_royalty_rate);
        }

        Ok(())
    }
    pub fn upgrade(env: Env, hash: BytesN<32>) -> Result<(), Error> {
        let admin = env
            .storage()
            .instance()
            .get::<Storage, Address>(&Storage::Admin)
            .ok_or(Error::NotInitialized)?;

        admin.require_auth();

        env.deployer().update_current_contract_wasm(hash);

        Ok(())
    }

    pub fn color_claim(env: Env, source: Address, owner: Address, color: u32) -> Result<(), Error> {
        let color = if color > 0xFFFFFF {
            return Err(Error::ColorOutOfRange);
        } else {
            color
        };
        let color_owner_key = Storage::ColorOwner(color);

        if env.storage().persistent().has(&color_owner_key) {
            return Err(Error::ColorAlreadyClaimed);
        }

        env.storage().persistent().set(&color_owner_key, &owner);

        let fee_address = env
            .storage()
            .instance()
            .get::<Storage, Address>(&Storage::FeeAddress)
            .ok_or(Error::NotInitialized)?;
        let fee_amount = env
            .storage()
            .instance()
            .get::<Storage, i128>(&Storage::ColorClaimFee)
            .ok_or(Error::NotInitialized)?;
        let fee_sac = env
            .storage()
            .instance()
            .get::<Storage, Address>(&Storage::FeeSAC)
            .ok_or(Error::NotInitialized)?;
        let fee_client = token::TokenClient::new(&env, &fee_sac);

        source.require_auth();

        fee_client.transfer(&source, &fee_address, &fee_amount);

        env.events()
            .publish((Symbol::new(&env, "color_claim"), owner), color);

        Ok(())
    }
    pub fn color_owner_get(env: Env, color: u32) -> Result<Address, Error> {
        env.storage()
            .persistent()
            .get::<Storage, Address>(&Storage::ColorOwner(color))
            .ok_or(Error::ColorNotClaimed)
    }
    pub fn color_owner_transfer(env: Env, color: u32, to: Address) -> Result<(), Error> {
        let color_owner_key = Storage::ColorOwner(color);

        let color_owner = env
            .storage()
            .persistent()
            .get::<Storage, Address>(&color_owner_key)
            .ok_or(Error::ColorNotClaimed)?;

        color_owner.require_auth();

        env.storage().persistent().set(&color_owner_key, &to);

        env.events()
            .publish((Symbol::new(&env, "color_owner_transfer"), to), color);

        Ok(())
    }

    pub fn glyph_mint(
        env: Env,
        author: Address,
        owner: Address,
        colors: Bytes,
        legend: Vec<u32>,
        width: u32,
        title: String,
        story: String,
    ) -> Result<u32, Error> {
        if colors.len() > 45 * 45 {
            return Err(Error::GlyphTooBig);
        }

        let mut colors_extended_with_width = Bytes::from(colors.clone());

        colors_extended_with_width.extend_from_slice(&width.to_be_bytes());

        let glyph_hash = env.crypto().sha256(&colors_extended_with_width).to_bytes();
        let glyph_index_key = Storage::GlyphIndexHashMap(glyph_hash.clone());

        if env.storage().persistent().has::<Storage>(&glyph_index_key) {
            return Err(Error::GlyphAlreadyMinted);
        }

        let glyph = Glyph {
            author,
            colors,
            legend,
            width,
        };

        let glyph_index = env.storage().instance().get::<Storage, u32>(&Storage::GlyphIndex).unwrap_or(0) + 1;

        env.storage()
            .instance()
            .set::<Storage, u32>(&Storage::GlyphIndex, &glyph_index);

        env.storage().persistent().set::<Storage, BytesN<32>>(&glyph_index_key, &glyph_hash);

        env.storage()
            .persistent()
            .set::<Storage, Glyph>(&Storage::Glyph(glyph_index), &glyph);

        env.storage()
            .persistent()
            .set::<Storage, Address>(&Storage::GlyphOwner(glyph_index), &owner);

        env.events().publish(
            (Symbol::new(&env, "glyph_mint"), owner),
            (glyph_index, title, story),
        );

        Ok(glyph_index)
    }
    pub fn glyph_get(env: Env, glyph_index: u32) -> Result<Glyph, Error> {
        env.storage()
            .persistent()
            .get::<Storage, Glyph>(&Storage::Glyph(glyph_index))
            .ok_or(Error::GlyphNotMinted)
    }
    pub fn glyph_owner_get(env: Env, glyph_index: u32) -> Result<Address, Error> {
        env.storage()
            .persistent()
            .get::<Storage, Address>(&Storage::GlyphOwner(glyph_index))
            .ok_or(Error::GlyphNotMinted)
    }
    pub fn glyph_owner_transfer(
        env: Env,
        glyph_index: u32,
        to: Address,
    ) -> Result<(), Error> {
        let glyph_owner_key = Storage::GlyphOwner(glyph_index);

        let glyph_owner = env
            .storage()
            .persistent()
            .get::<Storage, Address>(&glyph_owner_key)
            .ok_or(Error::GlyphNotMinted)?;

        glyph_owner.require_auth();

        env.storage().persistent().set(&glyph_owner_key, &to);

        env.events()
            .publish((Symbol::new(&env, "glyph_owner_transfer"), to), glyph_index);

        Ok(())
    }

    pub fn offer_sell_glyph(
        env: Env,
        sell: u32,
        buy: OfferBuy,
    ) -> Result<Option<Address>, Error> {
        let glyph_owner_key = Storage::GlyphOwner(sell);
        let offer_sell_glyph_key = Storage::OfferSellGlyph(sell);

        let glyph_owner = env
            .storage()
            .persistent()
            .get::<Storage, Address>(&glyph_owner_key)
            .ok_or(Error::GlyphNotMinted)?;

        glyph_owner.require_auth();

        match &buy {
            OfferBuy::Glyph(buy) => {
                let offer_buy_glyph_key = Storage::OfferSellGlyph(*buy);

                let offers = env
                    .storage()
                    .persistent()
                    .get::<Storage, Vec<OfferBuy>>(&offer_buy_glyph_key)
                    .unwrap_or(Vec::new(&env));

                match offers.binary_search(OfferBuy::Glyph(sell)) {
                    // Found a matching offer
                    Ok(_index) => {
                        let buy_glyph_owner_key = Storage::GlyphOwner(*buy);
                        let buy_glyph_owner = env
                            .storage()
                            .persistent()
                            .get::<Storage, Address>(&buy_glyph_owner_key)
                            .ok_or(Error::GlyphNotMinted)?;

                        // swap glyph ownership
                        env.storage()
                            .persistent()
                            .set::<Storage, Address>(&glyph_owner_key, &buy_glyph_owner);
                        env.storage()
                            .persistent()
                            .set::<Storage, Address>(&buy_glyph_owner_key, &glyph_owner);

                        // remove all open buy glyph sell offers
                        env.storage()
                            .persistent()
                            .remove::<Storage>(&Storage::OfferSellGlyph(*buy));

                        // remove all open sell glyph sell offers
                        env.storage()
                            .persistent()
                            .remove::<Storage>(&offer_sell_glyph_key);

                        // delete the offer
                        env.storage().persistent().remove(&offer_buy_glyph_key);

                        env.events()
                            .publish((Symbol::new(&env, "offer_sell_glyph"), sell, buy.clone()), Some(&buy_glyph_owner));

                        return Ok(Some(buy_glyph_owner));
                    }
                    // No matching offer found
                    Err(_index) => {}
                }
            }
            OfferBuy::Asset(buy, amount) => {
                let offer_sell_asset_key =
                    Storage::OfferSellAsset(sell.clone(), buy.clone(), *amount);

                let mut offers = env
                    .storage()
                    .persistent()
                    .get::<Storage, Vec<Address>>(&offer_sell_asset_key)
                    .unwrap_or(Vec::new(&env));

                match offers.get(0) {
                    // Found a matching offer
                    Some(owner) => {
                        let Glyph {
                            author,
                            colors,
                            legend,
                            ..
                        } = env
                            .storage()
                            .persistent()
                            .get::<Storage, Glyph>(&Storage::Glyph(sell.clone()))
                            .ok_or(Error::GlyphNotMinted)?;

                        // transfer to glyph author
                        let glyph_author_royalty_rate = env
                            .storage()
                            .instance()
                            .get::<Storage, i128>(&Storage::GlyphAuthorRoyaltyRate)
                            .ok_or(Error::NotInitialized)?;
                        let author_amount = glyph_author_royalty_rate
                            .fixed_mul_floor(&env, &amount, &100)
                            .max(1);

                        update_royalties(&env, &author, &buy, &author_amount);

                        // transfer to color owners
                        let colors_length = colors.len() as i128;
                        let color_owner_royalty_rate = env
                            .storage()
                            .instance()
                            .get::<Storage, i128>(&Storage::ColorOwnerRoyaltyRate)
                            .ok_or(Error::NotInitialized)?;

                        let legend_length = legend.len() as usize;
                        let mut color_owner_amounts = 0;

                        // TODO likely need to limit this to the first N ordered by highest count (125 storage gets)
                        for (index, count) in get_palette(colors).into_iter().enumerate() {
                            if index >= legend_length {
                                break;
                            }

                            let color = legend.get_unchecked(index as u32);

                            match env
                                .storage()
                                .persistent()
                                .get::<Storage, Address>(&Storage::ColorOwner(color))
                            {
                                Some(color_owner) => {
                                    let color_owner_amount = color_owner_royalty_rate
                                        .fixed_mul_floor(&env, &amount, &100)
                                        .fixed_mul_floor(&env, &(count as i128), &colors_length)
                                        .max(1);

                                    update_royalties(&env, &color_owner, &buy, &color_owner_amount);

                                    color_owner_amounts += color_owner_amount;
                                }
                                None => continue,
                            }
                        }

                        // transfer asset to buy glyph owner
                        update_royalties(
                            &env,
                            &glyph_owner,
                            &buy,
                            &(amount - author_amount - color_owner_amounts),
                        );

                        // swap glyph ownership
                        env.storage()
                            .persistent()
                            .set::<Storage, Address>(&glyph_owner_key, &owner);

                        // remove all open buy glyph sell offers
                        env.storage()
                            .persistent()
                            .remove::<Storage>(&offer_sell_glyph_key);

                        // remove and update offers
                        offers.remove(0);

                        env.storage()
                            .persistent()
                            .set::<Storage, Vec<Address>>(&offer_sell_asset_key, &offers);

                        env.events()
                            .publish((Symbol::new(&env, "offer_sell_glyph"), sell, buy, *amount), Some(&owner));

                        return Ok(Some(owner));
                    }
                    // No matching offer found
                    None => {}
                }
            }
        }

        let mut offers = env
            .storage()
            .persistent()
            .get::<Storage, Vec<OfferBuy>>(&offer_sell_glyph_key)
            .unwrap_or(Vec::new(&env));

        match offers.binary_search(buy.clone()) {
            Ok(_index) => {
                return Err(Error::OfferDuplicate);
            }
            Err(index) => offers.insert(index, buy.clone()),
        }

        env.storage()
            .persistent()
            .set::<Storage, Vec<OfferBuy>>(&offer_sell_glyph_key, &offers);

        env.events().publish(
            (Symbol::new(&env, "offer_sell_glyph"), sell, buy),
            None::<()>,
        );

        Ok(None)
    }
    pub fn offer_sell_asset(
        env: Env,
        sell: OfferSellAsset,
        buy: u32,
    ) -> Result<Option<()>, Error> {
        let OfferSellAsset(owner, sell, amount) = sell;

        owner.require_auth();

        let open_glyph_buy_now_offers_key = Storage::OfferSellGlyph(buy);
        let open_glyph_buy_now_offers = env
            .storage()
            .persistent()
            .get::<Storage, Vec<OfferBuy>>(&open_glyph_buy_now_offers_key)
            .unwrap_or(Vec::new(&env));

        match open_glyph_buy_now_offers.binary_search(OfferBuy::Asset(sell.clone(), amount)) {
            // Found a matching open counter offer. Take it
            Ok(_index) => {
                let buy_glyph_owner_key = Storage::GlyphOwner(buy);
                let buy_glyph_owner = env
                    .storage()
                    .persistent()
                    .get::<Storage, Address>(&buy_glyph_owner_key)
                    .ok_or(Error::GlyphNotMinted)?;

                // Send the amount to the contract for passive claiming later
                let token_client = token::TokenClient::new(&env, &sell);

                token_client.transfer(&owner, &env.current_contract_address(), &amount);

                let Glyph {
                    author,
                    colors,
                    legend,
                    ..
                } = env
                    .storage()
                    .persistent()
                    .get::<Storage, Glyph>(&Storage::Glyph(buy))
                    .ok_or(Error::GlyphNotMinted)?;

                // transfer to glyph author
                let glyph_author_royalty_rate = env
                    .storage()
                    .instance()
                    .get::<Storage, i128>(&Storage::GlyphAuthorRoyaltyRate)
                    .ok_or(Error::NotInitialized)?;
                let author_amount = glyph_author_royalty_rate
                    .fixed_mul_floor(&env, &amount, &100)
                    .max(1);

                update_royalties(&env, &author, &sell, &author_amount);

                // transfer to color owners
                let colors_length = colors.len() as i128;
                let color_owner_royalty_rate = env
                    .storage()
                    .instance()
                    .get::<Storage, i128>(&Storage::ColorOwnerRoyaltyRate)
                    .ok_or(Error::NotInitialized)?;

                let legend_length = legend.len() as usize;
                let mut color_owner_amounts = 0;

                // TODO likely need to limit this to the first N ordered by highest count (125 storage gets)
                for (index, count) in get_palette(colors).into_iter().enumerate() {
                    if index >= legend_length {
                        break;
                    }

                    let color = legend.get_unchecked(index as u32);

                    match env
                        .storage()
                        .persistent()
                        .get::<Storage, Address>(&Storage::ColorOwner(color))
                    {
                        Some(color_owner) => {
                            let color_owner_amount = color_owner_royalty_rate
                                .fixed_mul_floor(&env, &amount, &100)
                                .fixed_mul_floor(&env, &(count as i128), &colors_length)
                                .max(1);

                            update_royalties(&env, &color_owner, &sell, &color_owner_amount);

                            color_owner_amounts += color_owner_amount;
                        }
                        None => continue,
                    }
                }

                // transfer asset to buy glyph owner
                update_royalties(
                    &env,
                    &buy_glyph_owner,
                    &sell,
                    &(amount - author_amount - color_owner_amounts),
                );

                // swap glyph ownership
                env.storage()
                    .persistent()
                    .set::<Storage, Address>(&buy_glyph_owner_key, &owner);

                // remove all open buy glyph sell offers
                env.storage()
                    .persistent()
                    .remove::<Storage>(&open_glyph_buy_now_offers_key);

                env.events()
                    .publish((Symbol::new(&env, "offer_sell_asset"), sell, buy), Some(()));

                Ok(Some(()))
            }
            // No matching open counter offer. Add to buy glyph offers
            Err(_index) => {
                let offer_sell_asset_key =
                    Storage::OfferSellAsset(buy, sell.clone(), amount);

                let mut offers = env
                    .storage()
                    .persistent()
                    .get::<Storage, Vec<Address>>(&offer_sell_asset_key)
                    .unwrap_or(Vec::new(&env));

                match offers.binary_search(&owner) {
                    Ok(_index) => return Err(Error::OfferDuplicate),
                    Err(index) => offers.insert(index, owner.clone()),
                }

                env.storage()
                    .persistent()
                    .set::<Storage, Vec<Address>>(&offer_sell_asset_key, &offers);

                // transfer the asset to the contract for auto matching later
                let token_client = token::TokenClient::new(&env, &sell);

                token_client.transfer(&owner, &env.current_contract_address(), &amount);

                env.events().publish(
                    (Symbol::new(&env, "offer_sell_asset"), sell, buy),
                    None::<()>,
                );

                Ok(None)
            }
        }
    }
    pub fn offer_sell_glyph_remove(
        env: Env,
        sell: u32,
        buy: Option<OfferBuy>,
    ) -> Result<(), Error> {
        let glyph_owner_key = Storage::GlyphOwner(sell);
        let offer_sell_glyph_key = Storage::OfferSellGlyph(sell);

        let glyph_owner = env
            .storage()
            .persistent()
            .get::<Storage, Address>(&glyph_owner_key)
            .ok_or(Error::GlyphNotMinted)?;

        glyph_owner.require_auth();

        env.events().publish(
            (Symbol::new(&env, "offer_sell_glyph_remove"), sell, buy.clone()),
            (),
        );

        match buy {
            Some(buy) => {
                let mut offers = env
                    .storage()
                    .persistent()
                    .get::<Storage, Vec<OfferBuy>>(&offer_sell_glyph_key)
                    .unwrap_or(Vec::new(&env));

                match offers.binary_search(&buy) {
                    Ok(index) => {
                        offers.remove(index);

                        env.storage()
                            .persistent()
                            .set::<Storage, Vec<OfferBuy>>(&offer_sell_glyph_key, &offers);

                        Ok(())
                    }
                    Err(_index) => Err(Error::OfferNotFound),
                }
            }
            None => {
                env.storage().persistent().remove(&offer_sell_glyph_key);

                Ok(())
            }
        }
    }
    pub fn offer_sell_asset_remove(
        env: Env,
        sell: OfferSellAsset,
        buy: u32,
    ) -> Result<(), Error> {
        let OfferSellAsset(owner, sell, amount) = sell;

        owner.require_auth();

        let offer_sell_asset_key = Storage::OfferSellAsset(buy, sell.clone(), amount);

        let mut offers = env
            .storage()
            .persistent()
            .get::<Storage, Vec<Address>>(&offer_sell_asset_key)
            .unwrap_or(Vec::new(&env));

        match offers.binary_search(&owner) {
            Ok(index) => {
                offers.remove(index);
            }
            Err(_index) => return Err(Error::OfferNotFound),
        }

        env.storage()
            .persistent()
            .set::<Storage, Vec<Address>>(&offer_sell_asset_key, &offers);

        // refund the asset back to the user from the contract
        let token_client = token::TokenClient::new(&env, &sell);

        token_client.transfer(&env.current_contract_address(), &owner, &amount);

        env.events().publish(
            (Symbol::new(&env, "offer_sell_asset_remove"), sell, buy),
            (),
        );

        Ok(())
    }
    pub fn offer_sell_glyph_get(
        env: Env,
        sell: u32,
        buy: Option<OfferBuy>,
    ) -> Result<Option<u32>, Error> {
        let offers = env
            .storage()
            .persistent()
            .get::<Storage, Vec<OfferBuy>>(&Storage::OfferSellGlyph(sell))
            .unwrap_or(Vec::new(&env));

        match buy {
            Some(buy) => match offers.binary_search(&buy) {
                Ok(index) => Ok(Some(index)),
                Err(_index) => Ok(None),
            },
            None => {
                if offers.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(offers.len()))
                }
            }
        }
    }
    pub fn offer_sell_asset_get(
        env: Env,
        sell: OfferSellAssetGet,
        buy: u32,
    ) -> Result<Option<u32>, Error> {
        let OfferSellAssetGet(owner, sell, amount) = sell;

        let offers = env
            .storage()
            .persistent()
            .get::<Storage, Vec<Address>>(&Storage::OfferSellAsset(buy, sell, amount))
            .unwrap_or(Vec::new(&env));

        match owner {
            Some(owner) => match offers.binary_search(&owner) {
                Ok(index) => Ok(Some(index)),
                Err(_index) => Ok(None),
            },
            None => {
                if offers.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(offers.len()))
                }
            }
        }
    }

    pub fn royalties_get(env: Env, owner: Address, sac: Address) -> Result<i128, Error> {
        let royalties = env.storage()
            .persistent()
            .get::<Storage, i128>(&Storage::Royalties(owner, sac))
            .unwrap_or(0);
        
        Ok(royalties)
    }
    pub fn royalties_claim(env: Env, owner: Address, sac: Address) -> Result<i128, Error> {
        let royalties = env
            .storage()
            .persistent()
            .get::<Storage, i128>(&Storage::Royalties(owner.clone(), sac.clone()))
            .unwrap_or(0);

        if royalties == 0 {
            return Err(Error::NoRoyaltiesToClaim);
        }

        let token_client = token::TokenClient::new(&env, &sac);

        token_client.transfer(&env.current_contract_address(), &owner, &royalties);

        env.events().publish(
            (Symbol::new(&env, "royalties_claim"), owner, sac),
            royalties,
        );

        Ok(royalties)
    }
}

#[contractimpl]
impl CustomAccountInterface for Contract {
    type Error = Error;
    type Signature = Option<Vec<Val>>;

    #[allow(non_snake_case)]
    fn __check_auth(
        env: Env,
        _signature_payload: Hash<32>,
        _signatures: Option<Vec<Val>>,
        _auth_contexts: Vec<Context>,
    ) -> Result<(), Error> {
        let admin = env
            .storage()
            .instance()
            .get::<Storage, Address>(&Storage::Admin)
            .ok_or(Error::NotInitialized)?;

        admin.require_auth_for_args(vec![&env]);

        Ok(())
    }
}

fn get_palette(colors: Bytes) -> [u32; 256] {
    let colors_length = colors.len() as usize;

    // let mut colors_bytes = [0u8; 8192];
    let mut palette_bytes = [0u32; 256];

    // colors.copy_into_slice(&mut colors_bytes);

    // colors_bytes.

    for (index, color) in colors.into_iter().enumerate() {
        if index > colors_length {
            break;
        }

        palette_bytes[color as usize] += 1;
    }

    palette_bytes
}

fn update_royalties(env: &Env, owner: &Address, sac: &Address, amount: &i128) {
    let royalties_key = Storage::Royalties(owner.clone(), sac.clone());

    let royalties = env
        .storage()
        .persistent()
        .get::<Storage, i128>(&royalties_key)
        .unwrap_or(0);

    env.storage()
        .persistent()
        .set::<Storage, i128>(&royalties_key, &(royalties + amount));
}
