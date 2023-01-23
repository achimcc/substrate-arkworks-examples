mod mock;
use criterion::Criterion;
use frame_benchmarking::whitelisted_caller;
use frame_support::dispatch::RawOrigin;
use mock::{new_test_ext, AccountId, TemplateModule};

pub fn bench_msm_g1_bw6_761(c: &mut Criterion) {
	let mut group = c.benchmark_group("bench_msm_g1_bw6_761");
	let caller: AccountId = whitelisted_caller();
	group.bench_function("msm_g1_bw6_761_optimized", |b| {
		b.iter(|| {
			msm_g1_bw6_761_optimized(caller);
		});
	});
	group.bench_function("msm_g1_bw6_761", |b| {
		b.iter(|| {
			msm_g1_bw6_761(caller);
		});
	});
	group.finish();
}

pub fn bench_msm_g2_bw6_761(c: &mut Criterion) {
	let mut group = c.benchmark_group("bench_msm_g2_bw6_761");
	let caller: AccountId = whitelisted_caller();
	group.bench_function("msm_g2_bw6_761_optimized", |b| {
		b.iter(|| {
			msm_g2_bw6_761_optimized(caller);
		});
	});
	group.bench_function("msm_g2_bw6_761", |b| {
		b.iter(|| {
			msm_g2_bw6_761(caller);
		});
	});
	group.finish();
}

fn msm_g2_bw6_761_optimized(caller: AccountId) {
	new_test_ext().execute_with(|| {
		let _ = TemplateModule::msm_g2_bw6_761_optimized(RawOrigin::Signed(caller).into());
	});
}

fn msm_g2_bw6_761(caller: AccountId) {
	new_test_ext().execute_with(|| {
		let _ = TemplateModule::msm_g2_bw6_761(RawOrigin::Signed(caller).into());
	});
}

pub fn bench_mul_affine_g1_bw6_761(c: &mut Criterion) {
	let mut group = c.benchmark_group("bench_mul_affine_g1_bw6_761");
	let caller: AccountId = whitelisted_caller();
	group.bench_function("mul_affine_g1_bw6_761_optimized", |b| {
		b.iter(|| {
			mul_affine_g1_bw6_761_optimized(caller);
		});
	});
	group.bench_function("mul_affine_g1_bw6_761", |b| {
		b.iter(|| {
			mul_affine_g1_bw6_761(caller);
		});
	});
	group.finish();
}

fn mul_affine_g1_bw6_761_optimized(caller: AccountId) {
	new_test_ext().execute_with(|| {
		let _ = TemplateModule::mul_affine_g1_bw6_761_optimized(RawOrigin::Signed(caller).into());
	});
}

fn mul_affine_g1_bw6_761(caller: AccountId) {
	new_test_ext().execute_with(|| {
		let _ = TemplateModule::mul_affine_g1_bw6_761(RawOrigin::Signed(caller).into());
	});
}

pub fn bench_mul_projective_g1_bw6_761(c: &mut Criterion) {
	let mut group = c.benchmark_group("Fibonacci");
	let caller: AccountId = whitelisted_caller();
	group.bench_function("mul_projective_g1_bw6_761_optimized", |b| {
		b.iter(|| {
			mul_projective_g1_bw6_761_optimized(caller);
		});
	});
	group.bench_function("mul_projective_g1_bw6_761", |b| {
		b.iter(|| {
			mul_projective_g1_bw6_761(caller);
		});
	});
	group.finish();
}

fn mul_projective_g1_bw6_761_optimized(caller: AccountId) {
	new_test_ext().execute_with(|| {
		let _ =
			TemplateModule::mul_projective_g1_bw6_761_optimized(RawOrigin::Signed(caller).into());
	});
}

fn mul_projective_g1_bw6_761(caller: AccountId) {
	new_test_ext().execute_with(|| {
		let _ = TemplateModule::mul_projective_g1_bw6_761(RawOrigin::Signed(caller).into());
	});
}

pub fn bench_mul_affine_g2_bw6_761(c: &mut Criterion) {
	let mut group = c.benchmark_group("Fibonacci");
	let caller: AccountId = whitelisted_caller();
	group.bench_function("mul_affine_g2_bw6_761_optimized", |b| {
		b.iter(|| {
			mul_affine_g2_bw6_761_optimized(caller);
		});
	});
	group.bench_function("mul_affine_g2_bw6_761", |b| {
		b.iter(|| {
			mul_affine_g2_bw6_761(caller);
		});
	});
	group.finish();
}

fn mul_affine_g2_bw6_761_optimized(caller: AccountId) {
	new_test_ext().execute_with(|| {
		let _ = TemplateModule::mul_affine_g2_bw6_761_optimized(RawOrigin::Signed(caller).into());
	});
}

fn mul_affine_g2_bw6_761(caller: AccountId) {
	new_test_ext().execute_with(|| {
		let _ = TemplateModule::mul_affine_g2_bw6_761(RawOrigin::Signed(caller).into());
	});
}

pub fn bench_mul_projective_g2_bw6_761(c: &mut Criterion) {
	let mut group = c.benchmark_group("Fibonacci");
	let caller: AccountId = whitelisted_caller();
	group.bench_function("mul_projective_g2_bw6_761_optimized", |b| {
		b.iter(|| {
			mul_projective_g2_bw6_761_optimized(caller);
		});
	});
	group.bench_function("mul_projective_g2_bw6_761", |b| {
		b.iter(|| {
			mul_projective_g2_bw6_761(caller);
		});
	});
	group.finish();
}

fn mul_projective_g2_bw6_761_optimized(caller: AccountId) {
	new_test_ext().execute_with(|| {
		let _ =
			TemplateModule::mul_projective_g2_bw6_761_optimized(RawOrigin::Signed(caller).into());
	});
}

fn mul_projective_g2_bw6_761(caller: AccountId) {
	new_test_ext().execute_with(|| {
		let _ = TemplateModule::mul_projective_g2_bw6_761(RawOrigin::Signed(caller).into());
	});
}

pub fn bench_pairing_arkworks_bw6_761(c: &mut Criterion) {
	let mut group = c.benchmark_group("Fibonacci");
	let caller: AccountId = whitelisted_caller();
	group.bench_function("pairing_arkworks_bw6_761_optimized", |b| {
		b.iter(|| {
			pairing_arkworks_bw6_761_optimized(caller);
		});
	});
	group.bench_function("pairing_arkworks_bw6_761", |b| {
		b.iter(|| {
			pairing_arkworks_bw6_761(caller);
		});
	});
	group.finish();
}

fn pairing_arkworks_bw6_761_optimized(caller: AccountId) {
	new_test_ext().execute_with(|| {
		let _ =
			TemplateModule::pairing_arkworks_bw6_761_optimized(RawOrigin::Signed(caller).into());
	});
}

fn pairing_arkworks_bw6_761(caller: AccountId) {
	new_test_ext().execute_with(|| {
		let _ = TemplateModule::pairing_arkworks_bw6_761(RawOrigin::Signed(caller).into());
	});
}
