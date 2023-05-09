use ark_bls12_381::{Bls12_381, Fr as BlsFr};
use ark_ec::{pairing::Pairing, CurveConfig};
use ark_ff::Fp;
use ark_groth16::Groth16;
use ark_serialize::{CanonicalDeserialize, Compress, Validate};
use ark_snark::SNARK;
use ark_std::{
	io::{Cursor, Error},
	vec::Vec,
};
use frame_support::assert_ok;
pub use sp_bls12_381::{
	curves::{
		g1::{G1Affine as G1AffineOptimized, G1Projective as G1ProjectiveOptimized},
		g2::{G2Affine as G2AffineOptimized, G2Projective as G2ProjectiveOptimized},
		Bls12_381 as Bls12_381Optimized,
	},
	fr::Fr as BlsFrOptimized,
};

pub fn do_pairing_optimized(a: G1AffineOptimized, b: G2AffineOptimized) -> Result<(), Error> {
	let _ = Bls12_381Optimized::multi_pairing([a], [b]);
	Ok(())
}

pub fn do_pairing(a: ark_bls12_381::G1Affine, b: ark_bls12_381::G2Affine) -> Result<(), Error> {
	let _ = ark_bls12_381::Bls12_381::multi_pairing([a], [b]);
	Ok(())
}

pub fn do_msm_g1(
	bases: &[ark_ec::short_weierstrass::Affine<ark_bls12_381::g1::Config>],
	scalars: &[<ark_bls12_381::g1::Config as CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out = <ark_bls12_381::g1::Config as ark_ec::models::short_weierstrass::SWCurveConfig>::msm(
		bases, scalars,
	);
	Ok(())
}

pub fn do_msm_g1_optimized(
	bases: &[sp_ark_models::short_weierstrass::Affine<sp_bls12_381::g1::Config>],
	scalars: &[<sp_bls12_381::g1::Config as CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out = <sp_bls12_381::g1::Config as sp_ark_models::short_weierstrass::SWCurveConfig>::msm(
		bases, scalars,
	);
	Ok(())
}

pub fn do_msm_g2(
	bases: &[ark_ec::short_weierstrass::Affine<ark_bls12_381::g2::Config>],
	scalars: &[<ark_bls12_381::g2::Config as CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out = <ark_bls12_381::g2::Config as ark_ec::short_weierstrass::SWCurveConfig>::msm(
		bases, scalars,
	);
	Ok(())
}

pub fn do_msm_g2_optimized(
	bases: &[sp_ark_models::short_weierstrass::Affine<sp_bls12_381::g2::Config>],
	scalars: &[<sp_bls12_381::g2::Config as CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out = <sp_bls12_381::g2::Config as sp_ark_models::short_weierstrass::SWCurveConfig>::msm(
		bases, scalars,
	);
	Ok(())
}

pub fn do_mul_affine_g1(
	base: &ark_ec::short_weierstrass::Affine<ark_bls12_381::g1::Config>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out = <ark_bls12_381::g1::Config as ark_ec::short_weierstrass::SWCurveConfig>::mul_affine(
		base, scalar,
	);
	Ok(())
}

pub fn do_mul_affine_g1_optimized(
	base: &sp_ark_models::short_weierstrass::Affine<sp_bls12_381::g1::Config>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out =
		<sp_bls12_381::g1::Config as sp_ark_models::short_weierstrass::SWCurveConfig>::mul_affine(
			base, scalar,
		);
	Ok(())
}

pub fn do_mul_projective_g1(
	base: &ark_ec::short_weierstrass::Projective<ark_bls12_381::g1::Config>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out =
		<ark_bls12_381::g1::Config as ark_ec::short_weierstrass::SWCurveConfig>::mul_projective(
			base, scalar,
		);
	Ok(())
}

pub fn do_mul_projective_g1_optimized(
	base: &sp_ark_models::short_weierstrass::Projective<sp_bls12_381::g1::Config>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out = <sp_bls12_381::g1::Config as sp_ark_models::short_weierstrass::SWCurveConfig>::mul_projective(
		base,
		scalar,
	);
	Ok(())
}

pub fn do_mul_affine_g2(
	base: &ark_ec::short_weierstrass::Affine<ark_bls12_381::g2::Config>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out = <ark_bls12_381::g2::Config as ark_ec::short_weierstrass::SWCurveConfig>::mul_affine(
		base, scalar,
	);
	Ok(())
}

pub fn do_mul_affine_g2_optimized(
	base: &sp_ark_models::short_weierstrass::Affine<sp_bls12_381::g2::Config>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out =
		<sp_bls12_381::g2::Config as sp_ark_models::short_weierstrass::SWCurveConfig>::mul_affine(
			base, scalar,
		);
	Ok(())
}

pub fn do_mul_projective_g2(
	base: &ark_ec::short_weierstrass::Projective<ark_bls12_381::g2::Config>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out =
		<ark_bls12_381::g2::Config as ark_ec::short_weierstrass::SWCurveConfig>::mul_projective(
			base, scalar,
		);
	Ok(())
}

pub fn do_mul_projective_g2_optimized(
	base: &sp_ark_models::short_weierstrass::Projective<sp_bls12_381::g2::Config>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out = <sp_bls12_381::g2::Config as sp_ark_models::short_weierstrass::SWCurveConfig>::mul_projective(
		base,
		scalar,
	);
	Ok(())
}

pub fn do_verify_groth16(vk: Vec<u8>, c: Vec<u8>, proof: Vec<u8>) -> Result<(), Error> {
	let cursor = Cursor::new(&vk);
	let vk = <Groth16<Bls12_381> as SNARK<BlsFr>>::VerifyingKey::deserialize_with_mode(
		cursor,
		Compress::No,
		Validate::No,
	)
	.unwrap();

	let cursor = Cursor::new(&c);
	let c = Fp::deserialize_with_mode(cursor, Compress::No, Validate::No).unwrap();

	let cursor = Cursor::new(&proof);
	let proof = <Groth16<Bls12_381> as SNARK<BlsFr>>::Proof::deserialize_with_mode(
		cursor,
		Compress::No,
		Validate::No,
	)
	.unwrap();

	assert_ok!(Groth16::<Bls12_381>::verify(&vk, &[c], &proof));

	Ok(())
}

pub fn do_verify_groth16_optimized(vk: Vec<u8>, c: Vec<u8>, proof: Vec<u8>) -> Result<(), Error> {
	let cursor = Cursor::new(&vk);
	let vk = <Groth16<Bls12_381Optimized> as SNARK<BlsFrOptimized>>::VerifyingKey::deserialize_with_mode(
					cursor,
					Compress::No,
					Validate::No,
				)
				.unwrap();

	let cursor = Cursor::new(&c);
	let c = Fp::deserialize_with_mode(cursor, Compress::No, Validate::No).unwrap();

	let cursor = Cursor::new(&proof);
	let proof =
		<Groth16<Bls12_381Optimized> as SNARK<BlsFrOptimized>>::Proof::deserialize_with_mode(
			cursor,
			Compress::No,
			Validate::No,
		)
		.unwrap();

	assert_ok!(Groth16::<Bls12_381Optimized>::verify(&vk, &[c], &proof));

	Ok(())
}
