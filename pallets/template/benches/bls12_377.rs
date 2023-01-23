use criterion::Criterion;
use frame_benchmarking::whitelisted_caller;
use frame_support::dispatch::RawOrigin;
mod mock;
use mock::{new_test_ext, AccountId, TemplateModule};

pub fn bench_mul_affine_g1_bls12_377(c: &mut Criterion) {
	let mut group = c.benchmark_group("bench_mul_affine_g1_bls12_377");
	let caller: AccountId = whitelisted_caller();
	group.bench_function("mul_affine_g1_bls12_377_optimized", |b| {
		b.iter(|| {
			mul_affine_g1_bls12_377_optimized(caller);
		});
	});
	group.bench_function("mul_affine_g1_bls12_377", |b| {
		b.iter(|| {
			mul_affine_g1_bls12_377(caller);
		});
	});
	group.finish();
}

fn mul_affine_g1_bls12_377_optimized(caller: AccountId) {
	new_test_ext().execute_with(|| {
		let _ = TemplateModule::mul_affine_g1_bls12_377_optimized(RawOrigin::Signed(caller).into());
	});
}

fn mul_affine_g1_bls12_377(caller: AccountId) {
	new_test_ext().execute_with(|| {
		let _ = TemplateModule::mul_affine_g1_bls12_377(RawOrigin::Signed(caller).into());
	});
}

pub fn bench_mul_projective_g1_bls12_377(c: &mut Criterion) {
	let mut group = c.benchmark_group("bench_mul_projective_g1_bls12_377");
	let caller: AccountId = whitelisted_caller();
	group.bench_function("mul_projective_g1_bls12_377_optimized", |b| {
		b.iter(|| {
			mul_projective_g1_bls12_377_optimized(caller);
		});
	});
	group.bench_function("mul_projective_g1_bls12_377", |b| {
		b.iter(|| {
			mul_projective_g1_bls12_377(caller);
		});
	});
	group.finish();
}

fn mul_projective_g1_bls12_377_optimized(caller: AccountId) {
	new_test_ext().execute_with(|| {
		let _ =
			TemplateModule::mul_projective_g1_bls12_377_optimized(RawOrigin::Signed(caller).into());
	});
}

fn mul_projective_g1_bls12_377(caller: AccountId) {
	new_test_ext().execute_with(|| {
		let _ = TemplateModule::mul_projective_g1_bls12_377(RawOrigin::Signed(caller).into());
	});
}

pub fn bench_mul_affine_g2_bls12_377(c: &mut Criterion) {
	let mut group = c.benchmark_group("bench_mul_affine_g2_bls12_377");
	let caller: AccountId = whitelisted_caller();
	group.bench_function("mul_affine_g2_bls12_377_optimized", |b| {
		b.iter(|| {
			mul_affine_g2_bls12_377_optimized(caller);
		});
	});
	group.bench_function("mul_affine_g2_bls12_377", |b| {
		b.iter(|| {
			mul_affine_g2_bls12_377(caller);
		});
	});
	group.finish();
}

fn mul_affine_g2_bls12_377_optimized(caller: AccountId) {
	new_test_ext().execute_with(|| {
		let _ = TemplateModule::mul_affine_g2_bls12_377_optimized(RawOrigin::Signed(caller).into());
	});
}

fn mul_affine_g2_bls12_377(caller: AccountId) {
	new_test_ext().execute_with(|| {
		let _ = TemplateModule::mul_affine_g2_bls12_377(RawOrigin::Signed(caller).into());
	});
}

pub fn bench_mul_projective_g2_bls12_377(c: &mut Criterion) {
	let mut group = c.benchmark_group("Fibonacci");
	let caller: AccountId = whitelisted_caller();
	group.bench_function("mul_projective_g2_bls12_377_optimized", |b| {
		b.iter(|| {
			mul_projective_g2_bls12_377_optimized(caller);
		});
	});
	group.bench_function("mul_projective_g2_bls12_377", |b| {
		b.iter(|| {
			mul_projective_g2_bls12_377(caller);
		});
	});
	group.finish();
}

fn mul_projective_g2_bls12_377_optimized(caller: AccountId) {
	new_test_ext().execute_with(|| {
		let _ =
			TemplateModule::mul_projective_g2_bls12_377_optimized(RawOrigin::Signed(caller).into());
	});
}

fn mul_projective_g2_bls12_377(caller: AccountId) {
	new_test_ext().execute_with(|| {
		let _ = TemplateModule::mul_projective_g2_bls12_377(RawOrigin::Signed(caller).into());
	});
}

pub fn bench_pairing_arkworks_bls12_377(c: &mut Criterion) {
	let mut group = c.benchmark_group("Fibonacci");
	let caller: AccountId = whitelisted_caller();
	group.bench_function("pairing_arkworks_bls12_377_optimized", |b| {
		b.iter(|| {
			pairing_arkworks_bls12_377_optimized(caller);
		});
	});
	group.bench_function("pairing_arkworks_bls12_377", |b| {
		b.iter(|| {
			pairing_arkworks_bls12_377(caller);
		});
	});
	group.finish();
}

fn pairing_arkworks_bls12_377_optimized(caller: AccountId) {
	new_test_ext().execute_with(|| {
		let _ =
			TemplateModule::pairing_arkworks_bls12_377_optimized(RawOrigin::Signed(caller).into());
	});
}

fn pairing_arkworks_bls12_377(caller: AccountId) {
	new_test_ext().execute_with(|| {
		let _ = TemplateModule::pairing_arkworks_bls12_377(RawOrigin::Signed(caller).into());
	});
}
