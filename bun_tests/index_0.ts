import { xdr, Address, Asset, scValToNative } from '@stellar/stellar-sdk'

console.log(
    new Asset('0000FF', 'GCQIQYTHEJSDXNVC5BBPLQD2YQKED6XZJLL73HHV5ISMU44P5N5BM3FE').toXDRObject().alphaNum12().toXDR('base64')
);

console.log(
    Buffer.from(
        [48, 48, 48, 48, 70, 70, 0, 0, 0, 0, 0, 0]
    ).toString('utf8')
);

console.log(
    Address.fromString('CA3SPLLDBCOVZDDFAXNDNDBWH5E3ULRX5AL2MVQWOCGLJO7IGO5YHE7J').toBuffer(),
    Address.fromString('GBRZ7B3ZSKWFKLK2K2KFBPSKRW3OZF3QDJGE4TXDOSKUCOHC5T6N7G76').toScAddress().toXDR(),
);

console.log(
    Buffer.from([1, 0, 1, 0]).toString('hex')
);

const xdr_base64 = 'AAAAEAAAAAEAAAADAAAADwAAAAVBc3NldAAAAAAAABIAAAAB15KLcsJwPM/q9+uf9O9NUEpVqLl5/JtFDqLIQrTRzmEAAAAKAAAAAAAAAAAAAAAAAAAAZA=='

console.log(
    scValToNative(xdr.ScVal.fromXDR(xdr_base64, 'base64'))
);

const amount = 1_0000000n
// const amount = 170141183460469231731687303715884105727n
// const amount = -170141183460469231731687303715884105728n

function bigIntToUint8Array(num: bigint, littleEndian = false) {
    // Convert BigInt to a hex string
    let hex = num.toString(16).padStart(32, "0");

    // Convert hex to byte array
    let byteArray = new Uint8Array(
        hex.match(/../g)!.map(byte => parseInt(byte, 16))
    );

    // Convert to Little Endian if needed
    return littleEndian ? byteArray.reverse() : byteArray;
}

function bigIntToUint8ArraySigned(num: bigint, littleEndian = false) {
    if (num < 0n) {
        num = (1n << 128n) + num;
    }
    return bigIntToUint8Array(num, littleEndian);
}

function bigIntToUint8ArraySimple(num: bigint) {
    return new Uint8Array(num.toString(16).padStart(32, "0").match(/../g)!.map(b => parseInt(b, 16)));
}

console.log(bigIntToUint8ArraySigned(amount)); 
console.log(bigIntToUint8ArraySimple(amount));