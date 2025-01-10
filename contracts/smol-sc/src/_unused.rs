pub fn claim_color(env: Env, miner: Address, color: u32) -> Result<Address, Error> {
    let mut hex = [0; 12];

    let color = if color > 0xFFFFFF {
        return Err(Error::ColorOutOfRange);
    } else {
        color
    };

    for i in 0..6 {
        let digit = (color >> (20 - 4 * i)) & 0xF;

        hex[i] = if digit < 10 {
            b'0' + digit as u8
        } else {
            b'A' + (digit - 10) as u8
        };
    }

    let pubkey = env
        .storage()
        .instance()
        .get::<Storage, BytesN<32>>(&Storage::PubKey)
        .ok_or(Error::NotInitialized)?;

    let asset = Asset::CreditAlphanum12(AlphaNum12 {
        asset_code: AssetCode12(hex),
        issuer: AccountId(PublicKey::PublicKeyTypeEd25519(Uint256(pubkey.to_array()))),
    })
    .to_xdr(Limits::none())
    .unwrap();

    let deployer = env
        .deployer()
        .with_stellar_asset(Bytes::from_slice(&env, &asset));
    let deploy_address = deployer.deployed_address();
    let token_client = token::StellarAssetClient::new(&env, &deploy_address);
    let asset_key = Storage::ColorMiner(deploy_address.clone());

    // Asset has already been deployed
    if env.storage().persistent().has(&asset_key) {
        return Err(Error::AssetAlreadyDeployed);
    }

    env.storage().persistent().set(&asset_key, &miner);

    let fee_address = env
        .storage()
        .instance()
        .get::<Storage, Address>(&Storage::FeeAddress)
        .ok_or(Error::NotInitialized)?;
    let fee_amount = env
        .storage()
        .instance()
        .get::<Storage, i128>(&Storage::MineFee)
        .ok_or(Error::NotInitialized)?;
    let fee_sac = env
        .storage()
        .instance()
        .get::<Storage, Address>(&Storage::FeeSAC)
        .ok_or(Error::NotInitialized)?;
    let fee_client = token::TokenClient::new(&env, &fee_sac);

    miner.require_auth();

    fee_client.transfer(&miner, &fee_address, &fee_amount);

    deployer.deploy();

    token_client.admin().require_auth();

    token_client.set_admin(&env.current_contract_address());

    Ok(deploy_address)
}