use ark_serialize::{CanonicalDeserialize, Compress, Validate};
use ark_std::{io::Error, test_rng, UniformRand};
use sp_ark_bw6_761::{
	G1Affine as G1AffineBW6_761_Host, G1Projective as G1ProjectiveBW6_761_Host,
	G2Affine as G2AffineBW6_761_Host, G2Projective as G2ProjectiveBW6_761_Host,
	HostFunctions as BW6_761HostFunctions, BW6_761 as BW6_761_Host,
};
use sp_ark_models::{
	pairing::Pairing, short_weierstrass::SWCurveConfig, AffineRepr, Group, TECurveConfig,
};

type AccountId = u64;

struct HostBW6_761 {}

impl BW6_761HostFunctions for HostBW6_761 {
	fn bw6_761_multi_miller_loop(a: Vec<Vec<u8>>, b: Vec<Vec<u8>>) -> Vec<u8> {
		sp_io::elliptic_curves::bw6_761_multi_miller_loop(a, b)
	}
	fn bw6_761_final_exponentiation(f12: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::bw6_761_final_exponentiation(f12)
	}
	fn bw6_761_mul_projective_g2(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::bw6_761_mul_projective_g2(base, scalar)
	}
	fn bw6_761_mul_affine_g2(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::bw6_761_mul_affine_g2(base, scalar)
	}
	fn bw6_761_mul_projective_g1(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::bw6_761_mul_projective_g1(base, scalar)
	}
	fn bw6_761_mul_affine_g1(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::bw6_761_mul_affine_g1(base, scalar)
	}
	fn bw6_761_msm_g1(bases: Vec<Vec<u8>>, bigints: Vec<Vec<u8>>) -> Vec<u8> {
		sp_io::elliptic_curves::bw6_761_msm_g1(bases, bigints)
	}
	fn bw6_761_msm_g2(bases: Vec<Vec<u8>>, bigints: Vec<Vec<u8>>) -> Vec<u8> {
		sp_io::elliptic_curves::bw6_761_msm_g2(bases, bigints)
	}
}

type BW6_761Optimized = BW6_761_Host<HostBW6_761>;
type G1AffineBW6_761 = G1AffineBW6_761_Host<HostBW6_761>;
type G2AffineBW6_761 = G2AffineBW6_761_Host<HostBW6_761>;
type G1ProjectiveBW6_761 = G1ProjectiveBW6_761_Host<HostBW6_761>;
type G2ProjectiveBW6_761 = G2ProjectiveBW6_761_Host<HostBW6_761>;

pub fn bench_msm_g1_bw6_761(c: &mut Criterion) {
	let mut group = c.benchmark_group("msm_g1_bw6_761");
	let caller: AccountId = whitelisted_caller();
	group.bench_function("msm_g1_bw6_761_optimized", |b| {
		b.iter(|| {
			let _ = do_msm_g1_bw6_761_optimized();
		});
	});
	group.bench_function("msm_g1_bw6_761", |b| {
		b.iter(|| {
			let _ = do_msm_g1_bw6_761();
		});
	});
	group.finish();
}

fn do_msm_g1_bw6_761_optimized() -> Result<(), Error> {
	let _out = <sp_ark_bw6_761::g1::Config<HostBW6_761> as SWCurveConfig>::msm(
		&[G1AffineBW6_761::generator()],
		&[2u64.into()],
	);
	Ok(())
}

fn do_msm_g1_bw6_761() -> Result<(), Error> {
	let _out = <ark_bw6_761::g1::Config as SWCurveConfig>::msm(
		&[ark_bw6_761::G1Affine::generator()],
		&[2u64.into()],
	);

	Ok(())
}

pub fn bench_msm_g2_bw6_761(c: &mut Criterion) {
	let mut group = c.benchmark_group("msm_g2_bw6_761");
	let caller: AccountId = whitelisted_caller();
	group.bench_function("msm_g2_bw6_761_optimized", |b| {
		b.iter(|| {
			let _ = do_msm_g2_bw6_761_optimized();
		});
	});
	group.bench_function("msm_g2_bw6_761", |b| {
		b.iter(|| {
			let _ = do_msm_g2_bw6_761();
		});
	});
	group.finish();
}

fn do_msm_g2_bw6_761_optimized() -> Result<(), Error> {
	let _out = <sp_ark_bw6_761::g2::Config<HostBW6_761> as SWCurveConfig>::msm(
		&[G2AffineBW6_761::generator()],
		&[2u64.into()],
	);

	Ok(())
}

fn do_msm_g2_bw6_761() -> Result<(), Error> {
	let _out = <ark_bw6_761::g2::Config as SWCurveConfig>::msm(
		&[ark_bw6_761::G2Affine::generator()],
		&[2u64.into()],
	);

	Ok(())
}

pub fn bench_mul_affine_g1_bw6_761(c: &mut Criterion) {
	let mut group = c.benchmark_group("mul_affine_g1_bw6_761");
	let caller: AccountId = whitelisted_caller();
	group.bench_function("mul_affine_g1_bw6_761_optimized", |b| {
		b.iter(|| {
			let _ = do_mul_affine_g1_bw6_761_optimized();
		});
	});
	group.bench_function("mul_affine_g1_bw6_761", |b| {
		b.iter(|| {
			let _ = do_mul_affine_g1_bw6_761();
		});
	});
	group.finish();
}

