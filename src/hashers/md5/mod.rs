#[cfg(test)]
mod test;

use std::io;

use md5::{Digest, Md5};

use super::Hasher;

/// Hasher for MD5.
pub struct MD5Hasher;

impl MD5Hasher {
    pub fn new() -> Self {
        Self {}
    }
}

impl<R: io::Read> Hasher<R> for MD5Hasher {
    fn hash(&self, reader: &mut R) -> io::Result<Vec<u8>> {
        let mut md5 = Md5::new();

        let result = io::copy(reader, &mut md5);

        match result {
            Ok(_) => {
                let output = md5.finalize().to_vec();

                Ok(output)
            }

            Err(err) => Err(err),
        }
    }

    fn digest_size(&self) -> usize {
        16
    }
}
