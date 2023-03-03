use ark_serialize::{CanonicalSerialize, Compress};
use ark_std::{io::Cursor, vec, vec::Vec};

pub fn serialize_argument(argument: impl CanonicalSerialize) -> Vec<u8> {
	let mut serialized_argument = vec![0u8; argument.serialized_size(Compress::No)];
	let mut cursor = Cursor::new(&mut serialized_argument[..]);
	argument.serialize_uncompressed(&mut cursor).unwrap();
	serialized_argument
}
