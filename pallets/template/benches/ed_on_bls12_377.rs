use ark_algebra_bench_templates::*;
use criterion::Criterion;

pub fn bench_msm_ed_on_bls12_377(c: &mut Criterion) {
	let mut group = c.benchmark_group("msm_ed_on_bls12_377");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = pallet_template::ed_on_bls12_377::do_msm();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = pallet_template::ed_on_bls12_377::do_msm_optimized();
		});
	});
	group.finish();
}

pub fn bench_mul_affine_ed_on_bls12_377(c: &mut Criterion) {
	let mut group = c.benchmark_group("mul_affine_ed_on_bls12_377");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = pallet_template::ed_on_bls12_377::do_mul_affine();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = pallet_template::ed_on_bls12_377::do_mul_affine_optimized();
		});
	});
	group.finish();
}

pub fn bench_mul_projective_ed_on_bls12_377(c: &mut Criterion) {
	let mut group = c.benchmark_group("mul_projective_ed_on_bls12_377");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = pallet_template::ed_on_bls12_377::do_mul_projective();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = pallet_template::ed_on_bls12_377::do_mul_projective_optimized();
		});
	});
	group.finish();
}
