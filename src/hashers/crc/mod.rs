#[cfg(test)]
mod test;

use crc32fast;

use super::Hasher;

use std::io;

/// Hasher for CRC-32.
pub struct CRC32Hasher;

impl CRC32Hasher {
    pub fn new() -> Self {
        Self {}
    }
}

struct CRC32Digest(crc32fast::Hasher);

impl io::Write for CRC32Digest {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.update(buf);

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<R: io::Read> Hasher<R> for CRC32Hasher {
    fn hash(&self, reader: &mut R) -> io::Result<Vec<u8>> {
        let mut crc32 = CRC32Digest(crc32fast::Hasher::new());

        let result = io::copy(reader, &mut crc32);

        match result {
            Ok(_) => {
                let result = crc32.0.finalize().to_be_bytes().to_vec();

                Ok(result)
            }

            Err(err) => Err(err),
        }
    }

    fn digest_size(&self) -> usize {
        4
    }
}
