use ark_algebra_bench_templates::*;
use criterion::Criterion;

pub fn bench_msm_ed_on_bls12_377(c: &mut Criterion) {
	let mut group = c.benchmark_group("msm_10_ed_on_bls12_377");
	group.bench_function("normal, 10 arguments", |b| {
		b.iter(|| {
			let _ = pallet_template::ed_on_bls12_377::do_msm(10);
		});
	});
	group.bench_function("optimized, 10 arguments", |b| {
		b.iter(|| {
			let _ = pallet_template::ed_on_bls12_377::do_msm_optimized(10);
		});
	});
	group.bench_function("normal, 1000 arguments", |b| {
		b.iter(|| {
			let _ = pallet_template::ed_on_bls12_377::do_msm(1000);
		});
	});
	group.bench_function("optimized, 1000 arguments", |b| {
		b.iter(|| {
			let _ = pallet_template::ed_on_bls12_377::do_msm_optimized(1000);
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
