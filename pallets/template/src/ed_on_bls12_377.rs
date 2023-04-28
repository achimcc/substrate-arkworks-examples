use ark_std::io::Error;
pub use sp_ark_ed_on_bls12_377::curves::{
	EdwardsAffine as EdwardsAffineOptimized, EdwardsProjective as EdwardsProjectiveOptimized,
};
use sp_ark_models::TECurveConfig;

pub fn do_msm(
	bases: &[ark_ec::twisted_edwards::Affine<ark_ed_on_bls12_377::EdwardsConfig>],
	scalars: &[<ark_ed_on_bls12_377::EdwardsConfig as ark_ec::CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out =
		<ark_ed_on_bls12_377::EdwardsConfig as ark_ec::models::twisted_edwards::TECurveConfig>::msm(
			bases, scalars,
		);
	Ok(())
}

pub fn do_msm_optimized(
	bases: &[EdwardsAffineOptimized],
	scalars: &[<sp_ark_ed_on_bls12_377::EdwardsConfig<sp_ark_ed_on_bls12_377::curves::Host> as ark_ec::CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out = <sp_ark_ed_on_bls12_377::EdwardsConfig<sp_ark_ed_on_bls12_377::curves::Host> as TECurveConfig>::msm(
		bases, scalars,
	);
	Ok(())
}

pub fn do_mul_affine(
	base: &ark_ed_on_bls12_377::EdwardsAffine,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out =
		<ark_ed_on_bls12_377::EdwardsConfig as ark_ec::models::twisted_edwards::TECurveConfig>::mul_affine(
			base,
			scalar,
		);
	Ok(())
}

pub fn do_mul_affine_optimized(base: &EdwardsAffineOptimized, scalar: &[u64]) -> Result<(), Error> {
	let _out =
		<sp_ark_ed_on_bls12_377::EdwardsConfig<sp_ark_ed_on_bls12_377::curves::Host> as TECurveConfig>::mul_affine(
			base, scalar,
		);
	Ok(())
}

pub fn do_mul_projective(
	base: &ark_ed_on_bls12_377::EdwardsProjective,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out = <ark_ed_on_bls12_377::EdwardsConfig as TECurveConfig>::mul_projective(base, scalar);
	Ok(())
}

pub fn do_mul_projective_optimized(
	base: &EdwardsProjectiveOptimized,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out =
		<sp_ark_ed_on_bls12_377::EdwardsConfig<sp_ark_ed_on_bls12_377::curves::Host> as TECurveConfig>::mul_projective(
			base, scalar,
		);
	Ok(())
}
