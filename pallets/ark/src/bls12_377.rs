use crate::utils::ScalarFieldFor;
use ark_bls12_377::{Bls12_377, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_ec::{pairing::Pairing, short_weierstrass::SWCurveConfig};
use sp_ark_bls12_377::{
	Bls12_377 as Bls12_377Opt, G1Affine as G1AffineOpt, G1Projective as G1ProjectiveOpt,
	G2Affine as G2AffineOpt, G2Projective as G2ProjectiveOpt,
};

#[inline]
pub fn pairing(a: G1Affine, b: G2Affine) {
	let _ = Bls12_377::multi_pairing([a], [b]);
}

#[inline]
pub fn pairing_opt(a: G1AffineOpt, b: G2AffineOpt) {
	let _ = Bls12_377Opt::multi_pairing([a], [b]);
}

#[inline]
pub fn msm_g1(bases: &[G1Affine], scalars: &[ScalarFieldFor<G1Affine>]) {
	let _ = <ark_bls12_377::g1::Config as SWCurveConfig>::msm(bases, scalars).unwrap();
}

#[inline]
pub fn msm_g1_opt(bases: &[G1AffineOpt], scalars: &[ScalarFieldFor<G1AffineOpt>]) {
	let _ = <sp_ark_bls12_377::g1::Config as SWCurveConfig>::msm(bases, scalars);
}

#[inline]
pub fn msm_g2(bases: &[G2Affine], scalars: &[ScalarFieldFor<G2Affine>]) {
	let _out = <ark_bls12_377::g2::Config as SWCurveConfig>::msm(bases, scalars);
}

#[inline]
pub fn msm_g2_opt(bases: &[G2AffineOpt], scalars: &[ScalarFieldFor<G2AffineOpt>]) {
	let _out = <sp_ark_bls12_377::g2::Config as SWCurveConfig>::msm(bases, scalars);
}

#[inline]
pub fn mul_affine_g1(base: &G1Affine, scalar: &[u64]) {
	let _out = <ark_bls12_377::g1::Config as SWCurveConfig>::mul_affine(base, scalar);
}

#[inline]
pub fn mul_affine_g1_opt(base: &G1AffineOpt, scalar: &[u64]) {
	let _out = <sp_ark_bls12_377::g1::Config as SWCurveConfig>::mul_affine(base, scalar);
}

#[inline]
pub fn mul_projective_g1(base: &G1Projective, scalar: &[u64]) {
	let _out = <ark_bls12_377::g1::Config as SWCurveConfig>::mul_projective(base, scalar);
}

#[inline]
pub fn mul_projective_g1_opt(base: &G1ProjectiveOpt, scalar: &[u64]) {
	let _out = <sp_ark_bls12_377::g1::Config as SWCurveConfig>::mul_projective(base, scalar);
}

#[inline]
pub fn mul_affine_g2(base: &G2Affine, scalar: &[u64]) {
	let _out = <ark_bls12_377::g2::Config as SWCurveConfig>::mul_affine(base, scalar);
}

#[inline]
pub fn mul_affine_g2_opt(base: &G2AffineOpt, scalar: &[u64]) {
	let _out = <sp_ark_bls12_377::g2::Config as SWCurveConfig>::mul_affine(base, scalar);
}

#[inline]
pub fn mul_projective_g2(base: &G2Projective, scalar: &[u64]) {
	let _out = <ark_bls12_377::g2::Config as SWCurveConfig>::mul_projective(base, scalar);
}

#[inline]
pub fn mul_projective_g2_opt(base: &G2ProjectiveOpt, scalar: &[u64]) {
	let _out = <sp_ark_bls12_377::g2::Config as SWCurveConfig>::mul_projective(base, scalar);
}
