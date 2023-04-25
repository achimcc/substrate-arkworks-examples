use ark_std::{io::Error, vec::Vec};
pub use sp_ark_bls12_377::{
	Bls12_377 as Bls12_377_Host, G1Affine as G1AffineOptimized_Host,
	G1Projective as G1ProjectiveOptimized_Host, G2Affine as G2AffineOptimized_Host,
	G2Projective as G2ProjectiveOptimized_Host, HostFunctions as Bls12_377HostFunctions,
};
pub use sp_ark_models::{pairing::Pairing, short_weierstrass::SWCurveConfig, AffineRepr, Group};

pub struct HostBls12_377 {}

impl Bls12_377HostFunctions for HostBls12_377 {
	fn bls12_377_multi_miller_loop(a: Vec<u8>, b: Vec<u8>) -> Result<Vec<u8>, ()> {
		sp_io::elliptic_curves::bls12_377_multi_miller_loop(a, b)
	}
	fn bls12_377_final_exponentiation(f12: Vec<u8>) -> Result<Vec<u8>, ()> {
		sp_io::elliptic_curves::bls12_377_final_exponentiation(f12)
	}
	fn bls12_377_msm_g1(bases: Vec<u8>, bigints: Vec<u8>) -> Result<Vec<u8>, ()> {
		sp_io::elliptic_curves::bls12_377_msm_g1(bases, bigints)
	}
	fn bls12_377_msm_g2(bases: Vec<u8>, bigints: Vec<u8>) -> Result<Vec<u8>, ()> {
		sp_io::elliptic_curves::bls12_377_msm_g2(bases, bigints)
	}
	fn bls12_377_mul_projective_g1(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()> {
		sp_io::elliptic_curves::bls12_377_mul_projective_g1(base, scalar)
	}
	fn bls12_377_mul_projective_g2(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()> {
		sp_io::elliptic_curves::bls12_377_mul_projective_g2(base, scalar)
	}
}

pub type Bls12_377Optimized = Bls12_377_Host<HostBls12_377>;
pub type G1AffineOptimized = G1AffineOptimized_Host<HostBls12_377>;
pub type G2AffineOptimized = G2AffineOptimized_Host<HostBls12_377>;
pub type G1ProjectiveOptimized = G1ProjectiveOptimized_Host<HostBls12_377>;
pub type G2ProjectiveOptimized = G2ProjectiveOptimized_Host<HostBls12_377>;

pub fn do_pairing(a: ark_bls12_377::G1Affine, b: ark_bls12_377::G2Affine) -> Result<(), Error> {
	let _out = ark_bls12_377::Bls12_377::multi_pairing([a], [b]);
	Ok(())
}

pub fn do_pairing_optimized(a: G1AffineOptimized, b: G2AffineOptimized) -> Result<(), Error> {
	let _out = Bls12_377Optimized::multi_pairing([a], [b]);
	Ok(())
}

pub fn do_msm_g1(
	bases: &[ark_ec::short_weierstrass::Affine<ark_bls12_377::g1::Config>],
	scalars: &[<ark_bls12_377::g1::Config as ark_ec::CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out = <ark_bls12_377::g1::Config as SWCurveConfig>::msm(bases, scalars);
	Ok(())
}

pub fn do_msm_g1_optimized(
	bases: &[sp_ark_models::short_weierstrass::Affine<
		sp_ark_bls12_377::g1::Config<HostBls12_377>,
	>],
	scalars: &[<sp_ark_bls12_377::g1::Config<HostBls12_377> as sp_ark_models::CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out = <sp_ark_bls12_377::g1::Config<HostBls12_377> as sp_ark_models::short_weierstrass::SWCurveConfig>::msm(
		bases,
		scalars,
	);
	Ok(())
}

pub fn do_msm_g2(
	bases: &[ark_ec::short_weierstrass::Affine<ark_bls12_377::g2::Config>],
	scalars: &[<ark_bls12_377::g2::Config as ark_ec::CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out = <ark_bls12_377::g2::Config as SWCurveConfig>::msm(bases, scalars);
	Ok(())
}

pub fn do_msm_g2_optimized(
	bases: &[sp_ark_models::short_weierstrass::Affine<
		sp_ark_bls12_377::g2::Config<HostBls12_377>,
	>],
	scalars: &[<sp_ark_bls12_377::g2::Config<HostBls12_377> as ark_ec::CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out = <sp_ark_bls12_377::g2::Config<HostBls12_377> as SWCurveConfig>::msm(bases, scalars);
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
	base: &sp_ark_models::short_weierstrass::Affine<sp_ark_bls12_377::g1::Config<HostBls12_377>>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out = <sp_ark_bls12_377::g1::Config<HostBls12_377> as sp_ark_models::short_weierstrass::SWCurveConfig>::mul_affine(
		base,
		scalar,
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
	base: &sp_ark_models::short_weierstrass::Projective<
		sp_ark_bls12_377::g1::Config<HostBls12_377>,
	>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out = <sp_ark_bls12_377::g1::Config<HostBls12_377> as sp_ark_models::short_weierstrass::SWCurveConfig>::mul_projective(
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
	base: &sp_ark_models::short_weierstrass::Affine<sp_ark_bls12_377::g2::Config<HostBls12_377>>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out = <sp_ark_bls12_377::g2::Config<HostBls12_377> as sp_ark_models::short_weierstrass::SWCurveConfig>::mul_affine(
		base,
		scalar,
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
	base: &sp_ark_models::short_weierstrass::Projective<
		sp_ark_bls12_377::g2::Config<HostBls12_377>,
	>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out = <sp_ark_bls12_377::g2::Config<HostBls12_377> as sp_ark_models::short_weierstrass::SWCurveConfig>::mul_projective(
		base,
		scalar,
	);
	Ok(())
}
