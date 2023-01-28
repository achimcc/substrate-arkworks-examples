use ark_algebra_bench_templates::*;
use ark_bls12_381::{
	fq12::Fq12, fq2::Fq2, fr::Fr, g1::G1Projective, g2::G2Projective, Bls12_381, Fq,
};
use criterion::Criterion;
use pallet_template::bls12_381::{
	Bls12_381Optimized, G1ProjectiveBls12_381_Host, G2ProjectiveBls12_381_Host,
};
use sp_ark_bls12_381::{
	fq2::Fq2 as Fq2_optimized, fqFq as Fq_optimized, Fq12 as Fq12_optimized, Fr as Fr_optimized,
};

bench!(
	Name = "Bls12_381",
	Pairing = Bls12_381,
	G1 = G1Projective,
	G2 = G2Projective,
	ScalarField = Fr,
	G1BaseField = Fq,
	G2BaseField = Fq2,
	TargetField = Fq12,
);

bench!(
	Name = "Bls12_381_optimied",
	Pairing = Bls12_381_optimied,
	G1 = G1ProjectiveBls12_381_Host
	G2 = G2ProjectiveBls12_381_Host,
	ScalarField = Fr_optimized,
	G1BaseField = Fq_optimized,
	G2BaseField = Fq2_optimized,
	TargetField = Fq12_optimized,
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
