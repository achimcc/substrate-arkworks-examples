use crate::utils::ScalarFieldFor;
use ark_ec::twisted_edwards::TECurveConfig;
use ark_ed_on_bls12_377::EdwardsAffine;
pub use sp_ark_ed_on_bls12_377::curves::{
	EdwardsAffine as EdwardsAffineOpt, EdwardsProjective as EdwardsProjectiveOpt,
};

pub fn msm(bases: &[EdwardsAffine], scalars: &[ScalarFieldFor<EdwardsAffine>]) {
	let _out = <ark_ed_on_bls12_377::EdwardsConfig as TECurveConfig>::msm(bases, scalars);
}

pub fn msm_opt(bases: &[EdwardsAffineOpt], scalars: &[ScalarFieldFor<EdwardsAffineOpt>]) {
	let _out = <sp_ark_ed_on_bls12_377::EdwardsConfig as TECurveConfig>::msm(bases, scalars);
}

pub fn mul_affine(base: &ark_ed_on_bls12_377::EdwardsAffine, scalar: &[u64]) {
	let _out = <ark_ed_on_bls12_377::EdwardsConfig as TECurveConfig>::mul_affine(base, scalar);
}

pub fn mul_affine_opt(base: &EdwardsAffineOpt, scalar: &[u64]) {
	let _out = <sp_ark_ed_on_bls12_377::EdwardsConfig as TECurveConfig>::mul_affine(base, scalar);
}

pub fn mul_projective(base: &ark_ed_on_bls12_377::EdwardsProjective, scalar: &[u64]) {
	let _out = <ark_ed_on_bls12_377::EdwardsConfig as TECurveConfig>::mul_projective(base, scalar);
}

pub fn mul_projective_opt(base: &EdwardsProjectiveOpt, scalar: &[u64]) {
	let _out =
		<sp_ark_ed_on_bls12_377::EdwardsConfig as TECurveConfig>::mul_projective(base, scalar);
}
