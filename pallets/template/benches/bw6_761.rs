use ark_algebra_bench_templates::*;
use criterion::Criterion;

bench!(
	Name = "BW6_761",
	Pairing = ark_bw6_761::bw6_761,
	G1 = ark_bw6_761::g1::G1Projective,
	G2 = ark_bw6_761::g2::G2Projective,
	ScalarField = ark_bw6_761::fr::Fr,
	G1BaseField = ark_bw6_761::bw6_761::fq::Fq,
	G2BaseField = ark_bw6_761::bw6_761::fq3::Fq3,
	TargetField = ark_bw6_761::bw6_761::fq6::Fq6,
);

bench!(
	Name = "Bls12_381_optimied",
	Pairing = pallet_template::bw6_761::BW6_761Optimized,
	G1 = pallet_template::bw6_761::G1Projective_BW6_761_Host,
	G2 = pallet_template::bw6_761::G2Projective_BW6_761_Host,
	ScalarField = sp_ark_bw6_761::fr::Fr,
	G1BaseField = sp_ark_bw6_761::bw6_761::fq::Fq,
	G2BaseField = sp_ark_bw6_761::bw6_761::fq3::Fq3,
	TargetField = sp_ark_bw6_761::bw6_761::fq6::Fq6,
);

pub fn bench_msm_g1_bw6_761(c: &mut Criterion) {
	let mut group = c.benchmark_group("msm_g1_bw6_761");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = pallet_template::bw6_761::do_msm_g1();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = pallet_template::bw6_761::do_msm_g1_optimized();
		});
	});
	group.finish();
}

pub fn bench_msm_g2_bw6_761(c: &mut Criterion) {
	let mut group = c.benchmark_group("msm_g2_bw6_761");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = pallet_template::bw6_761::do_msm_g2();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = pallet_template::bw6_761::do_msm_g2_optimized();
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
