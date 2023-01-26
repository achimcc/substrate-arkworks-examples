use ark_std::io::Error;
use criterion::Criterion;
use sp_ark_bls12_377::{
	Bls12_377 as Bls12_377_Host, G1Affine as G1AffineBls12_377_Host,
	G1Projective as G1ProjectiveBls12_377_Host, G2Affine as G2AffineBls12_377_Host,
	G2Projective as G2ProjectiveBls12_377_Host, HostFunctions as Bls12_377HostFunctions,
};
use sp_ark_models::{pairing::Pairing, short_weierstrass::SWCurveConfig, AffineRepr, Group};

struct HostBls12_377 {}

impl Bls12_377HostFunctions for HostBls12_377 {
	fn bls12_377_multi_miller_loop(a: Vec<Vec<u8>>, b: Vec<Vec<u8>>) -> Vec<u8> {
		sp_io::elliptic_curves::bls12_377_multi_miller_loop(a, b)
	}
	fn bls12_377_final_exponentiation(f12: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::bls12_377_final_exponentiation(f12)
	}
	fn bls12_377_msm_g1(bases: Vec<Vec<u8>>, bigints: Vec<Vec<u8>>) -> Vec<u8> {
		sp_io::elliptic_curves::bls12_377_msm_g1(bases, bigints)
	}
	fn bls12_377_mul_projective_g1(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::bls12_377_mul_projective_g1(base, scalar)
	}
	fn bls12_377_mul_affine_g1(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::bls12_377_mul_affine_g1(base, scalar)
	}
	fn bls12_377_msm_g2(bases: Vec<Vec<u8>>, bigints: Vec<Vec<u8>>) -> Vec<u8> {
		sp_io::elliptic_curves::bls12_377_msm_g2(bases, bigints)
	}
	fn bls12_377_mul_projective_g2(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::bls12_377_mul_projective_g2(base, scalar)
	}
	fn bls12_377_mul_affine_g2(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::bls12_377_mul_affine_g2(base, scalar)
	}
}

type Bls12_377Optimized = Bls12_377_Host<HostBls12_377>;
type G1AffineBls12_377 = G1AffineBls12_377_Host<HostBls12_377>;
type G2AffineBls12_377 = G2AffineBls12_377_Host<HostBls12_377>;
type G1ProjectiveBls12_377 = G1ProjectiveBls12_377_Host<HostBls12_377>;
type G2ProjectiveBls12_377 = G2ProjectiveBls12_377_Host<HostBls12_377>;

pub fn bench_pairing_bls12_377(c: &mut Criterion) {
	let mut group = c.benchmark_group("pairing_bls12_377");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = do_pairing_bls12_377();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = do_pairing_bls12_377_optimized();
		});
	});
	group.finish();
}

pub fn do_pairing_bls12_377() -> Result<(), Error> {
	let _out = ark_bls12_377::Bls12_377::multi_pairing(
		[ark_bls12_377::G1Affine::generator()],
		[ark_bls12_377::G2Affine::generator()],
	);
	Ok(())
}

pub fn do_pairing_bls12_377_optimized() -> Result<(), Error> {
	let _out = Bls12_377Optimized::multi_pairing(
		[G1AffineBls12_377::generator()],
		[G2AffineBls12_377::generator()],
	);
	Ok(())
}

pub fn bench_msm_g1_bls12_377(c: &mut Criterion) {
	let mut group = c.benchmark_group("pairing_msm_g1_bls12_377");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = do_msm_g1_bls12_377();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = do_msm_g1_bls12_377_optimized();
		});
	});
	group.finish();
}

pub fn do_msm_g1_bls12_377() -> Result<(), Error> {
	let _out = <ark_bls12_377::g1::Config as SWCurveConfig>::msm(
		&[ark_bls12_377::G1Affine::generator()],
		&[2u64.into()],
	);
	Ok(())
}

pub fn do_msm_g1_bls12_377_optimized() -> Result<(), Error> {
	let _out = <sp_ark_bls12_377::g1::Config<HostBls12_377> as SWCurveConfig>::msm(
		&[G1AffineBls12_377::generator()],
		&[2u64.into()],
	);
	Ok(())
}

pub fn bench_msm_g2_bls12_377(c: &mut Criterion) {
	let mut group = c.benchmark_group("pairing_msm_g2_bls12_377");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = do_msm_g2_bls12_377();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = do_msm_g2_bls12_377_optimized();
		});
	});
	group.finish();
}

