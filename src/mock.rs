#![cfg(test)]

use std::hash::Hasher;

pub struct NopHash(pub u64);

impl Hasher for NopHash {
    fn write(&mut self, _: &[u8]) {}

    fn finish(&self) -> u64 {
        self.0
    }
}
