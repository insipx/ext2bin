// MIT License

// Copyright (c) 2020 Andrew Plaza

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use clap::{load_yaml, value_t, App};
use derive_builder::Builder;
use primitives::H256;
use std::{path::PathBuf, str::FromStr};

#[derive(Default, Builder, Debug)]
pub struct Configuration {
    pub hash: Option<H256>,
    pub out: Option<PathBuf>,
}

pub fn parse_args() -> Configuration {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let mut config = ConfigurationBuilder::default();

    let hash = if let Some(h) = value_t!(matches, "hash", String).ok() {
        Some(H256::from_str(&h).expect("Hash should be H256 type"))
    } else {
        None
    };
    config.hash(hash);

    let out = value_t!(matches, "out", PathBuf).ok();
    config.out(out);

    config.build().expect("Could not build config")
}
