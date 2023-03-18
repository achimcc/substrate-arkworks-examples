use ark_std::{io::Error, vec::Vec};
use sp_ark_ed_on_bls12_381::HostFunctions as EdwardBls12_381HostFunctions;
use sp_ark_models::{short_weierstrass::SWCurveConfig, AffineRepr, Group};

pub struct HostEdOnBls12_381 {}

impl EdwardBls12_381HostFunctions for HostEdOnBls12_381 {
	fn ed_on_bls12_381_te_msm(bases: Vec<u8>, scalars: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::ed_on_bls12_381_te_msm(bases, scalars)
	}
	fn ed_on_bls12_381_sw_msm(bases: Vec<u8>, scalars: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::ed_on_bls12_381_sw_msm(bases, scalars)
	}
}

pub fn do_msm_sw(
	bases: &[sp_ark_models::short_weierstrass::Affine<ark_ed_on_bls12_381::SWConfig>],
	scalars: &[<ark_ed_on_bls12_381::SWConfig as ark_ec::CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out = <ark_ed_on_bls12_381::EdwardsConfig as SWCurveConfig>::msm(bases, scalars);
	Ok(())
}

pub fn do_msm_sw_optimized(
	bases: &[sp_ark_models::short_weierstrass::Affine<
		sp_ark_ed_on_bls12_381::SWConfig<HostEdOnBls12_381>,
	>],
	scalars: &[<sp_ark_ed_on_bls12_381::SWConfig<HostEdOnBls12_381> as sp_ark_models::CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out = <sp_ark_ed_on_bls12_381::EdwardsConfig<HostEdOnBls12_381> as SWCurveConfig>::msm(
		bases, scalars,
	);
	Ok(())
}

pub fn do_msm_te(
	bases: &[ark_ec::twisted_edwards::Affine<ark_ed_on_bls12_381::JubjubConfig>],
	scalars: &[<ark_ed_on_bls12_381::EdwardsConfig as ark_ec::CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out = <ark_ed_on_bls12_381::JubjubConfig as ark_ec::twisted_edwards::TECurveConfig>::msm(
		bases, scalars,
	);
	Ok(())
}

pub fn do_msm_te_optimized(
	bases: &[sp_ark_models::twisted_edwards::Affine<
		sp_ark_ed_on_bls12_381::EdwardsConfig<HostEdOnBls12_381>,
	>],
	scalars: &[<sp_ark_ed_on_bls12_381::EdwardsConfig<HostEdOnBls12_381> as ark_ec::CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out = <sp_ark_ed_on_bls12_381::JubjubConfig<HostEdOnBls12_381> as sp_ark_models::twisted_edwards::TECurveConfig>::msm(
		bases,
		scalars,
	);
	Ok(())
}

pub fn do_mul_affine_sw() -> Result<(), Error> {
	let _out = <ark_ed_on_bls12_381::EdwardsConfig as SWCurveConfig>::mul_affine(
		&ark_ed_on_bls12_381::SWAffine::generator(),
		&[2u64],
	);
	Ok(())
}

pub fn do_mul_affine_te() -> Result<(), Error> {
	let _out =
		<ark_ed_on_bls12_381::EdwardsConfig as sp_ark_models::short_weierstrass::SWCurveConfig>::mul_affine(
			&ark_ed_on_bls12_381::SWAffine::generator(),
			&[2u64],
		);
	Ok(())
}

pub fn do_mul_affine_sw_optimized() -> Result<(), Error> {
	let _out =
		<sp_ark_ed_on_bls12_381::EdwardsConfig<HostEdOnBls12_381> as SWCurveConfig>::mul_affine(
			&sp_ark_ed_on_bls12_381::SWAffine::<HostEdOnBls12_381>::generator(),
			&[2u64],
		);
	Ok(())
}

pub fn do_mul_affine_te_optimized() -> Result<(), Error> {
	let _out =
		<sp_ark_ed_on_bls12_381::EdwardsConfig<HostEdOnBls12_381> as sp_ark_models::short_weierstrass::SWCurveConfig>::mul_affine(
			&sp_ark_ed_on_bls12_381::SWAffine::<HostEdOnBls12_381>::generator(),
			&[2u64],
		);
	Ok(())
}

pub fn do_mul_projective_sw() -> Result<(), Error> {
	let _out =
		<ark_ed_on_bls12_381::EdwardsConfig as sp_ark_models::short_weierstrass::SWCurveConfig>::mul_projective(
			&ark_ed_on_bls12_381::SWProjective::generator(),
			&[2u64],
		);
	Ok(())
}

pub fn do_mul_projective_sw_optimized() -> Result<(), Error> {
	let _out =
		<sp_ark_ed_on_bls12_381::EdwardsConfig<HostEdOnBls12_381> as SWCurveConfig>::mul_projective(
			&sp_ark_ed_on_bls12_381::SWProjective::<HostEdOnBls12_381>::generator(),
			&[2u64],
		);
	Ok(())
}

pub fn do_mul_projective_te() -> Result<(), Error> {
	let _out = <ark_ed_on_bls12_381::EdwardsConfig as ark_ec::twisted_edwards::TECurveConfig>::mul_projective(
		&ark_ed_on_bls12_381::EdwardsProjective::generator(),
		&[2u64],
	);
	Ok(())
}

pub fn do_mul_projective_te_optimized() -> Result<(), Error> {
	let _out =
	<sp_ark_ed_on_bls12_381::EdwardsConfig<HostEdOnBls12_381>  as sp_ark_models::twisted_edwards::TECurveConfig>::mul_projective(
			&sp_ark_ed_on_bls12_381::EdwardsProjective::<HostEdOnBls12_381>::generator(),
			&[2u64],
		);
	Ok(())
}
