use std::hash::{BuildHasherDefault, Hasher};

#[derive(Copy, Clone, Default)]
pub struct HashKeyHasher(u64);

impl Hasher for HashKeyHasher {
    fn write(&mut self, bytes: &[u8]) {
        for (index, byte) in bytes.iter().enumerate().take(8) {
            self.0 |= u64::from(*byte) << (index << 3);
        }
    }

    fn finish(&self) -> u64 {
        self.0
    }
}

pub type HashKeyBuildHasher = BuildHasherDefault<HashKeyHasher>;