use ark_std::{io::Error, test_rng, UniformRand};
use criterion::Criterion;
use frame_benchmarking::whitelisted_caller;
use frame_support::dispatch::RawOrigin;
use sp_ark_bls12_381::{
	Bls12_381 as Bls12_381_Host, Fr as BlsFrOptimized, G1Affine as G1AffineBls12_381_Host,
	G1Projective as G1ProjectiveBls12_381_Host, G2Affine as G2AffineBls12_381_Host,
	G2Projective as G2ProjectiveBls12_381_Host, HostFunctions as Bls12_381HostFunctions,
};
use sp_ark_models::{
	pairing::Pairing, short_weierstrass::SWCurveConfig, AffineRepr, Group, TECurveConfig,
};

type AccountId = u64;

struct HostBls12_381 {}

impl Bls12_381HostFunctions for HostBls12_381 {
	fn bls12_381_multi_miller_loop(a: Vec<Vec<u8>>, b: Vec<Vec<u8>>) -> Vec<u8> {
		sp_io::elliptic_curves::bls12_381_multi_miller_loop(a, b)
	}
	fn bls12_381_final_exponentiation(f12: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::bls12_381_final_exponentiation(f12)
	}
	fn bls12_381_msm_g1(bases: Vec<Vec<u8>>, bigints: Vec<Vec<u8>>) -> Vec<u8> {
		sp_io::elliptic_curves::bls12_381_msm_g1(bases, bigints)
	}
	fn bls12_381_mul_projective_g1(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::bls12_381_mul_projective_g1(base, scalar)
	}
	fn bls12_381_mul_affine_g1(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::bls12_381_mul_affine_g1(base, scalar)
	}
	fn bls12_381_msm_g2(bases: Vec<Vec<u8>>, bigints: Vec<Vec<u8>>) -> Vec<u8> {
		sp_io::elliptic_curves::bls12_381_msm_g2(bases, bigints)
	}
	fn bls12_381_mul_projective_g2(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::bls12_381_mul_projective_g2(base, scalar)
	}
	fn bls12_381_mul_affine_g2(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::bls12_381_mul_affine_g2(base, scalar)
	}
}

type Bls12_381Optimized = Bls12_381_Host<HostBls12_381>;
type G1AffineBls12_381 = G1AffineBls12_381_Host<HostBls12_381>;
type G2AffineBls12_381 = G2AffineBls12_381_Host<HostBls12_381>;
type G1ProjectiveBls12_381 = G1ProjectiveBls12_381_Host<HostBls12_381>;
type G2ProjectiveBls12_381 = G2ProjectiveBls12_381_Host<HostBls12_381>;

pub fn bench_groth16(c: &mut Criterion) {
	let mut group = c.benchmark_group("groth16");
	group.bench_function("verify_groth16_optimized", |b| {
		b.iter(|| {
			let _ = do_verify_groth16_optimized(caller);
		});
	});
	group.bench_function("verify_groth16", |b| {
		b.iter(|| {
			let _ = do_verify_groth16(caller);
		});
	});
	group.finish();
}

pub fn do_verify_groth16() -> Result<(), Error> {
	let vk = <Groth16<Bls12_381> as SNARK<BlsFr>>::VerifyingKey::deserialize_with_mode(
		VK_SERIALIZED,
		Compress::Yes,
		Validate::No,
	)
	.unwrap();

	let c = Fp::deserialize_with_mode(C_SERIALIZED, Compress::Yes, Validate::No).unwrap();

	let proof = <Groth16<Bls12_381> as SNARK<BlsFr>>::Proof::deserialize_with_mode(
		PROOF_SERIALIZED,
		Compress::Yes,
		Validate::No,
	)
	.unwrap();

	Groth16::<Bls12_381>::verify(&vk, &[c], &proof)
}

pub fn do_verify_groth16_optimized() -> Result<(), Error> {
	let vk = <Groth16<Bls12_381Optimized> as SNARK<BlsFrOptimized>>::VerifyingKey::deserialize_with_mode(
				VK_SERIALIZED,
				Compress::Yes,
				Validate::No,
			)
			.unwrap();

	let c = Fp::deserialize_with_mode(C_SERIALIZED, Compress::Yes, Validate::No).unwrap();

	let proof =
		<Groth16<Bls12_381Optimized> as SNARK<BlsFrOptimized>>::Proof::deserialize_with_mode(
			PROOF_SERIALIZED,
			Compress::Yes,
			Validate::No,
		)
		.unwrap();

	Groth16::<Bls12_381Optimized>::verify(&vk, &[c], &proof)
}

