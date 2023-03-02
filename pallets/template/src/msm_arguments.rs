use ark_std::{test_rng, vec::Vec, UniformRand};

pub fn generate_arguments_sw<Field: ark_ec::models::short_weierstrass::SWCurveConfig>(
	size: u32,
) -> (Vec<ark_ec::short_weierstrass::Affine<Field>>, Vec<Field::ScalarField>) {
	let rng = &mut test_rng();
	let scalars = (0..size).map(|_| Field::ScalarField::rand(rng)).collect::<Vec<_>>();
	let bases = (0..size)
		.map(|_| ark_ec::short_weierstrass::Affine::<Field>::rand(rng))
		.collect::<Vec<_>>();
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
