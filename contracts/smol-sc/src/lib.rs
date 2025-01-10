#![no_std]

use soroban_fixed_point_math::SorobanFixedPoint;
use soroban_sdk::{
    auth::{Context, CustomAccountInterface}, contract, contracterror, contractimpl, contracttype, crypto::Hash, token, vec, Address, Bytes, BytesN, Env, Val, Vec
};

#[contracterror]
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    ColorOutOfRange = 3,
    ColorAlreadyClaimed = 4,
    ColorNotClaimed = 5,
    GlyphAlreadyMinted = 6,
    GlyphNotMinted = 7,
    OfferDuplicate = 8,
    OfferNotFound = 9,
    NoRoyaltiesToClaim = 10,
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
    ColorOwner(u32),                           // Color : Owner
    Glyph(BytesN<32>),                         // Glyph Hash : Glyph
    GlyphOwner(BytesN<32>),                    // Glyph Hash : Owner
    OfferSellGlyph(BytesN<32>),                // Glyph Hash : Vec<OfferBuy>
    OfferSellAsset(BytesN<32>, Address, i128), // Glyph Hash, SAC, Amount : Vec<Owner>
    Royalties(Address, Address),                        // Owner, SAC : Amount
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum OfferBuy {
    Asset(Address, i128), // SAC, Amount
    Glyph(BytesN<32>),    // Glyph Hash
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
// add events

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

        env.storage().instance().set(&Storage::Admin, &admin);
        env.storage().instance().set(&Storage::FeeSAC, &fee_sac);
        env.storage()
            .instance()
            .set(&Storage::FeeAddress, &fee_address);
        env.storage()
            .instance()
            .set(&Storage::ColorClaimFee, &color_claim_fee);
        env.storage()
            .instance()
            .set(&Storage::ColorOwnerRoyaltyRate, &color_owner_royalty_rate);
        env.storage()
            .instance()
            .set(&Storage::GlyphAuthorRoyaltyRate, &glyph_author_royalty_rate);

        Ok(())
    }
    pub fn update() {
        // TODO update contract variables
    }
    pub fn upgrade(env: Env, hash: BytesN<32>) -> Result<(), Error> {
        let admin = env.storage().instance().get::<Storage, Address>(&Storage::Admin).ok_or(Error::NotInitialized)?;

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

        Ok(())
    }

    pub fn glyph_mint(
        env: Env,
        author: Address,
        owner: Address,
        colors: Bytes,
        legend: Vec<u32>,
        width: u32,
    ) -> Result<BytesN<32>, Error> {
        let glyph = Glyph {
            author,
            colors: colors.clone(),
            legend,
            width,
        };

        let glyph_hash = env.crypto().sha256(&colors).to_bytes();
        let glyph_key = Storage::Glyph(glyph_hash.clone());

        if env.storage().persistent().has(&glyph_key) {
            return Err(Error::GlyphAlreadyMinted);
        }

        env.storage()
            .persistent()
            .set::<Storage, Glyph>(&glyph_key, &glyph);

        env.storage()
            .persistent()
            .set::<Storage, Address>(&Storage::GlyphOwner(glyph_hash.clone()), &owner);

        Ok(glyph_hash)
    }
    pub fn glyph_get(env: Env, glyph_hash: BytesN<32>) -> Result<Glyph, Error> {
        env.storage()
            .persistent()
            .get::<Storage, Glyph>(&Storage::Glyph(glyph_hash))
            .ok_or(Error::GlyphNotMinted)
    }
    pub fn glyph_owner_get(env: Env, glyph_hash: BytesN<32>) -> Result<Address, Error> {
        env.storage()
            .persistent()
            .get::<Storage, Address>(&Storage::GlyphOwner(glyph_hash))
            .ok_or(Error::GlyphNotMinted)
    }
    pub fn glyph_owner_transfer(
        env: Env,
        glyph_hash: BytesN<32>,
        to: Address,
    ) -> Result<(), Error> {
        let glyph_owner_key = Storage::GlyphOwner(glyph_hash);

        let glyph_owner = env
            .storage()
            .persistent()
            .get::<Storage, Address>(&glyph_owner_key)
            .ok_or(Error::GlyphNotMinted)?;

        glyph_owner.require_auth();

        env.storage().persistent().set(&glyph_owner_key, &to);

        Ok(())
    }

    pub fn offer_sell_glyph(
        env: Env,
        sell: BytesN<32>,
        buy: OfferBuy,
    ) -> Result<Option<()>, Error> {
        let glyph_owner_key = Storage::GlyphOwner(sell.clone());
        let offer_sell_glyph_key = Storage::OfferSellGlyph(sell.clone());

        let glyph_owner = env
            .storage()
            .persistent()
            .get::<Storage, Address>(&glyph_owner_key)
            .ok_or(Error::GlyphNotMinted)?;

        glyph_owner.require_auth();

        match &buy {
            OfferBuy::Glyph(buy) => {
                let offer_buy_glyph_key = Storage::OfferSellGlyph(buy.clone());

                let offers = env
                    .storage()
                    .persistent()
                    .get::<Storage, Vec<OfferBuy>>(&offer_buy_glyph_key)
                    .unwrap_or(Vec::new(&env));

                match offers.binary_search(OfferBuy::Glyph(sell.clone())) {
                    // Found a matching offer
                    Ok(_index) => {
                        let buy_glyph_owner_key = Storage::GlyphOwner(buy.clone());
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
                            .remove::<Storage>(&Storage::OfferSellGlyph(buy.clone()));

                        // remove all open sell glyph sell offers
                        env.storage()
                            .persistent()
                            .remove::<Storage>(&offer_sell_glyph_key);

                        // delete the offer
                        env.storage().persistent().remove(&offer_buy_glyph_key);

                        return Ok(Some(()));
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
                            .get::<Storage, Glyph>(&Storage::Glyph(sell))
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

                        let mut color_owner_amounts = 0;

                        // TODO likely need to limit this to the first N ordered by highest count (125 storage gets)
                        for (index, count) in get_palette(colors).into_iter().enumerate() {
                            if index >= legend.len() as usize {
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
                        update_royalties(&env, &glyph_owner, &buy, &(amount - author_amount - color_owner_amounts));

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

                        return Ok(Some(()));
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

        match offers.binary_search(&buy) {
            Ok(_index) => {
                return Err(Error::OfferDuplicate);
            }
            Err(index) => offers.insert(index, buy),
        }

        env.storage()
            .persistent()
            .set::<Storage, Vec<OfferBuy>>(&offer_sell_glyph_key, &offers);

        Ok(None)
    }
    pub fn offer_sell_asset(
        env: Env,
        sell: OfferSellAsset,
        buy: BytesN<32>,
    ) -> Result<Option<()>, Error> {
        let OfferSellAsset(owner, sell, amount) = sell;

        owner.require_auth();

        let open_glyph_buy_now_offers_key = Storage::OfferSellGlyph(buy.clone());
        let open_glyph_buy_now_offers = env
            .storage()
            .persistent()
            .get::<Storage, Vec<OfferBuy>>(&open_glyph_buy_now_offers_key)
            .unwrap_or(Vec::new(&env));

        match open_glyph_buy_now_offers.binary_search(OfferBuy::Asset(sell.clone(), amount)) {
            // Found a matching open counter offer. Take it
            Ok(_index) => {
                let buy_glyph_owner_key = Storage::GlyphOwner(buy.clone());
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

                let mut color_owner_amounts = 0;

                // TODO likely need to limit this to the first N ordered by highest count (125 storage gets)
                for (index, count) in get_palette(colors).into_iter().enumerate() {
                    if index >= legend.len() as usize {
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
                update_royalties(&env, &buy_glyph_owner, &sell, &(amount - author_amount - color_owner_amounts));

                // swap glyph ownership
                env.storage()
                    .persistent()
                    .set::<Storage, Address>(&buy_glyph_owner_key, &owner);

                // remove all open buy glyph sell offers
                env.storage()
                    .persistent()
                    .remove::<Storage>(&open_glyph_buy_now_offers_key);

                Ok(Some(()))
            }
            // No matching open counter offer. Add to buy glyph offers
            Err(_index) => {
                let offer_sell_asset_key = Storage::OfferSellAsset(buy, sell.clone(), amount);

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

                Ok(None)
            }
        }
    }
    pub fn offer_sell_glyph_remove(env: Env, sell: BytesN<32>, buy: Option<OfferBuy>) -> Result<(), Error> {
        let glyph_owner_key = Storage::GlyphOwner(sell.clone());
        let offer_sell_glyph_key = Storage::OfferSellGlyph(sell);

        let glyph_owner = env
            .storage()
            .persistent()
            .get::<Storage, Address>(&glyph_owner_key)
            .ok_or(Error::GlyphNotMinted)?;

        glyph_owner.require_auth();

        let mut offers = env
            .storage()
            .persistent()
            .get::<Storage, Vec<OfferBuy>>(&offer_sell_glyph_key)
            .unwrap_or(Vec::new(&env));

        match buy {
            Some(buy) => match offers.binary_search(&buy) {
                Ok(index) => {
                    offers.remove(index);

                    env.storage()
                        .persistent()
                        .set::<Storage, Vec<OfferBuy>>(&offer_sell_glyph_key, &offers);

                        Ok(())
                }
                Err(_index) => {
                    return Err(Error::OfferNotFound);
                }
            },
            None => {
                env.storage().persistent().remove(&offer_sell_glyph_key);

                Ok(())
            }
        }
    }
    pub fn offer_sell_asset_remove(
        env: Env,
        sell: OfferSellAsset,
        buy: BytesN<32>,
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

        Ok(())
    }
    pub fn offer_sell_glyph_get(
        env: Env,
        sell: BytesN<32>,
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
        buy: BytesN<32>,
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
        let admin = env.storage().instance().get::<Storage, Address>(&Storage::Admin).ok_or(Error::NotInitialized)?;

        admin.require_auth_for_args(vec![&env]);

        Ok(())
    }
}

fn get_palette(colors: Bytes) -> [u32; 256] {
    let mut colors_bytes = [0u8; 2025];
    let mut palette_bytes = [0u32; 256];

    colors.copy_into_slice(&mut colors_bytes);

    for color in colors_bytes {
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