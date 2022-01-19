use super::*;

#[test]
fn digest_sizes_correct() {
	// digest size in *bytes*
	let sha1_digest_size = 20;

	let hasher = SHA1Hasher::new();
	let actual_digest_size = Hasher::<&[u8]>::digest_size(&hasher);

	assert_eq!(actual_digest_size, sha1_digest_size);

	let data = b"Hello, world!";
	let sum = hasher.hash(&mut &data[..]).unwrap_or_else(|err| panic!("testing sha1: {}", err));

	assert_eq!(sum.len(), sha1_digest_size);
}

#[test]
fn digest_result_correct() {
	let data = b"Hello, world!";
	let expected_sum = vec![148, 58, 112, 45, 6, 243, 69, 153, 174, 225, 248, 218, 142, 249, 247, 41, 96, 49, 214, 153];

	let hasher = SHA1Hasher::new();

	let sum = hasher.hash(&mut &data[..]).unwrap_or_else(|err| panic!("testing sha1: {}", err));

	assert_eq!(sum, expected_sum);
}