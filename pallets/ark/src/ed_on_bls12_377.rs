use crate::utils::ScalarFieldFor;
use ark_ed_on_bls12_377::EdwardsAffine;
use sp_ark_models::TECurveConfig;
pub use sp_ed_on_bls12_377::curves::{
	EdwardsAffine as EdwardsAffineOptimized, EdwardsProjective as EdwardsProjectiveOptimized,
};

pub fn msm(bases: &[EdwardsAffine], scalars: &[ScalarFieldFor<EdwardsAffine>]) {
	let _out =
		<ark_ed_on_bls12_377::EdwardsConfig as ark_ec::models::twisted_edwards::TECurveConfig>::msm(
			bases, scalars,
		);
}

pub fn msm_opt(
	bases: &[EdwardsAffineOptimized],
	scalars: &[<sp_ed_on_bls12_377::EdwardsConfig as ark_ec::CurveConfig>::ScalarField],
) {
	let _out = <sp_ed_on_bls12_377::EdwardsConfig as TECurveConfig>::msm(bases, scalars);
}

pub fn mul_affine(base: &ark_ed_on_bls12_377::EdwardsAffine, scalar: &[u64]) {
	let _out =
		<ark_ed_on_bls12_377::EdwardsConfig as ark_ec::models::twisted_edwards::TECurveConfig>::mul_affine(
			base,
			scalar,
		);
}

pub fn mul_affine_opt(base: &EdwardsAffineOptimized, scalar: &[u64]) {
	let _out = <sp_ed_on_bls12_377::EdwardsConfig as TECurveConfig>::mul_affine(base, scalar);
}

pub fn mul_projective(base: &ark_ed_on_bls12_377::EdwardsProjective, scalar: &[u64]) {
	let _out = <ark_ed_on_bls12_377::EdwardsConfig as TECurveConfig>::mul_projective(base, scalar);
}

pub fn mul_projective_opt(base: &EdwardsProjectiveOptimized, scalar: &[u64]) {
	let _out = <sp_ed_on_bls12_377::EdwardsConfig as TECurveConfig>::mul_projective(base, scalar);
}
