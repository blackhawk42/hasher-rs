use super::*;

#[test]
fn digest_sizes_correct() {
    // Digest size in *bits*
    let digest_bit_sizes = [160, 256, 512];
    let data = b"Hello, world!";

    for bit_size in digest_bit_sizes {
        let byte_size = bit_size / 8;

        let hasher = Blake2bHasher::new(byte_size);
        let actual_byte_size = Hasher::<&[u8]>::digest_size(&hasher);

        assert_eq!(
            byte_size, actual_byte_size,
            "testing blake2b{}: expected digest_size() to be {}, but got {}",
            bit_size, byte_size, actual_byte_size
        );

        let sum = hasher
            .hash(&mut &data[..])
            .unwrap_or_else(|err| panic!("blake2b{}: {}", bit_size, &err));
        let actual_sum_size = sum.len();

        assert_eq!(
            byte_size, actual_sum_size,
            "testing blake2b{}: expected sum to be of size {} bytes, but was {}",
            bit_size, byte_size, actual_sum_size
        );
    }
}

#[test]
#[should_panic = "digest_size cannot be 0 and should be <= 64 (max digest size for blake2b in bytes), but was 65"]
fn panics_on_digest_size_larger_64() {
    let _ = Blake2bHasher::new(65);
}

#[test]
#[should_panic = "digest_size cannot be 0 and should be <= 64 (max digest size for blake2b in bytes), but was 0"]
fn panics_on_digest_size_equal_0() {
    let _ = Blake2bHasher::new(0);
}

#[test]
fn digest_results_correct() {
    let data = b"Hello, world!";
    // Digest size in *bits*
    let digest_bit_size_results = [
        (
            160,
            vec![
                13, 236, 239, 61, 70, 10, 18, 35, 96, 36, 128, 128, 45, 112, 150, 24, 244, 25, 63,
                79,
            ],
        ),
        (
            256,
            vec![
                181, 218, 68, 28, 254, 114, 174, 4, 46, 244, 210, 177, 119, 66, 144, 127, 103, 93,
                228, 218, 87, 70, 45, 76, 54, 9, 194, 226, 237, 117, 89, 112,
            ],
        ),
        (
            512,
            vec![
                162, 118, 77, 19, 58, 22, 129, 107, 88, 71, 167, 55, 167, 134, 242, 236, 228, 193,
                72, 9, 92, 95, 170, 115, 226, 75, 76, 197, 214, 102, 195, 228, 94, 194, 113, 80,
                78, 20, 220, 97, 39, 221, 252, 228, 225, 68, 251, 35, 185, 26, 111, 123, 4, 181,
                61, 105, 85, 2, 41, 7, 34, 149, 59, 15,
            ],
        ),
    ];

    for (digest_bit_size, sum) in digest_bit_size_results {
        let digest_byte_size = digest_bit_size / 8;

        let hasher = Blake2bHasher::new(digest_byte_size);
        let actual_sum = hasher
            .hash(&mut &data[..])
            .unwrap_or_else(|err| panic!("blake2b{}: {}", digest_bit_size, &err));

        assert_eq!(
            sum, actual_sum,
            "testing blake2b{}: expected {:?}, but got {:?}",
            digest_bit_size, sum, actual_sum
        );
    }
}
