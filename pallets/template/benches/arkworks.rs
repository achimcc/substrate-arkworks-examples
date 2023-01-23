#![feature(custom_test_frameworks)]
#![test_runner(criterion::runner)]
#![feature(test)]
use criterion::{criterion_group, criterion_main, Criterion};
use frame_benchmarking::whitelisted_caller;
use frame_support::dispatch::RawOrigin;
type AccountId = u64;

use frame_support::traits::{ConstU16, ConstU64};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		TemplateModule: pallet_template,
	}
);

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_template::Config for Test {
	type RuntimeEvent = RuntimeEvent;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}

fn verify_groth16_optimized(c: &mut Criterion) {
	c.bench_function("verify_groth16_optimized", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ = TemplateModule::verify_groth16_optimized(RawOrigin::Signed(caller).into());
			});
		})
	});
}

fn pairing_arkworks_bls12_381_optimized(c: &mut Criterion) {
	c.bench_function("pairing_arkworks_bls12_381_optimized", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ = TemplateModule::pairing_arkworks_bls12_381_optimized(
					RawOrigin::Signed(caller).into(),
				);
			});
		})
	});
}

fn msm_g1_bls12_381_optimized(c: &mut Criterion) {
	c.bench_function("msm_g1_bls12_381_optimized", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ =
					TemplateModule::msm_g1_bls12_381_optimized(RawOrigin::Signed(caller).into());
			});
		})
	});
}

fn msm_g2_bls12_381_optimized(c: &mut Criterion) {
	c.bench_function("msm_g2_bls12_381_optimized", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ =
					TemplateModule::msm_g2_bls12_381_optimized(RawOrigin::Signed(caller).into());
			});
		})
	});
}

fn mul_affine_g1_bls12_381_optimized(c: &mut Criterion) {
	c.bench_function("fast_local_time", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ = TemplateModule::mul_affine_g1_bls12_381_optimized(
					RawOrigin::Signed(caller).into(),
				);
			});
		})
	});
}

fn mul_projective_g1_bls12_381_optimized(c: &mut Criterion) {
	c.bench_function("mul_projective_g1_bls12_381_optimized", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ = TemplateModule::mul_projective_g1_bls12_381_optimized(
					RawOrigin::Signed(caller).into(),
				);
			});
		})
	});
}

fn mul_affine_g2_bls12_381_optimized(c: &mut Criterion) {
	c.bench_function("mul_affine_g2_bls12_381_optimized", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ = TemplateModule::mul_affine_g2_bls12_381_optimized(
					RawOrigin::Signed(caller).into(),
				);
			});
		})
	});
}

fn mul_projective_g2_bls12_381_optimized(c: &mut Criterion) {
	c.bench_function("mul_projective_g2_bls12_381_optimized", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ = TemplateModule::mul_projective_g2_bls12_381_optimized(
					RawOrigin::Signed(caller).into(),
				);
			});
		})
	});
}

fn mul_affine_g1_bls12_377_optimized(c: &mut Criterion) {
	c.bench_function("mul_affine_g1_bls12_377_optimized", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ = TemplateModule::mul_affine_g1_bls12_377_optimized(
					RawOrigin::Signed(caller).into(),
				);
			});
		})
	});
}

fn mul_projective_g1_bls12_377_optimized(c: &mut Criterion) {
	c.bench_function("mul_projective_g1_bls12_377_optimized", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ = TemplateModule::mul_projective_g1_bls12_377_optimized(
					RawOrigin::Signed(caller).into(),
				);
			});
		})
	});
}

fn mul_affine_g2_bls12_377_optimized(c: &mut Criterion) {
	c.bench_function("mul_affine_g2_bls12_377_optimized", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ = TemplateModule::mul_affine_g2_bls12_377_optimized(
					RawOrigin::Signed(caller).into(),
				);
			});
		})
	});
}

fn mul_projective_g2_bls12_377_optimized(c: &mut Criterion) {
	c.bench_function("mul_projective_g2_bls12_377_optimized", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ = TemplateModule::mul_projective_g2_bls12_377_optimized(
					RawOrigin::Signed(caller).into(),
				);
			});
		})
	});
}