fn do_mul_affine_g1_bw6_761_optimized() -> Result<(), Error> {
	let _out = <sp_ark_bw6_761::g1::Config<HostBW6_761> as SWCurveConfig>::mul_affine(
		&G1AffineBW6_761::generator(),
		&[2u64],
	);
	Ok(())
}

fn do_mul_affine_g1_bw6_761() -> Result<(), Error> {
	let _out = <ark_bw6_761::g1::Config as SWCurveConfig>::mul_affine(
		&ark_bw6_761::G1Affine::generator(),
		&[2u64],
	);
	Ok(())
}

pub fn bench_mul_projective_g1_bw6_761(c: &mut Criterion) {
	let mut group = c.benchmark_group("mul_projective_g1_bw6_761");
	let caller: AccountId = whitelisted_caller();
	group.bench_function("mul_projective_g1_bw6_761_optimized", |b| {
		b.iter(|| {
			let _ = do_mul_projective_g1_bw6_761_optimized();
		});
	});
	group.bench_function("mul_projective_g1_bw6_761", |b| {
		b.iter(|| {
			let _ = do_mul_projective_g1_bw6_761();
		});
	});
	group.finish();
}

fn do_mul_projective_g1_bw6_761_optimized() -> Result<(), Error> {
	let _out = <sp_ark_bw6_761::g1::Config<HostBW6_761> as SWCurveConfig>::mul_projective(
		&G1ProjectiveBW6_761::generator(),
		&[2u64],
	);
	Ok(())
}

fn do_mul_projective_g1_bw6_761() -> Result<(), Error> {
	let _out = <ark_bw6_761::g1::Config as SWCurveConfig>::mul_projective(
		&ark_bw6_761::G1Projective::generator(),
		&[2u64],
	);
	Ok(())
}

pub fn bench_mul_affine_g2_bw6_761(c: &mut Criterion) {
	let mut group = c.benchmark_group("mul_affine_g2_bw6_761");
	let caller: AccountId = whitelisted_caller();
	group.bench_function("mul_affine_g2_bw6_761_optimized", |b| {
		b.iter(|| {
			let _ = do_mul_affine_g2_bw6_761_optimized();
		});
	});
	group.bench_function("mul_affine_g2_bw6_761", |b| {
		b.iter(|| {
			let _ = do_mul_affine_g2_bw6_761();
		});
	});
	group.finish();
}

fn do_mul_affine_g2_bw6_761_optimized() -> Result<(), Error> {
	let _out = <sp_ark_bw6_761::g2::Config<HostBW6_761> as SWCurveConfig>::mul_affine(
		&G2AffineBW6_761::generator(),
		&[2u64],
	);
	Ok(())
}

fn do_mul_affine_g2_bw6_761() -> Result<(), Error> {
	let _out = <sp_ark_bw6_761::g1::Config<HostBW6_761> as SWCurveConfig>::mul_projective(
		&G1ProjectiveBW6_761::generator(),
		&[2u64],
	);
	Ok(())
}

pub fn bench_mul_projective_g2_bw6_761(c: &mut Criterion) {
	let mut group = c.benchmark_group("mul_projective_g2_bw6_761");
	let caller: AccountId = whitelisted_caller();
	group.bench_function("mul_projective_g2_bw6_761_optimized", |b| {
		b.iter(|| {
			let _ = do_mul_projective_g2_bw6_761_optimized();
		});
	});
	group.bench_function("mul_projective_g2_bw6_761", |b| {
		b.iter(|| {
			let _ = do_mul_projective_g2_bw6_761();
		});
	});
	group.finish();
}

fn do_mul_projective_g2_bw6_761_optimized() -> Result<(), Error> {
	let _out = <sp_ark_bw6_761::g2::Config<HostBW6_761> as SWCurveConfig>::mul_projective(
		&G2ProjectiveBW6_761::generator(),
		&[2u64],
	);
	Ok(())
}

fn do_mul_projective_g2_bw6_761() -> Result<(), Error> {
	let _out = <ark_bw6_761::g2::Config as SWCurveConfig>::mul_projective(
		&ark_bw6_761::G2Projective::generator(),
		&[2u64],
	);
	Ok(())
}

pub fn bench_pairing_bw6_761(c: &mut Criterion) {
	let mut group = c.benchmark_group("pairing_arkworks_bw6_761");
	let caller: AccountId = whitelisted_caller();
	group.bench_function("pairing_arkworks_bw6_761_optimized", |b| {
		b.iter(|| {
			let _ = do_pairing_bw6_761_optimized();
		});
	});
	group.bench_function("pairing_arkworks_bw6_761", |b| {
		b.iter(|| {
			let _ = do_pairing_bw6_761();
		});
	});
	group.finish();
}

fn do_pairing_bw6_761_optimized() -> Result<(), Error> {
	let _out = BW6_761Optimized::multi_pairing(
		[G1AffineBW6_761::generator()],
		[G2AffineBW6_761::generator()],
	);
	Ok(())
}

fn do_pairing_bw6_761() -> Result<(), Error> {
	let _out = ark_bw6_761::BW6_761::multi_pairing(
		[ark_bw6_761::G1Affine::generator()],
		[ark_bw6_761::G2Affine::generator()],
	);
	Ok(())
}
