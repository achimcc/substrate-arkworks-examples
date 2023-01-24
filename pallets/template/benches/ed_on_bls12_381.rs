use ark_std::{io::Error, test_rng, UniformRand};
use criterion::Criterion;
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

pub fn bench_msm_ed_on_bls12_381(c: &mut Criterion) {
	let mut group = c.benchmark_group("msm_ed_on_bls12_381");
	group.bench_function("msm_ed_on_bls12_381", |b| {
		b.iter(|| {
			let _ = do_msm_ed_on_bls12_381();
		});
	});
	group.bench_function("msm_ed_on_bls12_381_optimized", |b| {
		b.iter(|| {
			let _ = do_msm_ed_on_bls12_381_optimized();
		});
	});
	group.finish();
}

fn do_msm_ed_on_bls12_381() -> Result<(), Error> {
	let mut rng = test_rng();
	let scalar = ark_ed_on_bls12_381::Fr::rand(&mut rng);
	let _out = <ark_ed_on_bls12_381::EdwardsConfig as SWCurveConfig>::msm(
		&[ark_ed_on_bls12_381::SWAffine::generator()],
		&[scalar],
	);
	Ok(())
}

fn do_msm_ed_on_bls12_381_optimized() -> Result<(), Error> {
	let mut rng = test_rng();
	let scalar = sp_ark_ed_on_bls12_381::Fr::rand(&mut rng);
	let _out = <sp_ark_ed_on_bls12_381::EdwardsConfig<HostEdOnBls12_381> as SWCurveConfig>::msm(
		&[sp_ark_ed_on_bls12_381::SWAffine::<HostEdOnBls12_381>::generator()],
		&[scalar],
	);
	Ok(())
}

pub fn bench_mul_affine_ed_on_bls12_381(c: &mut Criterion) {
	let mut group = c.benchmark_group("mul_affine_ed_on_bls12_381");
	group.bench_function("mul_affine_ed_on_bls12_381", |b| {
		b.iter(|| {
			let _ = do_mul_affine_ed_on_bls12_381();
		});
	});
	group.bench_function("mul_affine_ed_on_bls12_381_optimized", |b| {
		b.iter(|| {
			let _ = do_mul_affine_ed_on_bls12_381_optimized();
		});
	});
	group.finish();
}

fn do_mul_affine_ed_on_bls12_381() -> Result<(), Error> {
	let _out = <ark_ed_on_bls12_381::EdwardsConfig as SWCurveConfig>::mul_affine(
		&ark_ed_on_bls12_381::SWAffine::generator(),
		&[2u64],
	);
	Ok(())
}

fn do_mul_affine_ed_on_bls12_381_optimized() -> Result<(), Error> {
	let _out =
		<sp_ark_ed_on_bls12_381::EdwardsConfig<HostEdOnBls12_381> as SWCurveConfig>::mul_affine(
			&sp_ark_ed_on_bls12_381::SWAffine::<HostEdOnBls12_381>::generator(),
			&[2u64],
		);
	Ok(())
}

pub fn bench_mul_projective_ed_on_bls12_381(c: &mut Criterion) {
	let mut group = c.benchmark_group("mul_projective_ed_on_bls12_381");
	group.bench_function("mul_projective_ed_on_bls12_381", |b| {
		b.iter(|| {
			let _ = do_mul_projective_ed_on_bls12_381();
		});
	});
	group.bench_function("mul_projective_ed_on_bls12_381_optimized", |b| {
		b.iter(|| {
			let _ = do_mul_projective_ed_on_bls12_381_optimized();
		});
	});
	group.finish();
}

fn do_mul_projective_ed_on_bls12_381() -> Result<(), Error> {
	let _out = <ark_ed_on_bls12_381::EdwardsConfig as SWCurveConfig>::mul_projective(
		&ark_ed_on_bls12_381::SWProjective::generator(),
		&[2u64],
	);
	Ok(())
}

fn do_mul_projective_ed_on_bls12_381_optimized() -> Result<(), Error> {
	let _out =
		<sp_ark_ed_on_bls12_381::EdwardsConfig<HostEdOnBls12_381> as SWCurveConfig>::mul_projective(
			&sp_ark_ed_on_bls12_381::SWProjective::<HostEdOnBls12_381>::generator(),
			&[2u64],
		);
	Ok(())
}
