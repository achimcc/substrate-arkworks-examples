use crate::utils::{ProofFor, VerifyingKeyFor};
use ark_bls12_381::{Bls12_381, Fr as BlsFr, G1Affine, G2Affine};
use ark_ec::{pairing::Pairing, CurveConfig};
use ark_ff::Fp;
use ark_groth16::Groth16;
use ark_serialize::{CanonicalDeserialize, Compress, Validate};
use ark_snark::SNARK;
use ark_std::{
	io::{Cursor, Error},
	vec::Vec,
};
pub use sp_bls12_381::{
	curves::{
		g1::{G1Affine as G1AffineOpt, G1Projective as G1ProjectiveOpt},
		g2::{G2Affine as G2AffineOpt, G2Projective as G2ProjectiveOpt},
		Bls12_381 as Bls12_381Opt,
	},
	fr::Fr as BlsFrOpt,
};

pub fn groth16_verify(vk: Vec<u8>, c: Vec<u8>, proof: Vec<u8>) {
	let vk =
		VerifyingKeyFor::<Bls12_381, BlsFr>::deserialize_uncompressed_unchecked(&vk[..]).unwrap();

	let c = Fp::deserialize_uncompressed_unchecked(&c[..]).unwrap();

	let proof =
		ProofFor::<Bls12_381, BlsFr>::deserialize_uncompressed_unchecked(&proof[..]).unwrap();

	let result = Groth16::<Bls12_381>::verify(&vk, &[c], &proof).unwrap();
	assert!(result);
}

pub fn groth16_verify_opt(vk: Vec<u8>, c: Vec<u8>, proof: Vec<u8>) {
	let vk = VerifyingKeyFor::<Bls12_381Opt, BlsFrOpt>::deserialize_uncompressed_unchecked(&vk[..])
		.unwrap();
	let c = Fp::deserialize_uncompressed_unchecked(&c[..]).unwrap();
	let proof =
		ProofFor::<Bls12_381Opt, BlsFrOpt>::deserialize_uncompressed_unchecked(&proof[..]).unwrap();

	let result = Groth16::<Bls12_381Opt>::verify(&vk, &[c], &proof).unwrap();
	assert!(result);
}

pub fn pairing(a: G1Affine, b: G2Affine) {
	let _out = ark_bls12_381::Bls12_381::multi_pairing([a], [b]);
}

pub fn pairing_opt(a: G1AffineOpt, b: G2AffineOpt) {
	let _out = Bls12_381Opt::multi_pairing([a], [b]);
}

pub fn msm_g1(
	bases: &[ark_ec::short_weierstrass::Affine<ark_bls12_381::g1::Config>],
	scalars: &[<ark_bls12_381::g1::Config as CurveConfig>::ScalarField],
) {
	let _out = <ark_bls12_381::g1::Config as ark_ec::models::short_weierstrass::SWCurveConfig>::msm(
		bases, scalars,
	);
}

pub fn msm_g1_opt(
	bases: &[sp_ark_models::short_weierstrass::Affine<sp_bls12_381::g1::Config>],
	scalars: &[<sp_bls12_381::g1::Config as CurveConfig>::ScalarField],
) {
	let _out = <sp_bls12_381::g1::Config as sp_ark_models::short_weierstrass::SWCurveConfig>::msm(
		bases, scalars,
	);
}

pub fn msm_g2(
	bases: &[ark_ec::short_weierstrass::Affine<ark_bls12_381::g2::Config>],
	scalars: &[<ark_bls12_381::g2::Config as CurveConfig>::ScalarField],
) {
	let _out = <ark_bls12_381::g2::Config as ark_ec::short_weierstrass::SWCurveConfig>::msm(
		bases, scalars,
	);
}

pub fn msm_g2_opt(
	bases: &[sp_ark_models::short_weierstrass::Affine<sp_bls12_381::g2::Config>],
	scalars: &[<sp_bls12_381::g2::Config as CurveConfig>::ScalarField],
) {
	let _out = <sp_bls12_381::g2::Config as sp_ark_models::short_weierstrass::SWCurveConfig>::msm(
		bases, scalars,
	);
}

pub fn mul_projective_g1(
	base: &ark_ec::short_weierstrass::Projective<ark_bls12_381::g1::Config>,
	scalar: &[u64],
) {
	let _out =
		<ark_bls12_381::g1::Config as ark_ec::short_weierstrass::SWCurveConfig>::mul_projective(
			base, scalar,
		);
}

pub fn mul_projective_g1_opt(
	base: &sp_ark_models::short_weierstrass::Projective<sp_bls12_381::g1::Config>,
	scalar: &[u64],
) {
	let _out = <sp_bls12_381::g1::Config as sp_ark_models::short_weierstrass::SWCurveConfig>::mul_projective(
		base,
		scalar,
	);
}

pub fn mul_affine_g1(
	base: &ark_ec::short_weierstrass::Affine<ark_bls12_381::g1::Config>,
	scalar: &[u64],
) {
	let _out = <ark_bls12_381::g1::Config as ark_ec::short_weierstrass::SWCurveConfig>::mul_affine(
		base, scalar,
	);
}

pub fn mul_affine_g1_opt(
	base: &sp_ark_models::short_weierstrass::Affine<sp_bls12_381::g1::Config>,
	scalar: &[u64],
) {
	let _out =
		<sp_bls12_381::g1::Config as sp_ark_models::short_weierstrass::SWCurveConfig>::mul_affine(
			base, scalar,
		);
}

pub fn mul_projective_g2(
	base: &ark_ec::short_weierstrass::Projective<ark_bls12_381::g2::Config>,
	scalar: &[u64],
) {
	let _out =
		<ark_bls12_381::g2::Config as ark_ec::short_weierstrass::SWCurveConfig>::mul_projective(
			base, scalar,
		);
}

pub fn mul_projective_g2_opt(
	base: &sp_ark_models::short_weierstrass::Projective<sp_bls12_381::g2::Config>,
	scalar: &[u64],
) {
	let _out = <sp_bls12_381::g2::Config as sp_ark_models::short_weierstrass::SWCurveConfig>::mul_projective(
		base,
		scalar,
	);
}

pub fn mul_affine_g2(
	base: &ark_ec::short_weierstrass::Affine<ark_bls12_381::g2::Config>,
	scalar: &[u64],
) {
	let _out = <ark_bls12_381::g2::Config as ark_ec::short_weierstrass::SWCurveConfig>::mul_affine(
		base, scalar,
	);
}

pub fn mul_affine_g2_opt(
	base: &sp_ark_models::short_weierstrass::Affine<sp_bls12_381::g2::Config>,
	scalar: &[u64],
) {
	let _out =
		<sp_bls12_381::g2::Config as sp_ark_models::short_weierstrass::SWCurveConfig>::mul_affine(
			base, scalar,
		);
}
