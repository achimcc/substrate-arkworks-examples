use ark_algebra_bench_templates::*;
use ark_ed_on_bls12_381::{fq::Fq, fr::Fr, EdwardsProjective as G};
use criterion::Criterion;
use pallet_template::ed_on_bls12_381::HostEdOnBls12_381;
use sp_ark_ed_on_bls12_381::{
	fq::Fq as Fq_optimized, EdwardsProjective as EdwardsProjective_optimized, Fr as Fr_optimized,
};

bench!(Name = "EdOnBls12_381", Group = G, ScalarField = Fr, PrimeBaseField = Fq,);

type G_optimized = EdwardsProjective_optimized<HostEdOnBls12_381>;

bench!(
	Name = "EdOnBls12_381_optimized",
	Group = G_optimized,
	ScalarField = Fr_optimized,
	PrimeBaseField = Fq_optimized,
);

pub fn bench_msm_ed_on_bls12_381(c: &mut Criterion) {
	let mut group = c.benchmark_group("msm_ed_on_bls12_381");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = pallet_template::ed_on_bls12_381::do_msm();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = pallet_template::ed_on_bls12_381::do_msm_optimized();
		});
	});
	group.finish();
}

pub fn bench_mul_affine_ed_on_bls12_381(c: &mut Criterion) {
	let mut group = c.benchmark_group("mul_affine_ed_on_bls12_381");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = pallet_template::ed_on_bls12_381::do_mul_affine();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = pallet_template::ed_on_bls12_381::do_mul_affine_optimized();
		});
	});
	group.finish();
}

pub fn bench_mul_projective_ed_on_bls12_381(c: &mut Criterion) {
	let mut group = c.benchmark_group("mul_projective_ed_on_bls12_381");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = pallet_template::ed_on_bls12_381::do_mul_projective();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = pallet_template::ed_on_bls12_381::do_mul_projective_optimized();
		});
	});
	group.finish();
}
