use super::*;

#[test]
fn digest_size_correct() {
	// digest size in *bytes*
	let crc32_digest_size = 4;

	let hasher = CRC32Hasher::new();
	let actual_digest_size = Hasher::<&[u8]>::digest_size(&hasher);

	assert_eq!(actual_digest_size, crc32_digest_size);

	let data = b"Hello, world!";
	let sum = hasher.hash(&mut &data[..]).unwrap_or_else(|err| panic!("testing crc32: {}", err));

	assert_eq!(sum.len(), crc32_digest_size);
}

#[test]
fn digest_result_correct() {
	let data = b"Hello, world!";
	let expected_sum = vec![235, 230, 198, 230];

	let hasher = CRC32Hasher::new();

	let sum = hasher.hash(&mut &data[..]).unwrap_or_else(|err| panic!("testing crc32: {}", err));

	assert_eq!(sum, expected_sum);
}