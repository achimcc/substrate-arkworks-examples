use ark_std::{io::Error, vec::Vec};
pub use sp_ark_bw6_761::{
	G1Affine as G1AffineOptimized_Host, G1Projective as G1ProjectiveOptimized_Host,
	G2Affine as G2AffineOptimized_Host, G2Projective as G2ProjectiveOptimized_Host,
	HostFunctions as BW6_761HostFunctions, BW6_761 as BW6_761_Host,
};
pub use sp_ark_models::{pairing::Pairing, short_weierstrass::SWCurveConfig, AffineRepr, Group};

pub struct HostBW6_761 {}

impl BW6_761HostFunctions for HostBW6_761 {
	fn bw6_761_multi_miller_loop(a: Vec<Vec<u8>>, b: Vec<Vec<u8>>) -> Result<Vec<u8>, ()> {
		sp_io::elliptic_curves::bw6_761_multi_miller_loop(a, b)
	}
	fn bw6_761_final_exponentiation(f12: Vec<u8>) -> Result<Vec<u8>, ()> {
		sp_io::elliptic_curves::bw6_761_final_exponentiation(f12)
	}
	fn bw6_761_msm_g1(bases: Vec<Vec<u8>>, bigints: Vec<Vec<u8>>) -> Vec<u8> {
		sp_io::elliptic_curves::bw6_761_msm_g1(bases, bigints)
	}
	fn bw6_761_msm_g2(bases: Vec<Vec<u8>>, bigints: Vec<Vec<u8>>) -> Vec<u8> {
		sp_io::elliptic_curves::bw6_761_msm_g2(bases, bigints)
	}
	fn bw6_761_mul_projective_g1(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::bw6_761_mul_projective_g1(base, scalar)
	}
	fn bw6_761_mul_projective_g2(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::bw6_761_mul_projective_g2(base, scalar)
	}
	fn bw6_761_mul_affine_g1(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::bw6_761_mul_affine_g1(base, scalar)
	}
	fn bw6_761_mul_affine_g2(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::bw6_761_mul_affine_g2(base, scalar)
	}
}

pub type BW6_761Optimized = BW6_761_Host<HostBW6_761>;
pub type G1AffineOptimized = G1AffineOptimized_Host<HostBW6_761>;
pub type G2AffineOptimized = G2AffineOptimized_Host<HostBW6_761>;
pub type G1ProjectiveOptimized = G1ProjectiveOptimized_Host<HostBW6_761>;
pub type G2ProjectiveOptimized = G2ProjectiveOptimized_Host<HostBW6_761>;

pub fn do_msm_g1(
	bases: &[ark_ec::short_weierstrass::Affine<ark_bw6_761::g1::Config>],
	scalars: &[<ark_bw6_761::g1::Config as ark_ec::CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out = <ark_bw6_761::g1::Config as SWCurveConfig>::msm(bases, scalars);

	Ok(())
}

pub fn do_msm_g1_optimized(
	bases: &[sp_ark_models::short_weierstrass::Affine<sp_ark_bw6_761::g1::Config<HostBW6_761>>],
	scalars: &[<sp_ark_bw6_761::g1::Config<HostBW6_761> as sp_ark_models::CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out = <sp_ark_bw6_761::g1::Config<HostBW6_761> as SWCurveConfig>::msm(bases, scalars);
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
	bases: &[sp_ark_models::short_weierstrass::Affine<sp_ark_bw6_761::g2::Config<HostBW6_761>>],
	scalars: &[<sp_ark_bw6_761::g2::Config<HostBW6_761> as sp_ark_models::CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out = <sp_ark_bw6_761::g2::Config<HostBW6_761> as SWCurveConfig>::msm(bases, scalars);
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
	base: &sp_ark_models::short_weierstrass::Affine<sp_ark_bw6_761::g1::Config<HostBW6_761>>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out = <sp_ark_bw6_761::g1::Config<HostBW6_761> as SWCurveConfig>::mul_affine(base, scalar);
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
	base: &sp_ark_models::short_weierstrass::Projective<sp_ark_bw6_761::g1::Config<HostBW6_761>>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out =
		<sp_ark_bw6_761::g1::Config<HostBW6_761> as SWCurveConfig>::mul_projective(base, scalar);
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
	base: &sp_ark_models::short_weierstrass::Affine<sp_ark_bw6_761::g2::Config<HostBW6_761>>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out = <sp_ark_bw6_761::g2::Config<HostBW6_761> as SWCurveConfig>::mul_affine(base, scalar);
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
	base: &sp_ark_models::short_weierstrass::Projective<sp_ark_bw6_761::g2::Config<HostBW6_761>>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out =
		<sp_ark_bw6_761::g2::Config<HostBW6_761> as SWCurveConfig>::mul_projective(base, scalar);
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
