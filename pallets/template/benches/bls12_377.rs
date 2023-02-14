use ark_algebra_bench_templates::*;

pub fn bench_pairing_bls12_377(c: &mut Criterion) {
	let mut group = c.benchmark_group("pairing_bls12_377");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = pallet_template::bls12_377::do_pairing();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = pallet_template::bls12_377::do_pairing_optimized();
		});
	});
	group.finish();
}

pub fn bench_msm_g1_bls12_377(c: &mut Criterion) {
	let mut group = c.benchmark_group("pairing_msm_g1_bls12_377");
	group.bench_function("normal, 10 arguments", |b| {
		b.iter(|| {
			let _ = pallet_template::bls12_377::do_msm_g1(10);
		});
	});
	group.bench_function("optimized, 10 arguments", |b| {
		b.iter(|| {
			let _ = pallet_template::bls12_377::do_msm_g1_optimized(10);
		});
	});
	group.bench_function("normal, 1000 arguments", |b| {
		b.iter(|| {
			let _ = pallet_template::bls12_377::do_msm_g1(1000);
		});
	});
	group.bench_function("optimized, 1000 arguments", |b| {
		b.iter(|| {
			let _ = pallet_template::bls12_377::do_msm_g1_optimized(1000);
		});
	});
	group.finish();
}

pub fn bench_msm_g2_bls12_377(c: &mut Criterion) {
	let mut group = c.benchmark_group("pairing_msm_g2_bls12_377");
	group.bench_function("normal, 10 arguments", |b| {
		b.iter(|| {
			let _ = pallet_template::bls12_377::do_msm_g2(10);
		});
	});
	group.bench_function("optimized, 10 arguments", |b| {
		b.iter(|| {
			let _ = pallet_template::bls12_377::do_msm_g2_optimized(10);
		});
	});
	group.bench_function("normal, 1000 arguments", |b| {
		b.iter(|| {
			let _ = pallet_template::bls12_377::do_msm_g2(1000);
		});
	});
	group.bench_function("optimized, 1000 arguments", |b| {
		b.iter(|| {
			let _ = pallet_template::bls12_377::do_msm_g2_optimized(1000);
		});
	});
	group.finish();
}

pub fn bench_mul_projective_g1_bls12_377(c: &mut Criterion) {
	let mut group = c.benchmark_group("mul_projective_g1_bls12_377");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = pallet_template::bls12_377::do_mul_projective_g1();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = pallet_template::bls12_377::do_mul_projective_g1_optimized();
		})
	});
	group.finish();
}

pub fn bench_mul_affine_g1_bls12_377(c: &mut Criterion) {
	let mut group = c.benchmark_group("mul_affine_g1_bls12_377");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = pallet_template::bls12_377::do_mul_affine_g1();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = pallet_template::bls12_377::do_mul_affine_g1_optimized();
		});
	});
	group.finish();
}

pub fn bench_mul_projective_g2_bls12_377(c: &mut Criterion) {
	let mut group = c.benchmark_group("mul_projective_g2_bls12_377");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = pallet_template::bls12_377::do_mul_projective_g2();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = pallet_template::bls12_377::do_mul_projective_g2_optimized();
		});
	});
	group.finish();
}

pub fn bench_mul_affine_g2_bls12_377(c: &mut Criterion) {
	let mut group = c.benchmark_group("mul_affine_g2_bls12_377");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = pallet_template::bls12_377::do_mul_affine_g2();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = pallet_template::bls12_377::do_mul_affine_g2_optimized();
		});
	});
	group.finish();
}
