use ark_std::{io::Error, vec::Vec};
use sp_ark_ed_on_bls12_377::HostFunctions as EdwardBls12_377HostFunctions;
use sp_ark_models::TECurveConfig;

pub struct HostEdOnBls12_377 {}

impl EdwardBls12_377HostFunctions for HostEdOnBls12_377 {
	fn ed_on_bls12_377_msm(bases: Vec<Vec<u8>>, scalars: Vec<Vec<u8>>) -> Vec<u8> {
		sp_io::elliptic_curves::ed_on_bls12_377_msm(bases, scalars)
	}
	fn ed_on_bls12_377_mul_affine(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::ed_on_bls12_377_mul_affine(base, scalar)
	}
	fn ed_on_bls12_377_mul_projective(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::ed_on_bls12_377_mul_projective(base, scalar)
	}
}

pub type EdwardsAffineOptimized =
	ark_ec::twisted_edwards::Affine<ark_ed_on_bls12_377::EdwardsConfig>;
pub type EdwardsProjectiveOptimized = sp_ark_ed_on_bls12_377::EdwardsProjective<HostEdOnBls12_377>;

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
	scalars: &[<sp_ark_ed_on_bls12_377::EdwardsConfig<HostEdOnBls12_377> as ark_ec::CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out = <sp_ark_ed_on_bls12_377::EdwardsConfig<HostEdOnBls12_377> as TECurveConfig>::msm(
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
		<sp_ark_ed_on_bls12_377::EdwardsConfig<HostEdOnBls12_377> as TECurveConfig>::mul_affine(
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
		<sp_ark_ed_on_bls12_377::EdwardsConfig<HostEdOnBls12_377> as TECurveConfig>::mul_projective(
			base, scalar,
		);
	Ok(())
}
