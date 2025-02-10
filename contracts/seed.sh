#!/bin/zsh

CONTRACT_ID=CBNNRNJ4DNN26GTOOUIFFOM6NMOMGBOBAZOMN26DJX3KENUFFGHACFZ3

function invoke_default() {
    stellar contract invoke --id $CONTRACT_ID --network testnet --source default -- $@
}
function invoke_user() {
    stellar contract invoke --id $CONTRACT_ID --network testnet --source user -- $@
}

# # `color_claim`
# # Default claim black
# invoke_default color_claim \
#     --color 0 \
#     --source default \
#     --owner default
# # User claim white
# invoke_user color_claim \
#     --color 16777215 \
#     --source user \
#     --owner user

# # `color_owner_transfer`
# # Default transfer black to user
# invoke_default color_owner_transfer \
#     --color 0 \
#     --to $(stellar keys address user)
# # User transfer white to default
# invoke_user color_owner_transfer \
#     --color 16777215 \
#     --to $(stellar keys address default)

# # `glyph_mint`
# # Default mint glyph
# invoke_default glyph_mint \
#     --colors "00010001" \
#     --legend "[0, 16777215]" \
#     --width 2 \
#     --author default \
#     --owner default \
#     --title "Hello World" \
#     --story "Lorem Ipsum"
# # User mint glyph
# invoke_user glyph_mint \
#     --colors "01000100" \
#     --legend "[0, 16777215]" \
#     --width 2 \
#     --author user \
#     --owner user \
#     --title "Hello World" \
#     --story "Lorem Ipsum"

# # `glyph_owner_transfer`
# # Default transfer glyph to user
# invoke_default glyph_owner_transfer \
#     --glyph_index 1 \
#     --to $(stellar keys address user)
# # User transfer glyph to default
# invoke_user glyph_owner_transfer \
#     --glyph_index 2 \
#     --to $(stellar keys address default)

# # `offer_sell_glyph` & `offer_sell_asset`
# # Sell glyph for glyph post
# invoke_user offer_sell_glyph \
#     --sell 1 \
#     --buy '{"Glyph": 2}'
# # Sell glyph for glyph match
# invoke_default offer_sell_glyph \
#     --sell 2 \
#     --buy '{"Glyph": 1}'
# # Sell glyph for asset post
# invoke_user offer_sell_glyph \
#     --sell 2 \
#     --buy '{"Asset":["CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC", "100"]}'
# # Sell asset for glyph match
# invoke_default offer_sell_asset \
#     --sell '["'"$(stellar keys address default)"'", "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC", "100"]' \
#     --buy 2
# # Sell asset for glyph post
# invoke_user offer_sell_asset \
#     --sell '["'"$(stellar keys address user)"'", "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC", "100"]' \
#     --buy 2
# # Sell glyph for asset match
# invoke_default offer_sell_glyph \
#     --sell 2 \
#     --buy '{"Asset":["CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC", "100"]}'

# # `offer_sell_glyph_remove`
# # Remove glyph for glyph offer
# invoke_default offer_sell_glyph \
#     --sell 1 \
#     --buy '{"Glyph": 1}'
# invoke_default offer_sell_glyph_remove \
#     --sell 1 \
#     --buy '{"Glyph": 1}'
# # Remove glyph for asset offer
# invoke_default offer_sell_glyph \
#     --sell 1 \
#     --buy '{"Asset":["CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC", "100"]}'
# invoke_default offer_sell_glyph_remove \
#     --sell 1 \
#     --buy '{"Asset":["CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC", "100"]}'
# Remove all glyph sell offers
# invoke_default offer_sell_glyph \
#     --sell 1 \
#     --buy '{"Glyph": 1}'
# invoke_default offer_sell_glyph \
#     --sell 1 \
#     --buy '{"Asset":["CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC", "100"]}'
# invoke_default offer_sell_glyph_remove \
#     --sell 1

# # `offer_sell_asset_remove`
# invoke_default offer_sell_asset \
#     --sell '["'"$(stellar keys address default)"'", "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC", "100"]' \
#     --buy 1
# invoke_default offer_sell_asset_remove \
#     --sell '["'"$(stellar keys address default)"'", "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC", "100"]' \
#     --buy 1

# # `royalties_claim`
# invoke_default royalties_claim \
#     --owner default \
#     --sac CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC
# invoke_user royalties_claim \
#     --owner user \
#     --sac CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC