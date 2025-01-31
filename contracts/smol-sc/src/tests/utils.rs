use soroban_sdk::{testutils::Address as _, token, vec, Address, Bytes, Env, String};

use crate::{Contract, ContractArgs, ContractClient};

#[allow(dead_code)]
pub struct Init<'a> {
    pub admin: Address,
    pub fee_address: Address,
    pub fee_sac_address: Address,
    pub fee_sac_admin_client: token::StellarAssetClient<'a>,
    pub fee_sac_client: token::TokenClient<'a>,
    pub contract_id: Address,
    pub client: ContractClient<'a>,
}

pub fn initialize(
    env: &Env,
    mine_fee: i128,
    color_owner_royalty_rate: i128,
    glyph_author_royalty_rate: i128,
) -> Init {
    let admin = Address::generate(env);
    let fee_address = Address::generate(env);

    let fee_sac = env.register_stellar_asset_contract_v2(admin.clone());
    let fee_sac_address = fee_sac.address();
    let fee_sac_admin_client = token::StellarAssetClient::new(&env, &fee_sac_address);
    let fee_sac_client = token::TokenClient::new(&env, &fee_sac_address);

    let contract_id = env.register(
        Contract,
        ContractArgs::__constructor(
            &admin,
            &fee_sac_address,
            &fee_address,
            &mine_fee,
            &color_owner_royalty_rate,
            &glyph_author_royalty_rate,
        ),
    );
    let client = ContractClient::new(env, &contract_id);

    Init {
        admin,
        fee_address,
        fee_sac_address,
        fee_sac_admin_client,
        fee_sac_client,
        contract_id,
        client,
    }
}

pub fn mint(
    env: &Env,
    client: &ContractClient,
    contract_id: &Address,
    author: &Address,
    owner: &Address,
) -> u32 {
    const GLYPH_SIZE: usize = 45 * 45; // 95 * 95;

    let mut palette = [0u8; GLYPH_SIZE];

    env.as_contract(&contract_id, || {
        let end = 2;

        for i in 0..GLYPH_SIZE {
            let mut var = env.prng().gen_range::<u64>(0..end);

            while var > end {
                var = env.prng().gen_range::<u64>(0..end);
            }

            palette[i] = var as u8;
        }
    });

    let glyph_index = client.glyph_mint(
        &author,
        &owner,
        &Bytes::from_array(&env, &palette),
        &vec![&env, 0, 16777215],
        &45,
        &String::from_str(env, "Hello World"),
        &String::from_str(env, "Lorem Ipsum"),
    );

    glyph_index
}
