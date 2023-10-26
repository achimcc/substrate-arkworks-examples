use crate::utils::{ProofFor, ScalarFieldFor, VerifyingKeyFor};
use ark_bls12_381::{Bls12_381, Fr, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_ec::{pairing::Pairing, short_weierstrass::SWCurveConfig, AffineRepr};
use ark_ff::Fp;
use ark_groth16::Groth16;
use ark_serialize::CanonicalDeserialize;
use ark_snark::SNARK;
use ark_std::vec::Vec;
use sp_ark_bls12_381::{
	Bls12_381 as Bls12_381Opt, Fr as FrOpt, G1Affine as G1AffineOpt,
	G1Projective as G1ProjectiveOpt, G2Affine as G2AffineOpt, G2Projective as G2ProjectiveOpt,
};

pub fn groth16_verify(vk: Vec<u8>, c: Vec<u8>, proof: Vec<u8>) {
	let vk = VerifyingKeyFor::<Bls12_381, Fr>::deserialize_uncompressed_unchecked(&vk[..]).unwrap();

	let c = Fp::deserialize_uncompressed_unchecked(&c[..]).unwrap();

	let proof = ProofFor::<Bls12_381, Fr>::deserialize_uncompressed_unchecked(&proof[..]).unwrap();

	let result = Groth16::<Bls12_381>::verify(&vk, &[c], &proof).unwrap();
	assert!(result);
}

pub fn groth16_verify_opt(vk: Vec<u8>, c: Vec<u8>, proof: Vec<u8>) {
	let vk = VerifyingKeyFor::<Bls12_381Opt, FrOpt>::deserialize_uncompressed_unchecked(&vk[..])
		.unwrap();
	let c = Fp::deserialize_uncompressed_unchecked(&c[..]).unwrap();
	let proof =
		ProofFor::<Bls12_381Opt, FrOpt>::deserialize_uncompressed_unchecked(&proof[..]).unwrap();

	let result = Groth16::<Bls12_381Opt>::verify(&vk, &[c], &proof).unwrap();
	assert!(result);
}

#[inline]
pub fn pairing(a: G1Affine, b: G2Affine) {
	let _out = ark_bls12_381::Bls12_381::multi_pairing([a], [b]);
}

#[inline]
pub fn pairing_opt(a: G1AffineOpt, b: G2AffineOpt) {
	let _out = Bls12_381Opt::multi_pairing([a], [b]);
}

#[inline]
pub fn msm_g1(bases: &[G1Affine], scalars: &[ScalarFieldFor<G1Affine>]) {
	let _out = <ark_bls12_381::g1::Config as SWCurveConfig>::msm(bases, scalars);
}

#[inline]
pub fn msm_g1_opt(bases: &[G1AffineOpt], scalars: &[<G1AffineOpt as AffineRepr>::ScalarField]) {
	let _out = <sp_ark_bls12_381::g1::Config as SWCurveConfig>::msm(bases, scalars);
}

#[inline]
pub fn msm_g2(bases: &[G2Affine], scalars: &[ScalarFieldFor<G2Affine>]) {
	let _out = <ark_bls12_381::g2::Config as SWCurveConfig>::msm(bases, scalars);
}

#[inline]
pub fn msm_g2_opt(bases: &[G2AffineOpt], scalars: &[ScalarFieldFor<G2AffineOpt>]) {
	let _out = <sp_ark_bls12_381::g2::Config as SWCurveConfig>::msm(bases, scalars);
}

#[inline]
pub fn mul_projective_g1(base: &G1Projective, scalar: &[u64]) {
	let _out = <ark_bls12_381::g1::Config as SWCurveConfig>::mul_projective(base, scalar);
}

#[inline]
pub fn mul_projective_g1_opt(base: &G1ProjectiveOpt, scalar: &[u64]) {
	let _out = <sp_ark_bls12_381::g1::Config as SWCurveConfig>::mul_projective(base, scalar);
}

#[inline]
pub fn mul_affine_g1(base: &G1Affine, scalar: &[u64]) {
	let _out = <ark_bls12_381::g1::Config as SWCurveConfig>::mul_affine(base, scalar);
}

#[inline]
pub fn mul_affine_g1_opt(base: &G1AffineOpt, scalar: &[u64]) {
	let _out = <sp_ark_bls12_381::g1::Config as SWCurveConfig>::mul_affine(base, scalar);
}

#[inline]
pub fn mul_projective_g2(base: &G2Projective, scalar: &[u64]) {
	let _out = <ark_bls12_381::g2::Config as SWCurveConfig>::mul_projective(base, scalar);
}

#[inline]
pub fn mul_projective_g2_opt(base: &G2ProjectiveOpt, scalar: &[u64]) {
	let _out = <sp_ark_bls12_381::g2::Config as SWCurveConfig>::mul_projective(base, scalar);
}

#[inline]
pub fn mul_affine_g2(base: &G2Affine, scalar: &[u64]) {
	let _out = <ark_bls12_381::g2::Config as SWCurveConfig>::mul_affine(base, scalar);
}

#[inline]
pub fn mul_affine_g2_opt(base: &G2AffineOpt, scalar: &[u64]) {
	let _out = <sp_ark_bls12_381::g2::Config as SWCurveConfig>::mul_affine(base, scalar);
}
