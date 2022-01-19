mod blake2;
mod crc;
mod md5;
mod sha1;
mod sha2;

pub use self::blake2::Blake2bHasher;
pub use self::crc::CRC32Hasher;
pub use self::md5::MD5Hasher;
pub use self::sha1::SHA1Hasher;
pub use self::sha2::{SHA256Hasher, SHA512Hasher};

use std::io;

/// Defines a "hasher", something that can take a reader and output the digest
/// of a hash.
///
/// A hasher should be able to be shared between threads, and the [`hash()`]
/// method safe to use between them.
///
/// [`hash()`]: Hasher::hash
pub trait Hasher<R>: Sync
where
    R: io::Read,
{
    /// Take a reader and compute a hash digest.
    ///
    /// The reader will normally be consumed until EOF. Each call should
    /// initialize a completely new digest, and be safe to call from multiple
    /// threads.
    ///
    /// The digest will be retuned as a vector of size equal to [`digest_size`].
    /// If there's an error, it will be an [`std::io::Error`].
    ///
    /// [`digest_size`]: Hasher::digest_size
    fn hash(&self, reader: &mut R) -> io::Result<Vec<u8>>;

    // Return the digest size in bytes.
    fn digest_size(&self) -> usize;
}
