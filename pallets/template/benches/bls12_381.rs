use ark_algebra_bench_templates::*;
use criterion::Criterion;

bench!(
	Name = "Bls12_381",
	Pairing = ark_bls12_381::Bls12_381,
	G1 = ark_bls12_381::g1::G1Projective,
	G2 = ark_bls12_381::g2::G2Projective,
	ScalarField = ark_bls12_381::fr::Fr,
	G1BaseField = ark_bls12_381::fq::Fq,
	G2BaseField = ark_bls12_381::fq2::Fq2,
	TargetField = ark_bls12_381::Fq12,
);

bench!(
	Name = "Bls12_381_optimied",
	Pairing = pallet_template::bls12_381::Bls12_381_optimied,
	G1 = pallet_template::bls12_381::G1ProjectiveBls12_381_Host
	G2 = pallet_template::bls12_381::G2ProjectiveBls12_381_Host,
	ScalarField = sp_ark_bls12_381::fr::Fr,
	G1BaseField = sp_ark_bls12_381::bls12_381::fq::Fq,
	G2BaseField = sp_ark_bls12_381::bls12_381::fq2::Fq2,
	TargetField = sp_ark_bls12_381::bls12_381::Fq12,
);

pub fn bench_pairing_bls12_381(c: &mut Criterion) {
	let mut group = c.benchmark_group("pairing_bls12_381");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = pallet_template::bls12_381::do_pairing();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = pallet_template::bls12_381::do_pairing_optimized();
		});
	});
	group.finish();
}

pub fn bench_msm_g1_bls12_381(c: &mut Criterion) {
	let mut group = c.benchmark_group("msm_g1_bls12_381");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = pallet_template::bls12_381::do_msm_g1();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = pallet_template::bls12_381::do_msm_g1_optimized();
		});
	});
	group.finish();
}

pub fn bench_msm_g2_bls12_381(c: &mut Criterion) {
	let mut group = c.benchmark_group("msm_g2_bls12_381");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = pallet_template::bls12_381::do_msm_g2();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = pallet_template::bls12_381::do_msm_g2_optimized();
		});
	});
	group.finish();
}

pub fn bench_mul_affine_g1_bls12_381(c: &mut Criterion) {
	let mut group = c.benchmark_group("mul_affine_g1_bls12_381");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = pallet_template::bls12_381::do_mul_affine_g1();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = pallet_template::bls12_381::do_mul_affine_g1_optimized();
		});
	});
	group.finish();
}

pub fn bench_mul_projective_g1_bls12_381(c: &mut Criterion) {
	let mut group = c.benchmark_group("mul_projective_g1_bls12_381");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = pallet_template::bls12_381::do_mul_projective_g1();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = pallet_template::bls12_381::do_mul_projective_g1_optimized();
		});
	});
	group.finish();
}

pub fn bench_mul_affine_g2_bls12_381(c: &mut Criterion) {
	let mut group = c.benchmark_group("mul_affine_g2_bls12_381");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = pallet_template::bls12_381::do_mul_affine_g2();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = pallet_template::bls12_381::do_mul_affine_g2_optimized();
		});
	});
	group.finish();
}

pub fn bench_mul_projective_g2_bls12_381(c: &mut Criterion) {
	let mut group = c.benchmark_group("mul_projective_g2_bls12_381");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = pallet_template::bls12_381::do_mul_projective_g2();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = pallet_template::bls12_381::do_mul_projective_g2_optimized();
		});
	});
	group.finish();
}

pub fn bench_groth16(c: &mut Criterion) {
	let mut group = c.benchmark_group("groth16");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = pallet_template::bls12_381::do_verify_groth16();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = pallet_template::bls12_381::do_verify_groth16_optimized();
		});
	});
	group.finish();
}
