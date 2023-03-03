use ark_serialize::{CanonicalSerialize, Compress};
use ark_std::{io::Cursor, test_rng, vec, vec::Vec, UniformRand};

pub fn serialize_argument(argument: impl CanonicalSerialize) -> Vec<u8> {
	let mut serialized_argument = vec![0u8; argument.serialized_size(Compress::No)];
	let mut cursor = Cursor::new(&mut serialized_argument[..]);
	argument.serialize_uncompressed(&mut cursor).unwrap();
	serialized_argument
}

pub fn generate_arguments_sw<Group: ark_ec::VariableBaseMSM>(
	size: u32,
) -> (Vec<Group>, Vec<Group::ScalarField>) {
	let rng = &mut test_rng();
	let scalars = (0..size).map(|_| Group::ScalarField::rand(rng)).collect::<Vec<_>>();
	let bases = (0..size).map(|_| Group::rand(rng)).collect::<Vec<_>>();
	(bases, scalars)
}

pub fn generate_arguments_te<Field: ark_ec::models::twisted_edwards::TECurveConfig>(
	size: u32,
) -> (Vec<ark_ec::twisted_edwards::Affine<Field>>, Vec<Field::ScalarField>) {
	let rng = &mut test_rng();
	let scalars = (0..size).map(|_| Field::ScalarField::rand(rng)).collect::<Vec<_>>();
	let bases = (0..size)
		.map(|_| ark_ec::twisted_edwards::Affine::<Field>::rand(rng))
		.collect::<Vec<_>>();
	(bases, scalars)
}
