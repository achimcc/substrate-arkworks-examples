use ark_std::{io::Error, vec::Vec};
use sp_ark_ed_on_bls12_381_bandersnatch::{
	BandersnatchConfig as BandersnatchConfig_Host, HostFunctions,
};
use sp_ark_models::short_weierstrass::SWCurveConfig;

pub struct HostEdOnBls12_381Bandersnatch {}

impl HostFunctions for HostEdOnBls12_381Bandersnatch {
	fn ed_on_bls12_381_bandersnatch_te_msm(
		bases: Vec<u8>,
		scalars: Vec<u8>,
	) -> Result<Vec<u8>, ()> {
		sp_io::elliptic_curves::ed_on_bls12_381_bandersnatch_te_msm(bases, scalars)
	}
	fn ed_on_bls12_381_bandersnatch_sw_msm(
		bases: Vec<u8>,
		scalars: Vec<u8>,
	) -> Result<Vec<u8>, ()> {
		sp_io::elliptic_curves::ed_on_bls12_381_bandersnatch_sw_msm(bases, scalars)
	}
	fn ed_on_bls12_381_bandersnatch_te_mul_projective(
		base: Vec<u8>,
		scalar: Vec<u8>,
	) -> Result<Vec<u8>, ()> {
		sp_io::elliptic_curves::ed_on_bls12_381_bandersnatch_te_mul_projective(base, scalar)
	}
	fn ed_on_bls12_381_bandersnatch_sw_mul_projective(
		base: Vec<u8>,
		scalar: Vec<u8>,
	) -> Result<Vec<u8>, ()> {
		sp_io::elliptic_curves::ed_on_bls12_381_bandersnatch_sw_mul_projective(base, scalar)
	}
}

pub type SWAffineOptimized = sp_ark_models::short_weierstrass::Affine<
	sp_ark_ed_on_bls12_381_bandersnatch::SWConfig<HostEdOnBls12_381Bandersnatch>,
>;
pub type SWProjectiveOptimized = sp_ark_models::short_weierstrass::Projective<
	sp_ark_ed_on_bls12_381_bandersnatch::SWConfig<HostEdOnBls12_381Bandersnatch>,
>;
pub type EdwardsAffineOptimized = sp_ark_models::twisted_edwards::Affine<
	sp_ark_ed_on_bls12_381_bandersnatch::EdwardsConfig<HostEdOnBls12_381Bandersnatch>,
>;
pub type EdwardsProjectiveOptimized = sp_ark_models::twisted_edwards::Projective<
	sp_ark_ed_on_bls12_381_bandersnatch::EdwardsConfig<HostEdOnBls12_381Bandersnatch>,
>;
pub type BandersnatchConfig = BandersnatchConfig_Host<HostEdOnBls12_381Bandersnatch>;

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
	scalars: &[<sp_ark_ed_on_bls12_381_bandersnatch::SWConfig<HostEdOnBls12_381Bandersnatch> as sp_ark_models::CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out = <sp_ark_ed_on_bls12_381_bandersnatch::EdwardsConfig<HostEdOnBls12_381Bandersnatch> as SWCurveConfig>::msm(
		bases, scalars,
	);
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
	scalars: &[<sp_ark_ed_on_bls12_381_bandersnatch::EdwardsConfig<
		HostEdOnBls12_381Bandersnatch,
	> as ark_ec::CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out = <sp_ark_ed_on_bls12_381_bandersnatch::BandersnatchConfig<
		HostEdOnBls12_381Bandersnatch,
	> as sp_ark_models::twisted_edwards::TECurveConfig>::msm(bases, scalars);
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
		<sp_ark_ed_on_bls12_381_bandersnatch::EdwardsConfig<HostEdOnBls12_381Bandersnatch> as SWCurveConfig>::mul_affine(
			base, scalar,
		);
	Ok(())
}

pub fn do_mul_affine_te_optimized(
	base: &EdwardsAffineOptimized,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out =
		<sp_ark_ed_on_bls12_381_bandersnatch::EdwardsConfig<HostEdOnBls12_381Bandersnatch> as sp_ark_models::twisted_edwards::TECurveConfig>::mul_affine(
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
	let _out =
		<sp_ark_ed_on_bls12_381_bandersnatch::EdwardsConfig<HostEdOnBls12_381Bandersnatch> as SWCurveConfig>::mul_projective(
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
	<sp_ark_ed_on_bls12_381_bandersnatch::EdwardsConfig<HostEdOnBls12_381Bandersnatch>  as sp_ark_models::twisted_edwards::TECurveConfig>::mul_projective(
			base,
			scalar,
		);
	Ok(())
}
