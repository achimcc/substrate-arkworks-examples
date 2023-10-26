use crate::utils::ScalarFieldFor;
use ark_ec::{short_weierstrass::SWCurveConfig, twisted_edwards::TECurveConfig};
use ark_ed_on_bls12_381_bandersnatch::{EdwardsAffine, EdwardsProjective, SWAffine, SWProjective};
use sp_ark_ed_on_bls12_381_bandersnatch::curves::{
	EdwardsAffine as EdwardsAffineOpt, EdwardsProjective as EdwardsProjectiveOpt,
	SWAffine as SWAffineOpt, SWProjective as SWProjectiveOpt,
};

#[inline]
pub fn msm_sw(bases: &[SWAffine], scalars: &[ScalarFieldFor<SWAffine>]) {
	let _out =
		<ark_ed_on_bls12_381_bandersnatch::EdwardsConfig as SWCurveConfig>::msm(bases, scalars);
}

#[inline]
pub fn msm_sw_opt(bases: &[SWAffineOpt], scalars: &[ScalarFieldFor<SWAffineOpt>]) {
	let _out =
		<sp_ark_ed_on_bls12_381_bandersnatch::EdwardsConfig as SWCurveConfig>::msm(bases, scalars);
}

#[inline]
pub fn msm_te(bases: &[EdwardsAffine], scalars: &[ScalarFieldFor<EdwardsAffine>]) {
	let _out = <ark_ed_on_bls12_381_bandersnatch::BandersnatchConfig as TECurveConfig>::msm(
		bases, scalars,
	);
}

#[inline]
pub fn msm_te_opt(bases: &[EdwardsAffineOpt], scalars: &[ScalarFieldFor<EdwardsAffineOpt>]) {
	let _out = <sp_ark_ed_on_bls12_381_bandersnatch::BandersnatchConfig as TECurveConfig>::msm(
		bases, scalars,
	);
}

#[inline]
pub fn mul_affine_sw(base: &SWAffine, scalar: &[u64]) {
	let _out = <ark_ed_on_bls12_381_bandersnatch::EdwardsConfig as SWCurveConfig>::mul_affine(
		base, scalar,
	);
}

#[inline]
pub fn mul_affine_te(base: &EdwardsAffine, scalar: &[u64]) {
	let _out = <ark_ed_on_bls12_381_bandersnatch::EdwardsConfig as TECurveConfig>::mul_affine(
		base, scalar,
	);
}

#[inline]
pub fn mul_affine_sw_opt(base: &SWAffineOpt, scalar: &[u64]) {
	let _out = <sp_ark_ed_on_bls12_381_bandersnatch::EdwardsConfig as SWCurveConfig>::mul_affine(
		base, scalar,
	);
}

#[inline]
pub fn mul_affine_te_opt(base: &EdwardsAffineOpt, scalar: &[u64]) {
	let _out = <sp_ark_ed_on_bls12_381_bandersnatch::EdwardsConfig as TECurveConfig>::mul_affine(
		base, scalar,
	);
}

#[inline]
pub fn mul_projective_sw(base: &SWProjective, scalar: &[u64]) {
	let _out = <ark_ed_on_bls12_381_bandersnatch::EdwardsConfig as SWCurveConfig>::mul_projective(
		base, scalar,
	);
}

#[inline]
pub fn mul_projective_sw_opt(base: &SWProjectiveOpt, scalar: &[u64]) {
	let _out =
		<sp_ark_ed_on_bls12_381_bandersnatch::EdwardsConfig as SWCurveConfig>::mul_projective(
			base, scalar,
		);
}

#[inline]
pub fn mul_projective_te(base: &EdwardsProjective, scalar: &[u64]) {
	let _out = <ark_ed_on_bls12_381_bandersnatch::EdwardsConfig as TECurveConfig>::mul_projective(
		base, scalar,
	);
}

#[inline]
pub fn mul_projective_te_opt(base: &EdwardsProjectiveOpt, scalar: &[u64]) {
	let _out =
		<sp_ark_ed_on_bls12_381_bandersnatch::EdwardsConfig as TECurveConfig>::mul_projective(
			base, scalar,
		);
}
