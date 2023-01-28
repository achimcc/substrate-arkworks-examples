use ark_algebra_bench_templates::*;
use ark_bw6_761::{fq::Fq, fq3::Fq3, fq6::Fq6, Fr, G1Projective, G2Projective, BW6_761};
use criterion::Criterion;
use pallet_template::bw6_761::{BW6_761Optimized, G1ProjectiveBW6_761, G2ProjectiveBW6_761};
use sp_ark_bw6_761::{
	fq::Fq as Fq_optimized, fq3::Fq3 as Fq3_optimized, fq6::Fq6 as Fq6_optimized,
	fr::Fr as Fr_optimized,
};

bench!(
	Name = "BW6_761",
	Pairing = BW6_761,
	G1 = G1Projective,
	G2 = G2Projective,
	ScalarField = Fr,
	G1BaseField = Fq,
	G2BaseField = Fq3,
	TargetField = Fq6,
);

bench!(
	Name = "BW6_761_optimied",
	Pairing = BW6_761Optimized,
	G1 = G1ProjectiveBW6_761,
	G2 = G2ProjectiveBW6_761,
	ScalarField = Fr_optimized,
	G1BaseField = Fq_optimized,
	G2BaseField = Fq3_optimized,
	TargetField = Fq6_optimized,
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
