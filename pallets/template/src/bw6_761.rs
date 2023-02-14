use ark_std::{io::Error, test_rng, vec::Vec, UniformRand};
pub use sp_ark_bw6_761::{
	G1Affine as G1AffineBW6_761_Host, G1Projective as G1ProjectiveBW6_761_Host,
	G2Affine as G2AffineBW6_761_Host, G2Projective as G2ProjectiveBW6_761_Host,
	HostFunctions as BW6_761HostFunctions, BW6_761 as BW6_761_Host,
};
pub use sp_ark_models::{pairing::Pairing, short_weierstrass::SWCurveConfig, AffineRepr, Group};

pub struct HostBW6_761 {}

impl BW6_761HostFunctions for HostBW6_761 {
	fn bw6_761_multi_miller_loop(a: Vec<Vec<u8>>, b: Vec<Vec<u8>>) -> Vec<u8> {
		sp_io::elliptic_curves::bw6_761_multi_miller_loop(a, b)
	}
	fn bw6_761_final_exponentiation(f12: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::bw6_761_final_exponentiation(f12)
	}
	fn bw6_761_mul_projective_g2(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::bw6_761_mul_projective_g2(base, scalar)
	}
	fn bw6_761_mul_affine_g2(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::bw6_761_mul_affine_g2(base, scalar)
	}
	fn bw6_761_mul_projective_g1(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::bw6_761_mul_projective_g1(base, scalar)
	}
	fn bw6_761_mul_affine_g1(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::bw6_761_mul_affine_g1(base, scalar)
	}
	fn bw6_761_msm_g1(bases: Vec<Vec<u8>>, bigints: Vec<Vec<u8>>) -> Vec<u8> {
		sp_io::elliptic_curves::bw6_761_msm_g1(bases, bigints)
	}
	fn bw6_761_msm_g2(bases: Vec<Vec<u8>>, bigints: Vec<Vec<u8>>) -> Vec<u8> {
		sp_io::elliptic_curves::bw6_761_msm_g2(bases, bigints)
	}
}

pub type BW6_761Optimized = BW6_761_Host<HostBW6_761>;
pub type G1AffineBW6_761 = G1AffineBW6_761_Host<HostBW6_761>;
pub type G2AffineBW6_761 = G2AffineBW6_761_Host<HostBW6_761>;
pub type G1ProjectiveBW6_761 = G1ProjectiveBW6_761_Host<HostBW6_761>;
pub type G2ProjectiveBW6_761 = G2ProjectiveBW6_761_Host<HostBW6_761>;

pub fn do_msm_g1(samples: u32) -> Result<(), Error> {
	let mut rng = test_rng();
	let g = ark_bw6_761::G1Affine::rand(&mut rng);
	let v: Vec<_> = (0..samples).map(|_| g).collect();
	let scalars: Vec<_> = (0..samples)
		.map(|_| <ark_bw6_761::g1::Config as ark_ec::CurveConfig>::ScalarField::rand(&mut rng))
		.collect();
	let _out = <ark_bw6_761::g1::Config as SWCurveConfig>::msm(&v[..], &scalars[..]);

	Ok(())
}

pub fn do_msm_g1_optimized(samples: u32) -> Result<(), Error> {
	let mut rng = test_rng();
	let g = G1AffineBW6_761_Host::rand(&mut rng);
	let v: Vec<_> = (0..samples).map(|_| g).collect();
	let scalars: Vec<_> = (0..samples)
		.map(|_| {
			<sp_ark_bw6_761::g1::Config<HostBW6_761> as sp_ark_models::CurveConfig>::ScalarField::rand(&mut rng)
		})
		.collect();
	let _out =
		<sp_ark_bw6_761::g1::Config<HostBW6_761> as SWCurveConfig>::msm(&v[..], &scalars[..]);
	Ok(())
}

