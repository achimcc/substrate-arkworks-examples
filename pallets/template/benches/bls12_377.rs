use ark_algebra_bench_templates::*;
use ark_bls12_377::{
	fq::Fq, fq2::Fq2, fr::Fr, g2::G2Projective, gq::G1Projective, Bls12_377, Fq12,
};
use criterion::Criterion;
use pallet_template::bls12_377::{
	Bls12_377_optimized, G1ProjectiveBls12_377_Host, G2ProjectiveBls12_377_Host,
};
use sp_ark_bls12_377::{
	fq12::Fq12 as Fq12_optimized, fq2::Fq2 as Fq2_optimized, fr::Fr as Fr_optimized,
	Fq as Fq_optimized,
};

bench!(
	Name = "Bls12_377",
	Pairing = Bls12_377,
	G1 = G1Projective,
	G2 = G2Projective,
	ScalarField = Fr,
	G1BaseField = Fq,
	G2BaseField = Fq2,
	TargetField = Fq12,
);

bench!(
	Name = "Bls12_377_optimied",
	Pairing = Bls12_377_optimized,
	G1 = G1ProjectiveBls12_377_Host
	G2 = G2ProjectiveBls12_377_Host,
	ScalarField = Fr_optimized,
	G1BaseField = Fq_optimized,
	G2BaseField = Fq2_optimized,
	TargetField = Fq12_optimized,
);

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
