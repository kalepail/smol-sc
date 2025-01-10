import { Asset } from '@stellar/stellar-sdk'

console.log(
    new Asset('0000FF', 'GCQIQYTHEJSDXNVC5BBPLQD2YQKED6XZJLL73HHV5ISMU44P5N5BM3FE').toXDRObject().alphaNum12().toXDR('base64')
);

console.log(
    Buffer.from(
        [48, 48, 48, 48, 70, 70, 0, 0, 0, 0, 0, 0]
    ).toString('utf8')
);