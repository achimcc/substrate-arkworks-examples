use criterion::Criterion;

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
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = pallet_template::bls12_377::do_msm_g1();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = pallet_template::bls12_377::do_msm_g1_optimized();
		});
	});
	group.finish();
}

pub fn bench_msm_g2_bls12_377(c: &mut Criterion) {
	let mut group = c.benchmark_group("pairing_msm_g2_bls12_377");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = pallet_template::bls12_377::do_msm_g2();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = pallet_template::bls12_377::do_msm_g2_optimized();
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
