use criterion::Criterion;

pub fn bench_msm_g1_bw6_761(c: &mut Criterion) {
	let mut group = c.benchmark_group("msm_g1_bw6_761");
	group.bench_function("normal, 10 arguments", |b| {
		b.iter(|| {
			let _ = pallet_template::bw6_761::do_msm_g1(10);
		});
	});
	group.bench_function("optimized,, 10 arguments", |b| {
		b.iter(|| {
			let _ = pallet_template::bw6_761::do_msm_g1_optimized(10);
		});
	});
	group.bench_function("normal, 1000 arguments", |b| {
		b.iter(|| {
			let _ = pallet_template::bw6_761::do_msm_g1(1000);
		});
	});
	group.bench_function("optimized, 1000 arguments", |b| {
		b.iter(|| {
			let _ = pallet_template::bw6_761::do_msm_g1_optimized(1000);
		});
	});

	group.finish();
}

pub fn bench_msm_g2_bw6_761(c: &mut Criterion) {
	let mut group = c.benchmark_group("msm_g2_bw6_761");
	group.bench_function("normal, 10 arguments", |b| {
		b.iter(|| {
			let _ = pallet_template::bw6_761::do_msm_g2(10);
		});
	});
	group.bench_function("optimized, 10 arguments", |b| {
		b.iter(|| {
			let _ = pallet_template::bw6_761::do_msm_g2_optimized(10);
		});
	});
	group.bench_function("normal, 1000 arguments", |b| {
		b.iter(|| {
			let _ = pallet_template::bw6_761::do_msm_g2(1000);
		});
	});
	group.bench_function("optimized, 1000 arguments", |b| {
		b.iter(|| {
			let _ = pallet_template::bw6_761::do_msm_g2_optimized(1000);
		});
	});
	group.finish();
}

pub fn bench_mul_affine_g1_bw6_761(c: &mut Criterion) {
	let mut group = c.benchmark_group("mul_affine_g1_bw6_761");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = pallet_template::bw6_761::do_mul_affine_g1();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = pallet_template::bw6_761::do_mul_affine_g1_optimized();
		});
	});
	group.finish();
}

pub fn bench_mul_projective_g1_bw6_761(c: &mut Criterion) {
	let mut group = c.benchmark_group("mul_projective_g1_bw6_761");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = pallet_template::bw6_761::do_mul_projective_g1();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = pallet_template::bw6_761::do_mul_projective_g1_optimized();
		});
	});
	group.finish();
}

pub fn bench_mul_affine_g2_bw6_761(c: &mut Criterion) {
	let mut group = c.benchmark_group("mul_affine_g2_bw6_761");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = pallet_template::bw6_761::do_mul_affine_g2();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = pallet_template::bw6_761::do_mul_affine_g2_optimized();
		});
	});
	group.finish();
}

pub fn bench_mul_projective_g2_bw6_761(c: &mut Criterion) {
	let mut group = c.benchmark_group("mul_projective_g2_bw6_761");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = pallet_template::bw6_761::do_mul_projective_g2();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = pallet_template::bw6_761::do_mul_projective_g2_optimized();
		});
	});
	group.finish();
}

pub fn bench_pairing_bw6_761(c: &mut Criterion) {
	let mut group = c.benchmark_group("pairing_bw6_761");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = pallet_template::bw6_761::do_pairing();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = pallet_template::bw6_761::do_pairing_optimized();
		});
	});
	group.finish();
}
