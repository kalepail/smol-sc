import { encodePng } from '@lunapaint/png-codec'
import { Address, scValToNative, xdr } from "@stellar/stellar-sdk/minimal";
import { Server } from "@stellar/stellar-sdk/rpc";

const rpc = new Server('https://soroban-testnet.stellar.org');

let { entries } = await rpc._getLedgerEntries(xdr.LedgerKey.contractData(new xdr.LedgerKeyContractData({
    contract: Address.fromString('CDE37MDCRXLY5VJYRNYTSBBDBUIBIP5ZYO54T25P3UTFIOOGML4LZ7V4').toScAddress(),
    key: xdr.ScVal.scvVec([
        xdr.ScVal.scvSymbol('Glyph'),
        xdr.ScVal.scvU32(2)
    ]),
    durability: xdr.ContractDataDurability.persistent(),
})));

for (let entry of entries || []) {
    let data = xdr.LedgerEntryData.fromXDR(entry.xdr, 'base64');

    console.log(
        scValToNative(data.contractData().val())
    );

    // console.log(
    //     entry.val.contractData().val().map()?.map((entry) => {
    //         return scValToNative(entry.val())
    //     })
    // );

    // let glyph = scValToNative(entry.val.contractData().val());

    // console.log(glyph);

    // let palette = [...glyph.colors].map((legend_index: number) => {
    //     // return `#${glyph.legend[legend_index].toString(16).padStart(6, '0')}`;
    //     return glyph.legend[legend_index];
    // });

    // let base64 = await paletteToBase64(palette, glyph.width);

    // console.log(Buffer.from(base64).toString('base64'));
}

async function paletteToBase64(palette: number[], width: number) {
    const rgb_palette: number[] = []

    for (const color of palette) {
        rgb_palette.push(...[
            color >> 16,
            color >> 8 & 0xff,
            color & 0xff,
            255
        ])
    }

    const { data } = await encodePng({
        data: new Uint8Array(rgb_palette),
        width,
        height: Math.ceil(palette.length / width), 
    })
    
    return data
}