fn pairing_arkworks_bls12_377_optimized(c: &mut Criterion) {
	c.bench_function("pairing_arkworks_bls12_377_optimized", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ = TemplateModule::pairing_arkworks_bls12_377_optimized(
					RawOrigin::Signed(caller).into(),
				);
			});
		})
	});
}

fn msm_g2_bw6_761_optimized(c: &mut Criterion) {
	c.bench_function("msm_g2_bw6_761_optimized", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ = TemplateModule::msm_g2_bw6_761_optimized(RawOrigin::Signed(caller).into());
			});
		})
	});
}

fn mul_affine_g1_bw6_761_optimized(c: &mut Criterion) {
	c.bench_function("mul_affine_g1_bw6_761_optimized", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ = TemplateModule::mul_affine_g1_bw6_761_optimized(
					RawOrigin::Signed(caller).into(),
				);
			});
		})
	});
}

fn mul_projective_g1_bw6_761_optimized(c: &mut Criterion) {
	c.bench_function("mul_projective_g1_bw6_761_optimized", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ = TemplateModule::mul_projective_g1_bw6_761_optimized(
					RawOrigin::Signed(caller).into(),
				);
			});
		})
	});
}

fn mul_affine_g2_bw6_761_optimized(c: &mut Criterion) {
	c.bench_function("mul_affine_g2_bw6_761_optimized", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ = TemplateModule::mul_affine_g2_bw6_761_optimized(
					RawOrigin::Signed(caller).into(),
				);
			});
		})
	});
}

fn mul_projective_g2_bw6_761_optimized(c: &mut Criterion) {
	c.bench_function("mul_projective_g2_bw6_761_optimized", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ = TemplateModule::mul_projective_g2_bw6_761_optimized(
					RawOrigin::Signed(caller).into(),
				);
			});
		})
	});
}

fn pairing_arkworks_bw6_761_optimized(c: &mut Criterion) {
	c.bench_function("pairing_arkworks_bw6_761_optimized", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ = TemplateModule::pairing_arkworks_bw6_761_optimized(
					RawOrigin::Signed(caller).into(),
				);
			});
		})
	});
}

fn mul_affine_ed_on_bls12_377_optimized(c: &mut Criterion) {
	c.bench_function("mul_affine_ed_on_bls12_377_optimized", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ = TemplateModule::mul_affine_ed_on_bls12_377_optimized(
					RawOrigin::Signed(caller).into(),
				);
			});
		})
	});
}

fn mul_projective_ed_on_bls12_377_optimized(c: &mut Criterion) {
	c.bench_function("mul_projective_ed_on_bls12_377_optimized", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ = TemplateModule::mul_projective_ed_on_bls12_377_optimized(
					RawOrigin::Signed(caller).into(),
				);
			});
		})
	});
}

fn msm_ed_on_bls12_381_optimized(c: &mut Criterion) {
	c.bench_function("msm_ed_on_bls12_381_optimized", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ =
					TemplateModule::msm_ed_on_bls12_381_optimized(RawOrigin::Signed(caller).into());
			});
		})
	});
}

fn mul_affine_ed_on_bls12_381_optimized(c: &mut Criterion) {
	c.bench_function("mul_affine_ed_on_bls12_381_optimized", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ = TemplateModule::mul_affine_ed_on_bls12_381_optimized(
					RawOrigin::Signed(caller).into(),
				);
			});
		})
	});
}

fn mul_projective_ed_on_bls12_381_optimized(c: &mut Criterion) {
	c.bench_function("mul_projective_ed_on_bls12_381_optimized", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ = TemplateModule::mul_projective_ed_on_bls12_381_optimized(
					RawOrigin::Signed(caller).into(),
				);
			});
		})
	});
}

fn pairing_arkworks_bls12_381(c: &mut Criterion) {
	c.bench_function("pairing_arkworks_bls12_381", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ =
					TemplateModule::pairing_arkworks_bls12_381(RawOrigin::Signed(caller).into());
			});
		})
	});
}

