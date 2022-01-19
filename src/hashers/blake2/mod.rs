#[cfg(test)]
mod test;

use std::io;

use blake2::digest::VariableOutput;
use blake2::Blake2bVar;

use super::Hasher;

/// Hasher for Blake2b.
///
/// It's digest size is variable.
pub struct Blake2bHasher {
    digest_size: usize,
}

impl Blake2bHasher {
    /// Creates a new Blake2bHasher.
    /// 
    /// # Panics
    /// 
    /// Will panic if `digest_size == 0` or `digest_size > 64` (max digest size for blake2b).
    pub fn new(digest_size: usize) -> Self {
        if digest_size > 64 || digest_size == 0 {
            panic!("digest_size cannot be 0 and should be <= 64 (max digest size for blake2b in bytes), but was {}", digest_size)
        }

        Self { digest_size }
    }
}

impl<R: io::Read> Hasher<R> for Blake2bHasher {
    fn hash(&self, reader: &mut R) -> io::Result<Vec<u8>> {
        let mut blake2 = Blake2bVar::new(self.digest_size).unwrap();

        let result = io::copy(reader, &mut blake2);

        match result {
            Ok(_) => {
                let mut result = vec![0; self.digest_size];
                blake2.finalize_variable(&mut result).unwrap();

                Ok(result)
            }

            Err(err) => Err(err),
        }
    }

    fn digest_size(&self) -> usize {
        self.digest_size
    }
}