pub fn do_msm_g2(samples: u32) -> Result<(), Error> {
	let mut rng = test_rng();
	let g = ark_bw6_761::G2Affine::rand(&mut rng);
	let v: Vec<_> = (0..samples).map(|_| g).collect();
	let scalars: Vec<_> = (0..samples)
		.map(|_| <ark_bw6_761::g1::Config as ark_ec::CurveConfig>::ScalarField::rand(&mut rng))
		.collect();
	let _out = <ark_bw6_761::g2::Config as SWCurveConfig>::msm(&v[..], &scalars[..]);
	Ok(())
}

pub fn do_msm_g2_optimized(samples: u32) -> Result<(), Error> {
	let mut rng = test_rng();
	let g = G2AffineBW6_761_Host::rand(&mut rng);
	let v: Vec<_> = (0..samples).map(|_| g).collect();
	let scalars: Vec<_> = (0..samples)
		.map(|_| {
			<sp_ark_bw6_761::g1::Config<HostBW6_761> as sp_ark_models::CurveConfig>::ScalarField::rand(&mut rng)
		})
		.collect();
	let _out =
		<sp_ark_bw6_761::g2::Config<HostBW6_761> as SWCurveConfig>::msm(&v[..], &scalars[..]);
	Ok(())
}

pub fn do_mul_affine_g1() -> Result<(), Error> {
	let _out = <ark_bw6_761::g1::Config as SWCurveConfig>::mul_affine(
		&ark_bw6_761::G1Affine::generator(),
		&[2u64],
	);
	Ok(())
}

pub fn do_mul_affine_g1_optimized() -> Result<(), Error> {
	let _out = <sp_ark_bw6_761::g1::Config<HostBW6_761> as SWCurveConfig>::mul_affine(
		&G1AffineBW6_761::generator(),
		&[2u64],
	);
	Ok(())
}

pub fn do_mul_projective_g1() -> Result<(), Error> {
	let _out = <ark_bw6_761::g1::Config as SWCurveConfig>::mul_projective(
		&ark_bw6_761::G1Projective::generator(),
		&[2u64],
	);
	Ok(())
}

pub fn do_mul_projective_g1_optimized() -> Result<(), Error> {
	let _out = <sp_ark_bw6_761::g1::Config<HostBW6_761> as SWCurveConfig>::mul_projective(
		&G1ProjectiveBW6_761::generator(),
		&[2u64],
	);
	Ok(())
}

pub fn do_mul_affine_g2() -> Result<(), Error> {
	let _out = <ark_bw6_761::g2::Config as SWCurveConfig>::mul_affine(
		&ark_bw6_761::G2Affine::generator(),
		&[2u64],
	);
	Ok(())
}

pub fn do_mul_affine_g2_optimized() -> Result<(), Error> {
	let _out = <sp_ark_bw6_761::g2::Config<HostBW6_761> as SWCurveConfig>::mul_affine(
		&G2AffineBW6_761::generator(),
		&[2u64],
	);
	Ok(())
}

pub fn do_mul_projective_g2() -> Result<(), Error> {
	let _out = <ark_bw6_761::g2::Config as SWCurveConfig>::mul_projective(
		&ark_bw6_761::G2Projective::generator(),
		&[2u64],
	);
	Ok(())
}

pub fn do_mul_projective_g2_optimized() -> Result<(), Error> {
	let _out = <sp_ark_bw6_761::g2::Config<HostBW6_761> as SWCurveConfig>::mul_projective(
		&G2ProjectiveBW6_761::generator(),
		&[2u64],
	);
	Ok(())
}

pub fn do_pairing() -> Result<(), Error> {
	let _out = ark_bw6_761::BW6_761::multi_pairing(
		[ark_bw6_761::G1Affine::generator()],
		[ark_bw6_761::G2Affine::generator()],
	);
	Ok(())
}

pub fn do_pairing_optimized() -> Result<(), Error> {
	let _out = BW6_761Optimized::multi_pairing(
		[G1AffineBW6_761::generator()],
		[G2AffineBW6_761::generator()],
	);
	Ok(())
}
