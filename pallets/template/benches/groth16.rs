use criterion::Criterion;
use frame_benchmarking::whitelisted_caller;
use frame_support::dispatch::RawOrigin;
mod mock;
use mock::{new_test_ext, AccountId, TemplateModule};

pub fn bench_groth16(c: &mut Criterion) {
	let mut group = c.benchmark_group("Fibonacci");
	let caller: AccountId = whitelisted_caller();
	group.bench_function("verify_groth16_optimized", |b| {
		b.iter(|| {
			verify_groth16_optimized(caller);
		});
	});
	group.bench_function("verify_groth16", |b| {
		b.iter(|| {
			verify_groth16(caller);
		});
	});
	group.finish();
}

fn verify_groth16_optimized(caller: AccountId) {
	new_test_ext().execute_with(|| {
		let _ = TemplateModule::verify_groth16_optimized(RawOrigin::Signed(caller).into());
	});
}

fn verify_groth16(caller: AccountId) {
	new_test_ext().execute_with(|| {
		let _ = TemplateModule::verify_groth16(RawOrigin::Signed(caller).into());
	});
}
