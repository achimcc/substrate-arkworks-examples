use ark_bw6_761::{G1Affine, G1Projective, G2Affine, G2Projective, BW6_761};
use sp_ark_models::{pairing::Pairing, short_weierstrass::SWCurveConfig};
use sp_bw6_761::curves::{
	g1::{G1Affine as G1AffineOpt, G1Projective as G1ProjectiveOpt},
	g2::{G2Affine as G2AffineOpt, G2Projective as G2ProjectiveOpt},
	BW6_761 as BW6_761Opt,
};

pub fn pairing(a: G1Affine, b: G2Affine) {
	let _out = BW6_761::multi_pairing([a], [b]);
}

pub fn pairing_opt(a: G1AffineOpt, b: G2AffineOpt) {
	let _out = BW6_761Opt::multi_pairing([a], [b]);
}

pub fn msm_g1(
	bases: &[ark_ec::short_weierstrass::Affine<ark_bw6_761::g1::Config>],
	scalars: &[<ark_bw6_761::g1::Config as ark_ec::CurveConfig>::ScalarField],
) {
	let _out = <ark_bw6_761::g1::Config as SWCurveConfig>::msm(bases, scalars);
}

pub fn msm_g1_opt(
	bases: &[sp_bw6_761::g1::G1Affine],
	scalars: &[<sp_bw6_761::g1::Config as sp_ark_models::CurveConfig>::ScalarField],
) {
	let _out = <sp_bw6_761::g1::Config as SWCurveConfig>::msm(bases, scalars);
}

pub fn msm_g2(
	bases: &[ark_ec::models::short_weierstrass::Affine<ark_bw6_761::g2::Config>],
	scalars: &[<ark_bw6_761::g2::Config as ark_ec::CurveConfig>::ScalarField],
) {
	let _out = <ark_bw6_761::g2::Config as SWCurveConfig>::msm(bases, scalars);
}

pub fn msm_g2_opt(
	bases: &[sp_ark_models::short_weierstrass::Affine<sp_bw6_761::g2::Config>],
	scalars: &[<sp_bw6_761::g2::Config as sp_ark_models::CurveConfig>::ScalarField],
) {
	let _out = <sp_bw6_761::g2::Config as SWCurveConfig>::msm(bases, scalars);
}

pub fn mul_affine_g1(
	base: &ark_ec::short_weierstrass::Affine<ark_bw6_761::g1::Config>,
	scalar: &[u64],
) {
	let _out = <ark_bw6_761::g1::Config as SWCurveConfig>::mul_affine(base, scalar);
}

pub fn mul_affine_g1_opt(base: &sp_bw6_761::g1::G1Affine, scalar: &[u64]) {
	let _out = <sp_bw6_761::g1::Config as SWCurveConfig>::mul_affine(base, scalar);
}

pub fn mul_projective_g1(base: &G1Projective, scalar: &[u64]) {
	let _out = <ark_bw6_761::g1::Config as SWCurveConfig>::mul_projective(base, scalar);
}

pub fn mul_projective_g1_opt(base: &G1ProjectiveOpt, scalar: &[u64]) {
	let _out = <sp_bw6_761::g1::Config as SWCurveConfig>::mul_projective(base, scalar);
}

pub fn mul_affine_g2(
	base: &ark_ec::short_weierstrass::Affine<ark_bw6_761::g2::Config>,
	scalar: &[u64],
) {
	let _out = <ark_bw6_761::g2::Config as SWCurveConfig>::mul_affine(base, scalar);
}

pub fn mul_affine_g2_opt(
	base: &sp_ark_models::short_weierstrass::Affine<sp_bw6_761::g2::Config>,
	scalar: &[u64],
) {
	let _out = <sp_bw6_761::g2::Config as SWCurveConfig>::mul_affine(base, scalar);
}

pub fn mul_projective_g2(base: &G2Projective, scalar: &[u64]) {
	let _out = <ark_bw6_761::g2::Config as SWCurveConfig>::mul_projective(base, scalar);
}

pub fn mul_projective_g2_opt(base: &G2ProjectiveOpt, scalar: &[u64]) {
	let _out = <sp_bw6_761::g2::Config as SWCurveConfig>::mul_projective(base, scalar);
}
