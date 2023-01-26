use ark_std::io::Error;
use criterion::Criterion;
use sp_ark_ed_on_bls12_377::HostFunctions as EdwardBls12_377HostFunctions;
use sp_ark_models::{AffineRepr, Group, TECurveConfig};

struct HostEdOnBls12_377 {}

impl EdwardBls12_377HostFunctions for HostEdOnBls12_377 {
	fn ed_on_bls12_377_mul_affine(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::ed_on_bls12_377_mul_affine(base, scalar)
	}
	fn ed_on_bls12_377_mul_projective(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::ed_on_bls12_377_mul_projective(base, scalar)
	}
	fn ed_on_bls12_377_msm(bases: Vec<Vec<u8>>, scalars: Vec<Vec<u8>>) -> Vec<u8> {
		sp_io::elliptic_curves::ed_on_bls12_377_msm(bases, scalars)
	}
}

pub fn bench_msm_ed_on_bls12_377(c: &mut Criterion) {
	let mut group = c.benchmark_group("msm_ed_on_bls12_377");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = do_msm_ed_on_bls12_377();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = do_msm_ed_on_bls12_377_optimized();
		});
	});
	group.finish();
}

fn do_msm_ed_on_bls12_377() -> Result<(), Error> {
	let _out =
		<ark_ed_on_bls12_377::EdwardsConfig as ark_ec::models::twisted_edwards::TECurveConfig>::msm(
			&[ark_ed_on_bls12_377::EdwardsAffine::generator()],
			&[2u64.into()],
		);
	Ok(())
}

fn do_msm_ed_on_bls12_377_optimized() -> Result<(), Error> {
	let _out = <sp_ark_ed_on_bls12_377::EdwardsConfig<HostEdOnBls12_377> as TECurveConfig>::msm(
		&[sp_ark_ed_on_bls12_377::EdwardsAffine::<HostEdOnBls12_377>::generator()],
		&[2u64.into()],
	);
	Ok(())
}

pub fn bench_mul_affine_ed_on_bls12_377(c: &mut Criterion) {
	let mut group = c.benchmark_group("mul_affine_ed_on_bls12_377");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = do_mul_affine_ed_on_bls12_377();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = do_mul_affine_ed_on_bls12_377_optimized();
		});
	});
	group.finish();
}

fn do_mul_affine_ed_on_bls12_377() -> Result<(), Error> {
	let _out =
		<ark_ed_on_bls12_377::EdwardsConfig as ark_ec::models::twisted_edwards::TECurveConfig>::msm(
			&[ark_ed_on_bls12_377::EdwardsAffine::generator()],
			&[2u64.into()],
		);
	Ok(())
}

fn do_mul_affine_ed_on_bls12_377_optimized() -> Result<(), Error> {
	let _out = <sp_ark_ed_on_bls12_377::EdwardsConfig<HostEdOnBls12_377> as TECurveConfig>::msm(
		&[sp_ark_ed_on_bls12_377::EdwardsAffine::<HostEdOnBls12_377>::generator()],
		&[2u64.into()],
	);
	Ok(())
}

pub fn bench_mul_projective_ed_on_bls12_377(c: &mut Criterion) {
	let mut group = c.benchmark_group("mul_projective_ed_on_bls12_377");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = do_mul_projective_ed_on_bls12_377();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = do_mul_projective_ed_on_bls12_377_optimized();
		});
	});
	group.finish();
}

fn do_mul_projective_ed_on_bls12_377() -> Result<(), Error> {
	let _out = <ark_ed_on_bls12_377::EdwardsConfig as TECurveConfig>::mul_projective(
		&ark_ed_on_bls12_377::EdwardsProjective::generator(),
		&[2u64],
	);
	Ok(())
}

fn do_mul_projective_ed_on_bls12_377_optimized() -> Result<(), Error> {
	let _out =
		<sp_ark_ed_on_bls12_377::EdwardsConfig<HostEdOnBls12_377> as TECurveConfig>::mul_projective(
			&sp_ark_ed_on_bls12_377::EdwardsProjective::<HostEdOnBls12_377>::generator(),
			&[2u64],
		);
	Ok(())
}
