use ark_serialize::{CanonicalSerialize, Compress};
use ark_std::{io::Cursor, test_rng, vec, vec::Vec, UniformRand};

pub fn serialize_argument(argument: impl CanonicalSerialize) -> Vec<u8> {
	let mut serialized_argument = vec![0u8; argument.serialized_size(Compress::No)];
	let mut cursor = Cursor::new(&mut serialized_argument[..]);
	argument.serialize_uncompressed(&mut cursor).unwrap();
	serialized_argument
}

pub fn generate_msm_args<Group: ark_ec::VariableBaseMSM>(
	size: u32,
) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
	let rng = &mut test_rng();
	let scalars = (0..size).map(|_| Group::ScalarField::rand(rng)).collect::<Vec<_>>();
	let bases = (0..size).map(|_| Group::rand(rng)).collect::<Vec<_>>();
	let bases = bases
		.iter()
		.map(|base| {
			let mut serialized_base = vec![0u8; base.serialized_size(Compress::No)];
			let mut cursor = Cursor::new(&mut serialized_base[..]);
			base.serialize_uncompressed(&mut cursor).unwrap();
			serialized_base
		})
		.collect::<Vec<_>>();
	let scalars = scalars
		.iter()
		.map(|scalar| {
			let mut serialized_scalar = vec![0u8; scalar.serialized_size(Compress::No)];
			let mut cursor = Cursor::new(&mut serialized_scalar[..]);
			scalar.serialize_uncompressed(&mut cursor).unwrap();
			serialized_scalar
		})
		.collect::<Vec<_>>();
	(bases, scalars)
}

fn generate_scalar() -> Vec<u8> {
	let rng = &mut test_rng();
	let scalar = u64::rand(rng);
	let mut serialized_scalar = vec![0u8; scalar.serialized_size(Compress::No)];
	let mut cursor = Cursor::new(&mut serialized_scalar[..]);
	scalar.serialize_uncompressed(&mut cursor).unwrap();
	serialized_scalar
}

fn generate_base<Group: CanonicalSerialize + UniformRand>() -> Vec<u8> {
	let rng = &mut test_rng();
	let base = Group::rand(rng);
	let mut serialized_base = vec![0u8; base.serialized_size(Compress::No)];
	let mut cursor = Cursor::new(&mut serialized_base[..]);
	base.serialize_uncompressed(&mut cursor).unwrap();
	serialized_base
}

pub fn generate_scalar_args<Group: CanonicalSerialize + UniformRand>() -> (Vec<u8>, Vec<u8>) {
	let serialized_scalar = generate_scalar();
	let serialized_base = generate_base::<Group>();
	(serialized_base, serialized_scalar)
}

pub fn generate_pairing_args<
	GroupA: CanonicalSerialize + UniformRand,
	GroupB: CanonicalSerialize + UniformRand,
>() -> (Vec<u8>, Vec<u8>) {
	let serialized_a = generate_base::<GroupA>();
	let serialized_b = generate_base::<GroupB>();
	(serialized_a, serialized_b)
}
