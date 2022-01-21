use super::*;

#[test]
fn digest_sizes_correct() {
    // sizes in *bits*
    let hasher_expectedsize: Vec<(Box<dyn Hasher<&[u8]>>, usize)> = vec![
        (Box::new(SHA256Hasher::new()), 256),
        (Box::new(SHA512Hasher::new()), 512),
    ];

    for (hasher, digest_size_bits) in hasher_expectedsize {
        let expected_size_bytes = digest_size_bits / 8;
        let actual_size_bytes = hasher.digest_size();

        assert_eq!(
            actual_size_bytes, expected_size_bytes,
            "testing sha{}: actual size (bytes) was {}, expected {}",
            digest_size_bits, actual_size_bytes, expected_size_bytes
        );
    }
}

#[test]
fn digest_results_correct() {
    let data = b"Hello, world!";

    // Digest sizes in *bits*
    let hasher_sizebits_expectedresult: Vec<(Box<dyn Hasher<&[u8]>>, usize, Vec<u8>)> = vec![
        (
            Box::new(SHA256Hasher::new()),
            256,
            vec![
                49, 95, 91, 219, 118, 208, 120, 196, 59, 138, 192, 6, 78, 74, 1, 100, 97, 43, 31,
                206, 119, 200, 105, 52, 91, 252, 148, 199, 88, 148, 237, 211,
            ],
        ),
        (
            Box::new(SHA512Hasher::new()),
            512,
            vec![
                193, 82, 124, 216, 147, 193, 36, 119, 61, 129, 25, 17, 151, 12, 143, 230, 232, 87,
                214, 223, 93, 201, 34, 107, 216, 161, 96, 97, 76, 12, 217, 99, 164, 221, 234, 43,
                148, 187, 125, 54, 2, 30, 249, 216, 101, 213, 206, 162, 148, 168, 45, 212, 154, 11,
                178, 105, 245, 31, 110, 122, 87, 247, 148, 33,
            ],
        ),
    ];

    for (hasher, size_bits, expected_sum) in hasher_sizebits_expectedresult {
        let actual_sum = hasher
            .hash(&mut &data[..])
            .unwrap_or_else(|err| panic!("testing sha{}: {}", size_bits, err));

        assert_eq!(
            actual_sum, expected_sum,
            "testing sha{}: got sum {:?}, expected {:?}",
            size_bits, actual_sum, expected_sum
        );
    }
}
