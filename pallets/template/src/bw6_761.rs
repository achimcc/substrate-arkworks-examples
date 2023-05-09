use ark_std::io::Error;
pub use sp_ark_models::{pairing::Pairing, short_weierstrass::SWCurveConfig, AffineRepr, Group};
pub use sp_bw6_761::curves::{
	g1::{G1Affine as G1AffineOptimized, G1Projective as G1ProjectiveOptimized},
	g2::{G2Affine as G2AffineOptimized, G2Projective as G2ProjectiveOptimized},
	BW6_761 as BW6_761Optimized,
};

pub fn do_msm_g1(
	bases: &[ark_ec::short_weierstrass::Affine<ark_bw6_761::g1::Config>],
	scalars: &[<ark_bw6_761::g1::Config as ark_ec::CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out = <ark_bw6_761::g1::Config as SWCurveConfig>::msm(bases, scalars);

	Ok(())
}

pub fn do_msm_g1_optimized(
	bases: &[sp_bw6_761::g1::G1Affine],
	scalars: &[<sp_bw6_761::g1::Config as sp_ark_models::CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out = <sp_bw6_761::g1::Config as SWCurveConfig>::msm(bases, scalars);
	Ok(())
}

pub fn do_msm_g2(
	bases: &[ark_ec::models::short_weierstrass::Affine<ark_bw6_761::g2::Config>],
	scalars: &[<ark_bw6_761::g2::Config as ark_ec::CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out = <ark_bw6_761::g2::Config as SWCurveConfig>::msm(bases, scalars);
	Ok(())
}

pub fn do_msm_g2_optimized(
	bases: &[sp_ark_models::short_weierstrass::Affine<sp_bw6_761::g2::Config>],
	scalars: &[<sp_bw6_761::g2::Config as sp_ark_models::CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out = <sp_bw6_761::g2::Config as SWCurveConfig>::msm(bases, scalars);
	Ok(())
}

pub fn do_mul_affine_g1(
	base: &ark_ec::short_weierstrass::Affine<ark_bw6_761::g1::Config>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out = <ark_bw6_761::g1::Config as SWCurveConfig>::mul_affine(base, scalar);
	Ok(())
}

pub fn do_mul_affine_g1_optimized(
	base: &sp_bw6_761::g1::G1Affine,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out = <sp_bw6_761::g1::Config as SWCurveConfig>::mul_affine(base, scalar);
	Ok(())
}

pub fn do_mul_projective_g1(
	base: &ark_ec::short_weierstrass::Projective<ark_bw6_761::g1::Config>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out = <ark_bw6_761::g1::Config as SWCurveConfig>::mul_projective(base, scalar);
	Ok(())
}

pub fn do_mul_projective_g1_optimized(
	base: &sp_bw6_761::g1::G1Projective,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out = <sp_bw6_761::g1::Config as SWCurveConfig>::mul_projective(base, scalar);
	Ok(())
}

pub fn do_mul_affine_g2(
	base: &ark_ec::short_weierstrass::Affine<ark_bw6_761::g2::Config>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out = <ark_bw6_761::g2::Config as SWCurveConfig>::mul_affine(base, scalar);
	Ok(())
}

pub fn do_mul_affine_g2_optimized(
	base: &sp_ark_models::short_weierstrass::Affine<sp_bw6_761::g2::Config>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out = <sp_bw6_761::g2::Config as SWCurveConfig>::mul_affine(base, scalar);
	Ok(())
}

pub fn do_mul_projective_g2(
	base: &ark_ec::short_weierstrass::Projective<ark_bw6_761::g2::Config>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out = <ark_bw6_761::g2::Config as SWCurveConfig>::mul_projective(base, scalar);
	Ok(())
}

pub fn do_mul_projective_g2_optimized(
	base: &sp_ark_models::short_weierstrass::Projective<sp_bw6_761::g2::Config>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out = <sp_bw6_761::g2::Config as SWCurveConfig>::mul_projective(base, scalar);
	Ok(())
}

pub fn do_pairing(a: ark_bw6_761::G1Affine, b: ark_bw6_761::G2Affine) -> Result<(), Error> {
	let _out = ark_bw6_761::BW6_761::multi_pairing([a], [b]);
	Ok(())
}

pub fn do_pairing_optimized(a: G1AffineOptimized, b: G2AffineOptimized) -> Result<(), Error> {
	let _out = BW6_761Optimized::multi_pairing([a], [b]);
	Ok(())
}
