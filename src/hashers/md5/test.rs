use super::*;

#[test]
fn digest_size_correct() {
    // digest size in *bytes*
    let md5_digest_size = 16;

    let hasher = MD5Hasher::new();
    let actual_digest_size = Hasher::<&[u8]>::digest_size(&hasher);

    assert_eq!(actual_digest_size, md5_digest_size);

    let data = b"Hello, world!";
    let sum = hasher
        .hash(&mut &data[..])
        .unwrap_or_else(|err| panic!("testing md5: {}", err));

    assert_eq!(sum.len(), md5_digest_size);
}

#[test]
fn digest_result_correct() {
    let data = b"Hello, world!";
    let expected_sum = vec![
        108, 211, 85, 109, 235, 13, 165, 75, 202, 6, 11, 76, 57, 71, 152, 57,
    ];

    let hasher = MD5Hasher::new();

    let sum = hasher
        .hash(&mut &data[..])
        .unwrap_or_else(|err| panic!("testing md5: {}", err));

    assert_eq!(sum, expected_sum);
}
