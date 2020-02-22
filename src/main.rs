use subxt::{
    balances::{self, AccountData, Balances},
    contracts::Contracts,
    system::{self, System},
    Client,
    DefaultNodeRuntime as Runtime,
};
use sp_keyring::AccountKeyring;
use sp_runtime::generic::SignedBlock;
use std::fs::File;
use std::io::prelude::*;
use codec::Encode;

fn main() {
    async_std::task::block_on(async move {

        let signer = AccountKeyring::Alice.pair();

        let dest = AccountKeyring::Bob.to_account_id();

        let cli: Client<Runtime, _> = subxt::ClientBuilder::<Runtime>::new()
            .build()
            .await.unwrap();

        let version = cli.version();
        let spec = version.spec_version;
        println!("SPEC: {}", spec);

        let block = cli.block::<primitives::H256>(None).await.unwrap();
        let block_num = block.as_ref().unwrap().block.header.number;
        block.unwrap()
             .block
             .extrinsics
             .iter()
            .enumerate()
            .map(|(i, e)| {
                 (i, e.encode())
             })
             .for_each(|(i, e)| {
                 let file_name = format!("./EXTRINSIC_spec_{}_block_{}_index_{}.bin", spec, block_num, i);
                 write_bytes_to_file(file_name, e);
             });
    });
}

pub fn write_bytes_to_file<S: Into<String>>(path: S, bytes: Vec<u8>) {
    let path: String = path.into();
    let mut buffer = File::create(path.as_str()).unwrap();
    buffer.write_all(&bytes[..]).unwrap();
}
