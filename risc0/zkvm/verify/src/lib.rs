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

mod circuit;
mod eval;
mod poly_op;
mod poly_ops;
mod taps;

use arrayref::array_ref;

use serde::{Deserialize, Serialize};

use risc0_zkp_core::sha::Digest;
use risc0_zkp_verify::verify::verify;

use crate::circuit::Risc0Circuit;

#[derive(Deserialize, Serialize)]
pub struct Receipt {
    journal: Vec<u8>,
    seal: Vec<u32>,
}

impl Receipt {
    pub fn verify(&self) {
        let mut circuit = Risc0Circuit::default();
        verify(&mut circuit, &self.seal);
        assert!(self.journal.len() == (self.seal[8] as usize));
        if self.journal.len() > 32 {
            let digest = Digest::hash_bytes(&self.journal);
            assert!(digest == Digest::from_u32s(&self.seal[0..8]));
        } else {
            let mut vec = self.journal.clone();
            vec.resize(32, 0);
            for i in 0..8 {
                assert!(self.seal[i] == u32::from_le_bytes(*array_ref![&vec, i * 4, 4]));
            }
        }
    }

    pub fn get_journal_u32(&self) -> Vec<u32> {
        let mut as_words: Vec<u32> = vec![];
        assert!(self.journal.len() % 4 == 0);
        for i in 0..(self.journal.len() / 4) {
            as_words.push(u32::from_le_bytes(*array_ref![&self.journal, i * 4, 4]));
        }
        as_words
    }
}

#[cfg(test)]
mod tests {
    use super::Receipt;
    use std::vec::Vec;
    use core::convert::TryFrom;
    use std::fs;
    use std::io;
    use test_log::test;

    #[test]
    fn test_receipt() -> io::Result<()> {
        log::set_max_level(log::LevelFilter::Info);
        let data: Vec<u8> = fs::read("src/simple_receipt.receipt")?;
        let as_u32: Vec<u32> = data
            .chunks(4)
            .map(|bytes| u32::from_le_bytes(<[u8; 4]>::try_from(bytes).unwrap()))
            .collect();
        let receipt: Receipt = risc0_zkvm_serde::from_slice(&as_u32).unwrap();

        std::println!(
            "Receipt: journal length {} seal length {}",
            receipt.journal.len(),
            receipt.seal.len()
        );

        for i in 0..50 {
            std::print!(" {}", receipt.seal[i]);
        }
        std::println!("\n");

        receipt.verify();
        Ok(())
    }
}
