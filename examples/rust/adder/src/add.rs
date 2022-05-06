// Copyright 2022 Risc0, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use clap::{Arg, Command};
use log::LevelFilter;

use lib::add;

fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let matches = Command::new("add")
        .version("0.1.0")
        .author("Koh Wei Jie")
        .about("A simple adder with Risc0")
        .arg(
            Arg::new("a")
                .long("a")
                .takes_value(true)
                .help("Value A"),
        )
        .arg(
            Arg::new("b")
                .long("b")
                .takes_value(true)
                .help("Value B"),
        )
        .get_matches();

    let a_str = matches.value_of("a").unwrap();
    let b_str = matches.value_of("b").unwrap();

    let a: u32 = a_str.parse().unwrap();
    let b: u32 = b_str.parse().unwrap();

    let receipt = add(a, b).unwrap();
    receipt.verify().unwrap();

    log::info!("Value A: {:?}", &a);
    log::info!("Value B: {:?}", &b);
}
