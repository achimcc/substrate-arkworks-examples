use ark_std::io::Error;
use sp_ark_models::short_weierstrass::SWCurveConfig;
pub use sp_ed_on_bls12_381_bandersnatch::curves::{
	BandersnatchConfig, EdwardsAffine as EdwardsAffineOptimized,
	EdwardsProjective as EdwardsProjectiveOptimized, SWAffine as SWAffineOptimized,
	SWProjective as SWProjectiveOptimized,
};

pub fn do_msm_sw(
	bases: &[sp_ark_models::short_weierstrass::Affine<
		ark_ed_on_bls12_381_bandersnatch::SWConfig,
	>],
	scalars: &[<ark_ed_on_bls12_381_bandersnatch::SWConfig as ark_ec::CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out =
		<ark_ed_on_bls12_381_bandersnatch::EdwardsConfig as SWCurveConfig>::msm(bases, scalars);
	Ok(())
}

pub fn do_msm_sw_optimized(
	bases: &[SWAffineOptimized],
	scalars: &[<sp_ed_on_bls12_381_bandersnatch::SWConfig as sp_ark_models::CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out =
		<sp_ed_on_bls12_381_bandersnatch::EdwardsConfig as SWCurveConfig>::msm(bases, scalars);
	Ok(())
}

pub fn do_msm_te(
	bases: &[ark_ed_on_bls12_381_bandersnatch::EdwardsAffine],
	scalars: &[<ark_ed_on_bls12_381_bandersnatch::EdwardsConfig as ark_ec::CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out = <ark_ed_on_bls12_381_bandersnatch::BandersnatchConfig as ark_ec::twisted_edwards::TECurveConfig>::msm(
		bases, scalars,
	);
	Ok(())
}

pub fn do_msm_te_optimized(
	bases: &[EdwardsAffineOptimized],
	scalars: &[<sp_ed_on_bls12_381_bandersnatch::EdwardsConfig as ark_ec::CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out = <sp_ed_on_bls12_381_bandersnatch::BandersnatchConfig as sp_ark_models::twisted_edwards::TECurveConfig>::msm(bases, scalars);
	Ok(())
}

pub fn do_mul_affine_sw(
	base: &ark_ed_on_bls12_381_bandersnatch::SWAffine,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out = <ark_ed_on_bls12_381_bandersnatch::EdwardsConfig as SWCurveConfig>::mul_affine(
		base, scalar,
	);
	Ok(())
}

pub fn do_mul_affine_te(
	base: &ark_ed_on_bls12_381_bandersnatch::EdwardsAffine,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out =
		<ark_ed_on_bls12_381_bandersnatch::EdwardsConfig as sp_ark_models::twisted_edwards::TECurveConfig>::mul_affine(
			base,
			scalar,
		);
	Ok(())
}

pub fn do_mul_affine_sw_optimized(base: &SWAffineOptimized, scalar: &[u64]) -> Result<(), Error> {
	let _out =
		<sp_ed_on_bls12_381_bandersnatch::EdwardsConfig as SWCurveConfig>::mul_affine(base, scalar);
	Ok(())
}

pub fn do_mul_affine_te_optimized(
	base: &EdwardsAffineOptimized,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out =
		<sp_ed_on_bls12_381_bandersnatch::EdwardsConfig as sp_ark_models::twisted_edwards::TECurveConfig>::mul_affine(
			base,
			scalar,
		);
	Ok(())
}

pub fn do_mul_projective_sw(
	base: &ark_ec::short_weierstrass::Projective<ark_ed_on_bls12_381_bandersnatch::SWConfig>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out =
		<ark_ed_on_bls12_381_bandersnatch::EdwardsConfig as sp_ark_models::short_weierstrass::SWCurveConfig>::mul_projective(
			base,
			scalar,
		);
	Ok(())
}

pub fn do_mul_projective_sw_optimized(
	base: &SWProjectiveOptimized,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out = <sp_ed_on_bls12_381_bandersnatch::EdwardsConfig as SWCurveConfig>::mul_projective(
		base, scalar,
	);
	Ok(())
}

pub fn do_mul_projective_te(
	base: &ark_ec::twisted_edwards::Projective<
		ark_ed_on_bls12_381_bandersnatch::BandersnatchConfig,
	>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out = <ark_ed_on_bls12_381_bandersnatch::EdwardsConfig as ark_ec::twisted_edwards::TECurveConfig>::mul_projective(
		base,
		scalar,
	);
	Ok(())
}

pub fn do_mul_projective_te_optimized(
	base: &EdwardsProjectiveOptimized,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out =
	<sp_ed_on_bls12_381_bandersnatch::EdwardsConfig as sp_ark_models::twisted_edwards::TECurveConfig>::mul_projective(
			base,
			scalar,
		);
	Ok(())
}
