use ark_bls12_377::{Bls12_377, G1Affine, G2Affine};
use ark_std::io::Error;
pub use sp_ark_models::{pairing::Pairing, short_weierstrass::SWCurveConfig, AffineRepr, Group};
pub use sp_bls12_377::{
	curves::{
		g1::{G1Affine as G1AffineOpt, G1Projective as G1ProjectiveOpt},
		g2::{G2Affine as G2AffineOpt, G2Projective as G2ProjectiveOpt},
	},
	Bls12_377 as Bls12_377Opt,
};

pub fn do_pairing(a: G1Affine, b: G2Affine) {
	let _ = Bls12_377::multi_pairing([a], [b]);
}

pub fn do_pairing_opt(a: G1AffineOpt, b: G2AffineOpt) {
	let _ = Bls12_377Opt::multi_pairing([a], [b]);
}

pub fn do_msm_g1(
	bases: &[ark_ec::short_weierstrass::Affine<ark_bls12_377::g1::Config>],
	scalars: &[<ark_bls12_377::g1::Config as ark_ec::CurveConfig>::ScalarField],
) {
	let _ = <ark_bls12_377::g1::Config as SWCurveConfig>::msm(bases, scalars).unwrap();
}

pub fn do_msm_g1_opt(
	bases: &[sp_ark_models::short_weierstrass::Affine<sp_bls12_377::g1::Config>],
	scalars: &[<sp_bls12_377::g1::Config as sp_ark_models::CurveConfig>::ScalarField],
) {
	let _ = <sp_bls12_377::g1::Config as sp_ark_models::short_weierstrass::SWCurveConfig>::msm(
		bases, scalars,
	);
}

pub fn do_msm_g2(
	bases: &[ark_ec::short_weierstrass::Affine<ark_bls12_377::g2::Config>],
	scalars: &[<ark_bls12_377::g2::Config as ark_ec::CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out = <ark_bls12_377::g2::Config as SWCurveConfig>::msm(bases, scalars);
	Ok(())
}

pub fn do_msm_g2_optimized(
	bases: &[sp_ark_models::short_weierstrass::Affine<sp_bls12_377::g2::Config>],
	scalars: &[<sp_bls12_377::g2::Config as ark_ec::CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out = <sp_bls12_377::g2::Config as SWCurveConfig>::msm(bases, scalars);
	Ok(())
}

pub fn do_mul_affine_g1(
	base: &ark_ec::short_weierstrass::Affine<ark_bls12_377::g1::Config>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out = <ark_bls12_377::g1::Config as ark_ec::short_weierstrass::SWCurveConfig>::mul_affine(
		base, scalar,
	);
	Ok(())
}

pub fn do_mul_affine_g1_optimized(
	base: &sp_ark_models::short_weierstrass::Affine<sp_bls12_377::g1::Config>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out =
		<sp_bls12_377::g1::Config as sp_ark_models::short_weierstrass::SWCurveConfig>::mul_affine(
			base, scalar,
		);
	Ok(())
}

pub fn do_mul_projective_g1(
	base: &ark_ec::short_weierstrass::Projective<ark_bls12_377::g1::Config>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out =
		<ark_bls12_377::g1::Config as ark_ec::short_weierstrass::SWCurveConfig>::mul_projective(
			base, scalar,
		);
	Ok(())
}

pub fn do_mul_projective_g1_optimized(
	base: &sp_ark_models::short_weierstrass::Projective<sp_bls12_377::g1::Config>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out = <sp_bls12_377::g1::Config as sp_ark_models::short_weierstrass::SWCurveConfig>::mul_projective(
		base,
		scalar,
	);
	Ok(())
}

pub fn do_mul_affine_g2(
	base: &ark_ec::short_weierstrass::Affine<ark_bls12_377::g2::Config>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out = <ark_bls12_377::g2::Config as ark_ec::short_weierstrass::SWCurveConfig>::mul_affine(
		base, scalar,
	);
	Ok(())
}

pub fn do_mul_affine_g2_optimized(
	base: &sp_ark_models::short_weierstrass::Affine<sp_bls12_377::g2::Config>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out =
		<sp_bls12_377::g2::Config as sp_ark_models::short_weierstrass::SWCurveConfig>::mul_affine(
			base, scalar,
		);
	Ok(())
}

pub fn do_mul_projective_g2(
	base: &ark_ec::short_weierstrass::Projective<ark_bls12_377::g2::Config>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out =
		<ark_bls12_377::g2::Config as ark_ec::short_weierstrass::SWCurveConfig>::mul_projective(
			base, scalar,
		);
	Ok(())
}

pub fn do_mul_projective_g2_optimized(
	base: &sp_ark_models::short_weierstrass::Projective<sp_bls12_377::g2::Config>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out = <sp_bls12_377::g2::Config as sp_ark_models::short_weierstrass::SWCurveConfig>::mul_projective(
		base,
		scalar,
	);
	Ok(())
}