pub fn bench_pairing_arkworks_bls12_381(c: &mut Criterion) {
	let mut group = c.benchmark_group("pairing_arkworks_bls12_381");
	let caller: AccountId = whitelisted_caller();
	group.bench_function("pairing_arkworks_bls12_381_optimized", |b| {
		b.iter(|| {
			let _ = do_pairing_arkworks_bls12_381_optimized();
		});
	});
	group.bench_function("pairing_arkworks_bls12_381", |b| {
		b.iter(|| {
			let _ = do_pairing_arkworks_bls12_381();
		});
	});
	group.finish();
}

fn do_pairing_arkworks_bls12_381_optimized() -> Result<(), Error> {
	let _ = Bls12_381Optimized::multi_pairing(
		[G1AffineBls12_381::generator()],
		[G2AffineBls12_381::generator()],
	);
	Ok(())
}

fn do_pairing_arkworks_bls12_381() -> Result<(), Error> {
	let _ = ark_bls12_381::Bls12_381::multi_pairing(
		[ark_bls12_381::G1Affine::generator()],
		[ark_bls12_381::G2Affine::generator()],
	);
	Ok(())
}

pub fn bench_msm_g1_bls12_381(c: &mut Criterion) {
	let mut group = c.benchmark_group("msm_g1_bls12_381");
	let caller: AccountId = whitelisted_caller();
	group.bench_function("msm_g1_bls12_381_optimized", |b| {
		b.iter(|| {
			let _ = do_msm_g1_bls12_381_optimized();
		});
	});
	group.bench_function("msm_g1_bls12_381", |b| {
		b.iter(|| {
			let _ = do_msm_g1_bls12_381();
		});
	});
	group.finish();
}

fn do_msm_g1_bls12_381_optimized() -> Result<(), Error> {
	let mut rng = test_rng();
	let scalar = sp_ark_bls12_381::Fr::rand(&mut rng);
	let _out = <sp_ark_bls12_381::g1::Config<HostBls12_381> as SWCurveConfig>::msm(
		&[G1AffineBls12_381::generator()],
		&[scalar],
	);
	Ok(())
}

fn do_msm_g1_bls12_381() -> Result<(), Error> {
	let mut rng = test_rng();
	let scalar = ark_bls12_381::Fr::rand(&mut rng);
	let _out = <ark_bls12_381::g1::Config as SWCurveConfig>::msm(
		&[ark_bls12_381::G1Affine::generator()],
		&[scalar],
	);
	Ok(())
}

pub fn bench_msm_g2_bls12_381(c: &mut Criterion) {
	let mut group = c.benchmark_group("msm_g2_bls12_381");
	let caller: AccountId = whitelisted_caller();
	group.bench_function("msm_g2_bls12_381_optimized", |b| {
		b.iter(|| {
			let _ = do_msm_g2_bls12_381_optimized();
		});
	});
	group.bench_function("msm_g2_bls12_381", |b| {
		b.iter(|| {
			let _ = do_msm_g2_bls12_381();
		});
	});
	group.finish();
}

fn do_msm_g2_bls12_381_optimized() -> Result<(), Error> {
	let mut rng = test_rng();
	let scalar = sp_ark_bls12_381::Fr::rand(&mut rng);
	let _out = <sp_ark_bls12_381::g2::Config<HostBls12_381> as SWCurveConfig>::msm(
		&[G2AffineBls12_381::generator()],
		&[scalar],
	);
	Ok(())
}

fn do_msm_g2_bls12_381() -> Result<(), Error> {
	let mut rng = test_rng();
	let scalar = ark_bls12_381::Fr::rand(&mut rng);
	let _out = <ark_bls12_381::g2::Config as SWCurveConfig>::msm(
		&[ark_bls12_381::G2Affine::generator()],
		&[scalar],
	);
	Ok(())
}

pub fn bench_mul_affine_g1_bls12_381(c: &mut Criterion) {
	let mut group = c.benchmark_group("mul_affine_g1_bls12_381");
	let caller: AccountId = whitelisted_caller();
	group.bench_function("mul_affine_g1_bls12_381_optimized", |b| {
		b.iter(|| {
			let _ = do_mul_affine_g1_bls12_381_optimized();
		});
	});
	group.bench_function("mul_affine_g1_bls12_381", |b| {
		b.iter(|| {
			let _ = do_mul_affine_g1_bls12_381();
		});
	});
	group.finish();
}

