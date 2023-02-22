use ark_std::{io::Error, vec::Vec};
use sp_ark_ed_on_bls12_381::HostFunctions as EdwardBls12_381HostFunctions;
use sp_ark_models::{short_weierstrass::SWCurveConfig, AffineRepr, Group};

pub struct HostEdOnBls12_381 {}

impl EdwardBls12_381HostFunctions for HostEdOnBls12_381 {
	fn ed_on_bls12_381_sw_mul_affine(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::ed_on_bls12_381_sw_mul_affine(base, scalar)
	}
	fn ed_on_bls12_381_te_mul_projective(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::ed_on_bls12_381_te_mul_projective(base, scalar)
	}
	fn ed_on_bls12_381_te_mul_affine(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::ed_on_bls12_381_te_mul_affine(base, scalar)
	}
	fn ed_on_bls12_381_sw_mul_projective(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::ed_on_bls12_381_sw_mul_projective(base, scalar)
	}
	fn ed_on_bls12_381_te_msm(bases: Vec<Vec<u8>>, scalars: Vec<Vec<u8>>) -> Vec<u8> {
		sp_io::elliptic_curves::ed_on_bls12_381_te_msm(bases, scalars)
	}
	fn ed_on_bls12_381_sw_msm(bases: Vec<Vec<u8>>, scalars: Vec<Vec<u8>>) -> Vec<u8> {
		sp_io::elliptic_curves::ed_on_bls12_381_sw_msm(bases, scalars)
	}
}

pub fn do_msm_sw(samples: u32) -> Result<(), Error> {
	let g = ark_ed_on_bls12_381::SWAffine::generator();
	let v: Vec<_> = (0..samples).map(|_| g).collect();
	let scalars: Vec<_> = (0..samples).map(|_| ark_ff::Fp::from(2u32)).collect();
	let _out = <ark_ed_on_bls12_381::EdwardsConfig as SWCurveConfig>::msm(&v[..], &scalars[..]);
	Ok(())
}

pub fn do_msm_sw_optimized(samples: u32) -> Result<(), Error> {
	let g = ark_ec::short_weierstrass::Affine::<
		sp_ark_ed_on_bls12_381::JubjubConfig<HostEdOnBls12_381>,
	>::generator();
	let v: Vec<_> = (0..samples).map(|_| g).collect();
	let scalars: Vec<_> = (0..samples).map(|_| ark_ff::Fp::from(2u64)).collect();
	let _out = <sp_ark_ed_on_bls12_381::EdwardsConfig<HostEdOnBls12_381> as SWCurveConfig>::msm(
		&v[..],
		&scalars[..],
	);
	Ok(())
}

pub fn do_msm_te(samples: u32) -> Result<(), Error> {
	let g = ark_ed_on_bls12_381::EdwardsAffine::generator();
	let v: Vec<_> = (0..samples).map(|_| g).collect();
	let scalars: Vec<_> = (0..samples).map(|_| ark_ff::Fp::from(2u64)).collect();
	let _out = <ark_ed_on_bls12_381::JubjubConfig as ark_ec::twisted_edwards::TECurveConfig>::msm(
		&v[..],
		&scalars[..],
	);
	Ok(())
}

pub fn do_msm_te_optimized(samples: u32) -> Result<(), Error> {
	let g = sp_ark_ed_on_bls12_381::EdwardsAffine::<HostEdOnBls12_381>::generator();
	let v: Vec<_> = (0..samples).map(|_| g).collect();
	let scalars: Vec<_> = (0..samples).map(|_| ark_ff::Fp::from(2u64)).collect();
	let _out = <sp_ark_ed_on_bls12_381::JubjubConfig<HostEdOnBls12_381> as sp_ark_models::twisted_edwards::TECurveConfig>::msm(
		&v[..],
		&scalars[..],
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
