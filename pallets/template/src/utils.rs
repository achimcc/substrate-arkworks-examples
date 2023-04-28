#![allow(dead_code)]
use crate::ArkScale;
use ark_scale::hazmat::ArkScaleProjective;
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
) -> (ArkScale<Vec<Group>>, ArkScale<Vec<Group::ScalarField>>) {
	let rng = &mut test_rng();
	let scalars = (0..size).map(|_| Group::ScalarField::rand(rng)).collect::<Vec<_>>();
	let bases = (0..size).map(|_| Group::rand(rng)).collect::<Vec<_>>();
	let bases: ArkScale<Vec<Group>> = bases.into();
	let scalars: ArkScale<Vec<Group::ScalarField>> = scalars.into();
	(bases, scalars)
}

fn generate_scalar() -> ArkScale<Vec<u64>> {
	let rng = &mut test_rng();
	let scalar = vec![u64::rand(rng)];
	let scalar: ArkScale<Vec<u64>> = scalar.into();
	scalar
}

fn generate_base<Group: CanonicalSerialize + UniformRand>() -> ArkScale<Group> {
	let rng = &mut test_rng();
	let base = Group::rand(rng);
	let base: ArkScale<Group> = base.into();
	base
}

fn generate_base_projective<Group: CanonicalSerialize + UniformRand>() -> ArkScaleProjective<Group>
{
	let rng = &mut test_rng();
	let base = Group::rand(rng);
	let base: ArkScaleProjective<Group> = base.into();
	base
}

pub fn generate_scalar_args<Group: CanonicalSerialize + UniformRand>(
) -> (ArkScale<Group>, ArkScale<Vec<u64>>) {
	let serialized_scalar = generate_scalar();
	let serialized_base = generate_base::<Group>();
	(serialized_base, serialized_scalar)
}

pub fn generate_scalar_args_projective<Group: CanonicalSerialize + UniformRand>(
) -> (ArkScaleProjective<Group>, ArkScale<Vec<u64>>) {
	let scalar = generate_scalar();
	let base = generate_base_projective::<Group>();
	(base, scalar)
}

pub fn generate_pairing_args<
	GroupA: CanonicalSerialize + UniformRand,
	GroupB: CanonicalSerialize + UniformRand,
>() -> (ArkScale<GroupA>, ArkScale<GroupB>) {
	let a = generate_base::<GroupA>();
	let b = generate_base::<GroupB>();
	(a, b)
}