pub fn do_msm_g2_bls12_377() -> Result<(), Error> {
	let _out = <ark_bls12_377::g2::Config as SWCurveConfig>::msm(
		&[ark_bls12_377::G2Affine::generator()],
		&[2u64.into()],
	);
	Ok(())
}

pub fn do_msm_g2_bls12_377_optimized() -> Result<(), Error> {
	let _out = <sp_ark_bls12_377::g2::Config<HostBls12_377> as SWCurveConfig>::msm(
		&[G2AffineBls12_377::generator()],
		&[2u64.into()],
	);
	Ok(())
}

pub fn bench_mul_projective_g1_bls12_377(c: &mut Criterion) {
	let mut group = c.benchmark_group("mul_projective_g1_bls12_377");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = do_mul_projective_g1_bls12_377();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = do_mul_projective_g1_bls12_377_optimized();
		})
	});
	group.finish();
}

pub fn do_mul_projective_g1_bls12_377() -> Result<(), Error> {
	let _out = <ark_bls12_377::g1::Config as SWCurveConfig>::mul_projective(
		&ark_bls12_377::G1Projective::generator(),
		&[2u64],
	);
	Ok(())
}

pub fn do_mul_projective_g1_bls12_377_optimized() -> Result<(), Error> {
	let _out = <sp_ark_bls12_377::g1::Config<HostBls12_377> as SWCurveConfig>::mul_projective(
		&G1ProjectiveBls12_377::generator(),
		&[2u64],
	);
	Ok(())
}

pub fn bench_mul_affine_g1_bls12_377(c: &mut Criterion) {
	let mut group = c.benchmark_group("mul_affine_g1_bls12_377");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = do_mul_affine_g1_bls12_377();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = do_mul_affine_g1_bls12_377_optimized();
		});
	});
	group.finish();
}

pub fn do_mul_affine_g1_bls12_377() -> Result<(), Error> {
	let _out = <ark_bls12_377::g1::Config as SWCurveConfig>::mul_affine(
		&ark_bls12_377::G1Affine::generator(),
		&[2u64],
	);
	Ok(())
}

pub fn do_mul_affine_g1_bls12_377_optimized() -> Result<(), Error> {
	let _out = <sp_ark_bls12_377::g1::Config<HostBls12_377> as SWCurveConfig>::mul_affine(
		&G1AffineBls12_377::generator(),
		&[2u64],
	);
	Ok(())
}

pub fn bench_mul_projective_g2_bls12_377(c: &mut Criterion) {
	let mut group = c.benchmark_group("mul_projective_g2_bls12_377");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = do_mul_projective_g2_bls12_377();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = do_mul_projective_g2_bls12_377_optimized();
		});
	});
	group.finish();
}

pub fn do_mul_projective_g2_bls12_377() -> Result<(), Error> {
	let _out = <ark_bls12_377::g2::Config as SWCurveConfig>::mul_projective(
		&ark_bls12_377::G2Projective::generator(),
		&[2u64],
	);
	Ok(())
}

pub fn do_mul_projective_g2_bls12_377_optimized() -> Result<(), Error> {
	let _out = <sp_ark_bls12_377::g2::Config<HostBls12_377> as SWCurveConfig>::mul_projective(
		&G2ProjectiveBls12_377::generator(),
		&[2u64],
	);
	Ok(())
}

pub fn bench_mul_affine_g2_bls12_377(c: &mut Criterion) {
	let mut group = c.benchmark_group("mul_affine_g2_bls12_377");
	group.bench_function("normal", |b| {
		b.iter(|| {
			let _ = do_mul_affine_g2_bls12_377();
		});
	});
	group.bench_function("optimized", |b| {
		b.iter(|| {
			let _ = do_mul_affine_g2_bls12_377_optimized();
		});
	});
	group.finish();
}

pub fn do_mul_affine_g2_bls12_377() -> Result<(), Error> {
	let _out = <ark_bls12_377::g2::Config as SWCurveConfig>::mul_affine(
		&ark_bls12_377::G2Affine::generator(),
		&[2u64],
	);
	Ok(())
}

pub fn do_mul_affine_g2_bls12_377_optimized() -> Result<(), Error> {
	let _out = <sp_ark_bls12_377::g2::Config<HostBls12_377> as SWCurveConfig>::mul_affine(
		&G2AffineBls12_377::generator(),
		&[2u64],
	);
	Ok(())
}
