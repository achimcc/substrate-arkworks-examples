mod mock;
use criterion::Criterion;
use frame_benchmarking::whitelisted_caller;
use frame_support::dispatch::RawOrigin;
use mock::{new_test_ext, AccountId, TemplateModule};

pub fn bench_msm_ed_on_bls12_381(c: &mut Criterion) {
	let mut group = c.benchmark_group("bench_msm_ed_on_bls12_381");
	let caller: AccountId = whitelisted_caller();
	group.bench_function("msm_ed_on_bls12_381_optimized", |b| {
		b.iter(|| {
			msm_ed_on_bls12_381_optimized(caller);
		});
	});
	group.bench_function("msm_ed_on_bls12_381", |b| {
		b.iter(|| {
			msm_ed_on_bls12_381(caller);
		});
	});
	group.finish();
}

fn msm_ed_on_bls12_381_optimized(caller: AccountId) {
	new_test_ext().execute_with(|| {
		let _ = TemplateModule::msm_ed_on_bls12_381_optimized(RawOrigin::Signed(caller).into());
	});
}

fn msm_ed_on_bls12_381(caller: AccountId) {
	new_test_ext().execute_with(|| {
		let _ = TemplateModule::msm_ed_on_bls12_381(RawOrigin::Signed(caller).into());
	});
}

pub fn bench_mul_affine_ed_on_bls12_381(c: &mut Criterion) {
	let mut group = c.benchmark_group("bench_mul_affine_ed_on_bls12_381");
	let caller: AccountId = whitelisted_caller();
	group.bench_function("mul_affine_ed_on_bls12_381_optimized", |b| {
		b.iter(|| {
			mul_affine_ed_on_bls12_381_optimized(caller);
		});
	});
	group.bench_function("mul_affine_ed_on_bls12_381", |b| {
		b.iter(|| {
			mul_affine_ed_on_bls12_381(caller);
		});
	});
	group.finish();
}

fn mul_affine_ed_on_bls12_381_optimized(caller: AccountId) {
	new_test_ext().execute_with(|| {
		let _ =
			TemplateModule::mul_affine_ed_on_bls12_381_optimized(RawOrigin::Signed(caller).into());
	});
}

fn mul_affine_ed_on_bls12_381(caller: AccountId) {
	new_test_ext().execute_with(|| {
		let _ = TemplateModule::mul_affine_ed_on_bls12_381(RawOrigin::Signed(caller).into());
	});
}

pub fn bench_mul_projective_ed_on_bls12_381(c: &mut Criterion) {
	let mut group = c.benchmark_group("bench_mul_projective_ed_on_bls12_381");
	let caller: AccountId = whitelisted_caller();
	group.bench_function("mul_projective_ed_on_bls12_381_optimized", |b| {
		b.iter(|| {
			mul_projective_ed_on_bls12_381_optimized(caller);
		});
	});
	group.bench_function("mul_projective_ed_on_bls12_381", |b| {
		b.iter(|| {
			mul_projective_ed_on_bls12_381(caller);
		});
	});
	group.finish();
}

fn mul_projective_ed_on_bls12_381_optimized(caller: AccountId) {
	new_test_ext().execute_with(|| {
		let _ = TemplateModule::mul_projective_ed_on_bls12_381_optimized(
			RawOrigin::Signed(caller).into(),
		);
	});
}

fn mul_projective_ed_on_bls12_381(caller: AccountId) {
	new_test_ext().execute_with(|| {
		let _ = TemplateModule::mul_projective_ed_on_bls12_381(RawOrigin::Signed(caller).into());
	});
}
