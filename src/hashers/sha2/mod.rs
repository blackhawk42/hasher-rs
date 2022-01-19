#[cfg(test)]
mod test;

use std::io;

use super::Hasher;

use sha2::{Digest, Sha256, Sha512};

/// Hasher for SHA-256.
pub struct SHA256Hasher;

impl SHA256Hasher {
    pub fn new() -> Self {
        Self {}
    }
}

impl<R: io::Read> Hasher<R> for SHA256Hasher {
    fn hash(&self, reader: &mut R) -> io::Result<Vec<u8>> {
        let mut sha256 = Sha256::new();

        let result = io::copy(reader, &mut sha256);

        match result {
            Ok(_) => {
                let result = sha256.finalize().to_vec();

                Ok(result)
            }

            Err(err) => Err(err),
        }
    }

    fn digest_size(&self) -> usize {
        32
    }
}

/// Hasher for SHA-512.
pub struct SHA512Hasher;

impl SHA512Hasher {
    pub fn new() -> Self {
        Self {}
    }
}

impl<R: io::Read> Hasher<R> for SHA512Hasher {
    fn hash(&self, reader: &mut R) -> io::Result<Vec<u8>> {
        let mut sha512 = Sha512::new();

        let result = io::copy(reader, &mut sha512);

        match result {
            Ok(_) => {
                let result = sha512.finalize().to_vec();

                Ok(result)
            }

            Err(err) => Err(err),
        }
    }

    fn digest_size(&self) -> usize {
        64
    }
}
