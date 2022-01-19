#[cfg(test)]
mod test;

use std::io;

use sha1::{Digest, Sha1};

use super::Hasher;

// Hasher for SHA-1;
pub struct SHA1Hasher;

impl SHA1Hasher {
    pub fn new() -> Self {
        Self {}
    }
}

impl<R: io::Read> Hasher<R> for SHA1Hasher {
    fn hash(&self, reader: &mut R) -> std::io::Result<Vec<u8>> {
        let mut sha1 = Sha1::new();

        let result = io::copy(reader, &mut sha1);

        match result {
            Ok(_) => {
                let result = sha1.finalize().to_vec();

                Ok(result)
            }

            Err(err) => Err(err),
        }
    }

    fn digest_size(&self) -> usize {
        20
    }
}