fn msm_g1_bls12_381(c: &mut Criterion) {
	c.bench_function("msm_g1_bls12_381", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ = TemplateModule::msm_g1_bls12_381(RawOrigin::Signed(caller).into());
			});
		})
	});
}

fn msm_g2_bls12_381(c: &mut Criterion) {
	c.bench_function("msm_g2_bls12_381", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ = TemplateModule::msm_g2_bls12_381(RawOrigin::Signed(caller).into());
			});
		})
	});
}

fn mul_affine_g1_bls12_381(c: &mut Criterion) {
	c.bench_function("mul_affine_g1_bls12_381", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ = TemplateModule::mul_affine_g1_bls12_381(RawOrigin::Signed(caller).into());
			});
		})
	});
}

fn mul_projective_g1_bls12_381(c: &mut Criterion) {
	c.bench_function("mul_projective_g1_bls12_381", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ =
					TemplateModule::mul_projective_g1_bls12_381(RawOrigin::Signed(caller).into());
			});
		})
	});
}

fn mul_affine_g2_bls12_381(c: &mut Criterion) {
	c.bench_function("mul_affine_g2_bls12_381", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ = TemplateModule::mul_affine_g2_bls12_381(RawOrigin::Signed(caller).into());
			});
		})
	});
}

fn mul_projective_g2_bls12_381(c: &mut Criterion) {
	c.bench_function("mul_projective_g2_bls12_381", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ =
					TemplateModule::mul_projective_g2_bls12_381(RawOrigin::Signed(caller).into());
			});
		})
	});
}

fn mul_projective_g1_bls12_377(c: &mut Criterion) {
	c.bench_function("mul_projective_g1_bls12_377", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ =
					TemplateModule::mul_projective_g1_bls12_377(RawOrigin::Signed(caller).into());
			});
		})
	});
}

fn mul_affine_g1_bls12_377(c: &mut Criterion) {
	c.bench_function("mul_affine_g1_bls12_377", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ = TemplateModule::mul_affine_g1_bls12_377(RawOrigin::Signed(caller).into());
			});
		})
	});
}

fn mul_affine_g2_bls12_377(c: &mut Criterion) {
	c.bench_function("mul_affine_g2_bls12_377", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ = TemplateModule::mul_affine_g2_bls12_377(RawOrigin::Signed(caller).into());
			});
		})
	});
}

fn mul_projective_g2_bls12_377(c: &mut Criterion) {
	c.bench_function("mul_projective_g2_bls12_377", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ =
					TemplateModule::mul_projective_g2_bls12_377(RawOrigin::Signed(caller).into());
			});
		})
	});
}

fn pairing_arkworks_bls12_377(c: &mut Criterion) {
	c.bench_function("pairing_arkworks_bls12_377", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ =
					TemplateModule::pairing_arkworks_bls12_377(RawOrigin::Signed(caller).into());
			});
		})
	});
}

fn msm_g2_bw6_761(c: &mut Criterion) {
	c.bench_function("msm_g2_bw6_761", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ = TemplateModule::msm_g2_bw6_761(RawOrigin::Signed(caller).into());
			});
		})
	});
}

fn mul_affine_g1_bw6_761(c: &mut Criterion) {
	c.bench_function("mul_affine_g1_bw6_761", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ = TemplateModule::mul_affine_g1_bw6_761(RawOrigin::Signed(caller).into());
			});
		})
	});
}

fn mul_projective_g1_bw6_761(c: &mut Criterion) {
	c.bench_function("mul_projective_g1_bw6_761", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ = TemplateModule::mul_projective_g1_bw6_761(RawOrigin::Signed(caller).into());
			});
		})
	});
}

fn mul_affine_g2_bw6_761(c: &mut Criterion) {
	c.bench_function("mul_affine_g2_bw6_761", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ = TemplateModule::mul_affine_g2_bw6_761(RawOrigin::Signed(caller).into());
			});
		})
	});
}

fn mul_projective_g2_bw6_761(c: &mut Criterion) {
	c.bench_function("mul_projective_g2_bw6_761", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ = TemplateModule::mul_projective_g2_bw6_761(RawOrigin::Signed(caller).into());
			});
		})
	});
}

