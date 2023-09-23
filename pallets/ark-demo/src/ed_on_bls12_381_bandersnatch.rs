use crate::utils::ScalarFieldFor;
use ark_ed_on_bls12_381_bandersnatch::{EdwardsAffine, EdwardsProjective, SWAffine, SWProjective};
use sp_ark_models::short_weierstrass::SWCurveConfig;
use sp_ed_on_bls12_381_bandersnatch::curves::{
	EdwardsAffine as EdwardsAffineOptimized, EdwardsProjective as EdwardsProjectiveOptimized,
	SWAffine as SWAffineOptimized, SWProjective as SWProjectiveOptimized,
};

#[inline]
pub fn msm_sw(
	bases: &[SWAffine],
	scalars: &[<ark_ed_on_bls12_381_bandersnatch::SWConfig as ark_ec::CurveConfig>::ScalarField],
) {
	let _out =
		<ark_ed_on_bls12_381_bandersnatch::EdwardsConfig as SWCurveConfig>::msm(bases, scalars);
}

#[inline]
pub fn msm_sw_opt(bases: &[SWAffineOptimized], scalars: &[ScalarFieldFor<SWAffineOptimized>]) {
	let _out =
		<sp_ed_on_bls12_381_bandersnatch::EdwardsConfig as SWCurveConfig>::msm(bases, scalars);
}

#[inline]
pub fn msm_te(
	bases: &[EdwardsAffine],
	scalars: &[<ark_ed_on_bls12_381_bandersnatch::EdwardsConfig as ark_ec::CurveConfig>::ScalarField],
) {
	let _out = <ark_ed_on_bls12_381_bandersnatch::BandersnatchConfig as ark_ec::twisted_edwards::TECurveConfig>::msm(
		bases, scalars,
	);
}

#[inline]
pub fn msm_te_opt(
	bases: &[EdwardsAffineOptimized],
	scalars: &[ScalarFieldFor<EdwardsAffineOptimized>],
) {
	let _out = <sp_ed_on_bls12_381_bandersnatch::BandersnatchConfig as sp_ark_models::twisted_edwards::TECurveConfig>::msm(bases, scalars);
}

#[inline]
pub fn mul_affine_sw(base: &SWAffine, scalar: &[u64]) {
	let _out = <ark_ed_on_bls12_381_bandersnatch::EdwardsConfig as SWCurveConfig>::mul_affine(
		base, scalar,
	);
}

#[inline]
pub fn mul_affine_te(base: &ark_ed_on_bls12_381_bandersnatch::EdwardsAffine, scalar: &[u64]) {
	let _out =
		<ark_ed_on_bls12_381_bandersnatch::EdwardsConfig as sp_ark_models::twisted_edwards::TECurveConfig>::mul_affine(
			base,
			scalar,
		);
}

#[inline]
pub fn mul_affine_sw_opt(base: &SWAffineOptimized, scalar: &[u64]) {
	let _out =
		<sp_ed_on_bls12_381_bandersnatch::EdwardsConfig as SWCurveConfig>::mul_affine(base, scalar);
}

#[inline]
pub fn mul_affine_te_opt(base: &EdwardsAffineOptimized, scalar: &[u64]) {
	let _out =
		<sp_ed_on_bls12_381_bandersnatch::EdwardsConfig as sp_ark_models::twisted_edwards::TECurveConfig>::mul_affine(
			base,
			scalar,
		);
}

#[inline]
pub fn mul_projective_sw(
	base: &ark_ec::short_weierstrass::Projective<ark_ed_on_bls12_381_bandersnatch::SWConfig>,
	scalar: &[u64],
) {
	let _out =
		<ark_ed_on_bls12_381_bandersnatch::EdwardsConfig as sp_ark_models::short_weierstrass::SWCurveConfig>::mul_projective(
			base,
			scalar,
		);
}

#[inline]
pub fn mul_projective_sw_opt(base: &SWProjectiveOptimized, scalar: &[u64]) {
	let _out = <sp_ed_on_bls12_381_bandersnatch::EdwardsConfig as SWCurveConfig>::mul_projective(
		base, scalar,
	);
}

#[inline]
pub fn mul_projective_te(
	base: &ark_ec::twisted_edwards::Projective<
		ark_ed_on_bls12_381_bandersnatch::BandersnatchConfig,
	>,
	scalar: &[u64],
) {
	let _out = <ark_ed_on_bls12_381_bandersnatch::EdwardsConfig as ark_ec::twisted_edwards::TECurveConfig>::mul_projective(
		base,
		scalar,
	);
}

#[inline]
pub fn mul_projective_te_opt(base: &EdwardsProjectiveOptimized, scalar: &[u64]) {
	let _out =
	<sp_ed_on_bls12_381_bandersnatch::EdwardsConfig as sp_ark_models::twisted_edwards::TECurveConfig>::mul_projective(
			base,
			scalar,
		);
}
