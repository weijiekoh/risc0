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

#![no_main]
#![no_std]

use risc0_zkvm_guest::{env};

use adder_core::{AdditionResultCommit, AddRequest};

risc0_zkvm_guest::entry!(main);

pub fn main() {
    let request: AddRequest = env::read();
    let c: u32 = request.a + request.b;

    // The compiler will remove unused computations so `c` must be written to
    // the host. env::write writes to the private output, while env::commit
    // writes to the public output (aka the journal). Since we want to keep `c`
    // secret, we use env::write.
    env::write(&c);

    env::commit(&AdditionResultCommit {});
}
