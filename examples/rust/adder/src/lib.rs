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

use risc0_zkvm_host::{Prover, Receipt, Result};
use risc0_zkvm_serde::{from_slice, to_vec};

pub use adder_core::{AddRequest, AdditionResultCommit};

pub struct AdditionReceipt {
    receipt: Receipt,
}

impl AdditionReceipt {
    pub fn get_commit(&self) -> Result<AdditionResultCommit> {
        let msg = self.receipt.get_journal_vec()?;
        Ok(from_slice(msg.as_slice()).unwrap())
    }

    pub fn verify(&self) -> Result<AdditionResultCommit> {
        self.receipt
            .verify("examples/rust/adder/core/add")?;
        self.get_commit()
    }
}

pub fn add(
    a: u32,
    b: u32
) -> Result<AdditionReceipt> {
    let params = AddRequest {
        a: a,
        b: b
    };
    let mut prover = Prover::new("examples/rust/adder/core/add")?;
    let vec = to_vec(&params).unwrap();
    prover.add_input(vec.as_slice())?;
    let receipt = prover.run()?;
    Ok(AdditionReceipt { receipt })
}

#[cfg(test)]
mod tests {
    use log::LevelFilter;

    use super::*;

    #[ctor::ctor]
    fn init() {
        env_logger::builder().filter_level(LevelFilter::Info).init();
    }

    #[test]
    fn protocol() {
        let a = 1;
        let b = 2;
        let receipt = add(a, b).unwrap();
        receipt.verify().unwrap();
    }
}