fn do_mul_affine_g1_bls12_381_optimized() -> Result<(), Error> {
	let _out = <sp_ark_bls12_381::g1::Config<HostBls12_381> as SWCurveConfig>::mul_affine(
		&G1AffineBls12_381::generator(),
		&[2u64],
	);
	Ok(())
}

fn do_mul_affine_g1_bls12_381() -> Result<(), Error> {
	let _out = <ark_bls12_381::g1::Config as SWCurveConfig>::mul_affine(
		&ark_bls12_381::G1Affine::generator(),
		&[2u64],
	);
	Ok(())
}

pub fn bench_mul_projective_g1_bls12_381(c: &mut Criterion) {
	let mut group = c.benchmark_group("mul_projective_g1_bls12_381");
	let caller: AccountId = whitelisted_caller();
	group.bench_function("mul_projective_g1_bls12_381_optimized", |b| {
		b.iter(|| {
			let _ = do_mul_projective_g1_bls12_381_optimized();
		});
	});
	group.bench_function("mul_projective_g1_bls12_381", |b| {
		b.iter(|| {
			let _ = do_mul_projective_g1_bls12_381();
		});
	});
	group.finish();
}

fn do_mul_projective_g1_bls12_381_optimized() -> Result<(), Error> {
	let _out = <sp_ark_bls12_381::g1::Config<HostBls12_381> as SWCurveConfig>::mul_projective(
		&G1ProjectiveBls12_381::generator(),
		&[2u64],
	);
	Ok(())
}

fn do_mul_projective_g1_bls12_381() -> Result<(), Error> {
	let _out = <ark_bls12_381::g1::Config as SWCurveConfig>::mul_projective(
		&ark_bls12_381::G1Projective::generator(),
		&[2u64],
	);
	Ok(())
}

pub fn bench_mul_affine_g2_bls12_381(c: &mut Criterion) {
	let mut group = c.benchmark_group("mul_affine_g2_bls12_381");
	let caller: AccountId = whitelisted_caller();
	group.bench_function("mul_affine_g2_bls12_381_optimized", |b| {
		b.iter(|| {
			let _ = do_mul_affine_g2_bls12_381_optimized();
		});
	});
	group.bench_function("mul_affine_g2_bls12_381", |b| {
		b.iter(|| {
			let _ = do_mul_affine_g2_bls12_381();
		});
	});
	group.finish();
}

fn do_mul_affine_g2_bls12_381_optimized() -> Result<(), Error> {
	let _out = <sp_ark_bls12_381::g2::Config<HostBls12_381> as SWCurveConfig>::mul_affine(
		&G2AffineBls12_381::generator(),
		&[2u64],
	);
	Ok(())
}

fn do_mul_affine_g2_bls12_381() -> Result<(), Error> {
	let _out = <ark_bls12_381::g2::Config as SWCurveConfig>::mul_affine(
		&ark_bls12_381::G2Affine::generator(),
		&[2u64],
	);
	Ok(())
}

pub fn bench_mul_projective_g2_bls12_381(c: &mut Criterion) {
	let mut group = c.benchmark_group("mul_projective_g2_bls12_381");
	let caller: AccountId = whitelisted_caller();
	group.bench_function("mul_projective_g2_bls12_381_optimized", |b| {
		b.iter(|| {
			let _ = do_mul_projective_g2_bls12_381_optimized();
		});
	});
	group.bench_function("mul_projective_g2_bls12_381", |b| {
		b.iter(|| {
			let _ = do_mul_projective_g2_bls12_381();
		});
	});
	group.finish();
}

fn do_mul_projective_g2_bls12_381_optimized() -> Result<(), Error> {
	let _out = <sp_ark_bls12_381::g2::Config<HostBls12_381> as SWCurveConfig>::mul_projective(
		&G2ProjectiveBls12_381::generator(),
		&[2u64],
	);
	Ok(())
}

fn do_mul_projective_g2_bls12_381() -> Result<(), Error> {
	let _out = <ark_bls12_381::g2::Config as SWCurveConfig>::mul_projective(
		&ark_bls12_381::G2Projective::generator(),
		&[2u64],
	);
	Ok(())
}
