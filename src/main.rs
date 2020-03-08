use codec::Encode;
use std::{
    fs::OpenOptions,
    io::prelude::*,
    path::{Path, PathBuf},
};
use subxt::{Client, DefaultNodeRuntime as Runtime};
mod cli;

fn main() {
    let conf = cli::parse_args();

    async_std::task::block_on(async move {
        let cli: Client<Runtime, _> = subxt::ClientBuilder::<Runtime>::new()
            .build()
            .await
            .unwrap();

        let version = cli.version();
        let spec = version.spec_version;
        println!("SPEC: {}", spec);

        let block = cli.block::<primitives::H256>(conf.hash).await.unwrap();
        let meta = cli.raw_metadata(conf.hash).await.unwrap();
        let block_num = block.as_ref().unwrap().block.header.number;
        block
            .unwrap()
            .block
            .extrinsics
            .iter()
            .enumerate()
            .map(|(i, e)| (i, e.encode()))
            .for_each(|(i, e)| {
                let file_name = if let Some(o) = &conf.out {
                    o.clone()
                } else {
                    PathBuf::from(format!(
                        "./EXTRINSIC_spec_{}_block_{}_index_{}.bin",
                        spec, block_num, i
                    ))
                };
                write_bytes_to_file(file_name.as_path(), e);
            });
        let meta_path = format!("spec_{}_block_{}_METADATA.bin", spec, block_num);
        write_bytes_to_file(Path::new(meta_path.as_str()), meta)
    });
}

pub fn write_bytes_to_file<'a, P: Into<&'a Path>>(path: P, bytes: Vec<u8>) {
    let path: &Path = path.into();
    let mut buffer = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path.to_str().expect("Should be valid file path"))
        .expect("File Path should be valid");
    buffer.write_all(&bytes[..]).unwrap();
    buffer.write_all(b"\n").unwrap();
}