fn msm_ed_on_bls12_377(c: &mut Criterion) {
	c.bench_function("msm_ed_on_bls12_377", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ = TemplateModule::msm_ed_on_bls12_377(RawOrigin::Signed(caller).into());
			});
		})
	});
}

fn mul_affine_ed_on_bls12_377(c: &mut Criterion) {
	c.bench_function("mul_affine_ed_on_bls12_377", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ =
					TemplateModule::mul_affine_ed_on_bls12_377(RawOrigin::Signed(caller).into());
			});
		})
	});
}

fn mul_projective_ed_on_bls12_377(c: &mut Criterion) {
	c.bench_function("mul_projective_ed_on_bls12_377", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ = TemplateModule::mul_projective_ed_on_bls12_377(
					RawOrigin::Signed(caller).into(),
				);
			});
		})
	});
}

fn msm_ed_on_bls12_381(c: &mut Criterion) {
	c.bench_function("msm_ed_on_bls12_381", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ = TemplateModule::msm_ed_on_bls12_381(RawOrigin::Signed(caller).into());
			});
		})
	});
}

fn mul_affine_ed_on_bls12_381(c: &mut Criterion) {
	c.bench_function("mul_affine_ed_on_bls12_381", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ =
					TemplateModule::mul_affine_ed_on_bls12_381(RawOrigin::Signed(caller).into());
			});
		})
	});
}

fn mul_projective_ed_on_bls12_381(c: &mut Criterion) {
	c.bench_function("mul_projective_ed_on_bls12_381", |b| {
		let caller: AccountId = whitelisted_caller();
		b.iter(|| {
			new_test_ext().execute_with(|| {
				let _ = TemplateModule::mul_projective_ed_on_bls12_381(
					RawOrigin::Signed(caller).into(),
				);
			});
		})
	});
}

criterion_group! {
	name = arkwrorks;
	config = Criterion::default();
	targets =
	  pairing_arkworks_bls12_381, msm_g1_bls12_381, msm_g2_bls12_381, mul_affine_g1_bls12_381,
	  mul_projective_g1_bls12_381, mul_affine_g2_bls12_381, mul_projective_g2_bls12_381,
	  mul_projective_g1_bls12_377, mul_affine_g1_bls12_377, mul_affine_g2_bls12_377,
	  mul_projective_g2_bls12_377, pairing_arkworks_bls12_377, msm_g2_bw6_761,
	  mul_affine_g1_bw6_761, mul_projective_g1_bw6_761, mul_affine_g2_bw6_761,
	  mul_projective_g2_bw6_761, msm_ed_on_bls12_377, mul_affine_ed_on_bls12_377,
	  mul_projective_ed_on_bls12_377, msm_ed_on_bls12_381, mul_affine_ed_on_bls12_381,
	  mul_projective_ed_on_bls12_381
}

criterion_group! {
	name = arkwork_optimized;
	config = Criterion::default();
	targets = verify_groth16_optimized, pairing_arkworks_bls12_381_optimized,
	  msm_g1_bls12_381_optimized, msm_g2_bls12_381_optimized, mul_affine_g1_bls12_381_optimized,
	  mul_projective_g1_bls12_381_optimized, mul_affine_g2_bls12_381_optimized,
	  mul_projective_g2_bls12_381_optimized, mul_affine_g1_bls12_377_optimized,
	  mul_projective_g1_bls12_377_optimized, mul_affine_g2_bls12_377_optimized,
	  mul_projective_g2_bls12_377_optimized, pairing_arkworks_bls12_377_optimized,
	  msm_g2_bw6_761_optimized, mul_affine_g1_bw6_761_optimized, mul_projective_g1_bw6_761_optimized,
	  mul_affine_g2_bw6_761_optimized, mul_projective_g2_bw6_761_optimized,
	  pairing_arkworks_bw6_761_optimized, mul_affine_ed_on_bls12_377_optimized,
	  mul_projective_ed_on_bls12_377_optimized, msm_ed_on_bls12_381_optimized,
	  mul_affine_ed_on_bls12_381_optimized, mul_projective_ed_on_bls12_381_optimized,
}
criterion_main!(arkworks, arkworks_optimized);
