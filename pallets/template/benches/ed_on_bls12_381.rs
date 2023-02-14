// use ark_algebra_bench_templates::*;
// use ark_ed_on_bls12_381::{fq::Fq, fr::Fr, EdwardsProjective as G};
use criterion::Criterion;
// use pallet_template::ed_on_bls12_381::HostEdOnBls12_381;
// use sp_ark_ed_on_bls12_381::{
// 	fq::Fq as Fq_optimized, EdwardsProjective as EdwardsProjective_optimized, Fr as Fr_optimized,
// };

// bench!(Name = "EdOnBls12_381", Group = G, ScalarField = Fr, PrimeBaseField = Fq,);

// type G_optimized = EdwardsProjective_optimized<HostEdOnBls12_381>;

// bench!(
// 	Name = "EdOnBls12_381_optimized",
// 	Group = G_optimized,
// 	ScalarField = Fr_optimized,
// 	PrimeBaseField = Fq_optimized,
// );

pub fn bench_msm_sw_ed_on_bls12_381(c: &mut Criterion) {
	let mut group = c.benchmark_group("msm_sw_ed_on_bls12_381");
	group.bench_function("normal, 10 arguments", |b| {
		b.iter(|| {
			let _ = pallet_template::ed_on_bls12_381::do_msm_sw(10);
		});
	});
	group.bench_function("optimized, 10 arguments", |b| {
		b.iter(|| {
			let _ = pallet_template::ed_on_bls12_381::do_msm_sw_optimized(10);
		});
	});
	group.bench_function("normal, 1000 arguments", |b| {
		b.iter(|| {
			let _ = pallet_template::ed_on_bls12_381::do_msm_sw(1000);
		});
	});
	group.bench_function("optimized, 1000 arguments", |b| {
		b.iter(|| {
			let _ = pallet_template::ed_on_bls12_381::do_msm_sw_optimized(1000);
		});
	});
	group.finish();
}

pub fn bench_mul_affine_sw_ed_on_bls12_381(c: &mut Criterion) {
	let mut group = c.benchmark_group("mul_affine_sw_ed_on_bls12_381");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = pallet_template::ed_on_bls12_381::do_mul_affine_sw();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = pallet_template::ed_on_bls12_381::do_mul_affine_sw_optimized();
		});
	});
	group.finish();
}

pub fn bench_mul_projective_sw_ed_on_bls12_381(c: &mut Criterion) {
	let mut group = c.benchmark_group("mul_projective_sw_ed_on_bls12_381");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = pallet_template::ed_on_bls12_381::do_mul_projective_sw();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = pallet_template::ed_on_bls12_381::do_mul_projective_sw_optimized();
		});
	});
	group.finish();
}

pub fn bench_msm_te_ed_on_bls12_381(c: &mut Criterion) {
	let mut group = c.benchmark_group("msm_te_10_ed_on_bls12_381");
	group.bench_function("normal, 10 arguments", |b| {
		b.iter(|| {
			let _ = pallet_template::ed_on_bls12_381::do_msm_sw(10);
		});
	});
	group.bench_function("optimized, 10 arguments", |b| {
		b.iter(|| {
			let _ = pallet_template::ed_on_bls12_381::do_msm_sw_optimized(10);
		});
	});
	group.bench_function("normal, 1000 arguments", |b| {
		b.iter(|| {
			let _ = pallet_template::ed_on_bls12_381::do_msm_te(1000);
		});
	});
	group.bench_function("optimized, 1000 arguments", |b| {
		b.iter(|| {
			let _ = pallet_template::ed_on_bls12_381::do_msm_te_optimized(1000);
		});
	});
	group.finish();
}

pub fn bench_mul_affine_te_ed_on_bls12_381(c: &mut Criterion) {
	let mut group = c.benchmark_group("mul_affine_te_ed_on_bls12_381");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = pallet_template::ed_on_bls12_381::do_mul_affine_te();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = pallet_template::ed_on_bls12_381::do_mul_affine_te_optimized();
		});
	});
	group.finish();
}

pub fn bench_mul_projective_te_ed_on_bls12_381(c: &mut Criterion) {
	let mut group = c.benchmark_group("mul_projective_te_ed_on_bls12_381");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = pallet_template::ed_on_bls12_381::do_mul_projective_te();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = pallet_template::ed_on_bls12_381::do_mul_projective_te_optimized();
		});
	});
	group.finish();
}
