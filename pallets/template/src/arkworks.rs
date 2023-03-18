use ark_bls12_377::Bls12_377;
use ark_bls12_381::{Bls12_381, Fr as BlsFr};
use ark_bw6_761::BW6_761;
use ark_ff::Fp;
use ark_groth16::Groth16;
use ark_serialize::{CanonicalDeserialize, Compress, Validate};
use ark_snark::SNARK;
use ark_std::{test_rng, vec, vec::Vec, UniformRand};
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use sp_ark_bls12_377::{
	Bls12_377 as Bls12_377_Host, G1Affine as G1AffineBls12_377_Host,
	G1Projective as G1ProjectiveBls12_377_Host, G2Affine as G2AffineBls12_377_Host,
	G2Projective as G2ProjectiveBls12_377_Host, HostFunctions as Bls12_377HostFunctions,
};
use sp_ark_bls12_381::{
	Bls12_381 as Bls12_381_Host, Fr as BlsFrOptimized, G1Affine as G1AffineBls12_381_Host,
	G1Projective as G1ProjectiveBls12_381_Host, G2Affine as G2AffineBls12_381_Host,
	G2Projective as G2ProjectiveBls12_381_Host, HostFunctions as Bls12_381HostFunctions,
};
use sp_ark_bw6_761::{
	G1Affine as G1AffineBW6_761_Host, G1Projective as G1ProjectiveBW6_761_Host,
	G2Affine as G2AffineBW6_761_Host, G2Projective as G2ProjectiveBW6_761_Host,
	HostFunctions as BW6_761HostFunctions, BW6_761 as BW6_761_Host,
};
use sp_ark_models::{pairing::Pairing, short_weierstrass::SWCurveConfig, AffineRepr, Group};

impl BW6_761HostFunctions for HostBW6_761 {
	fn bw6_761_multi_miller_loop(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
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
	fn bw6_761_msm_g1(bases: Vec<u8>, bigints: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::bw6_761_msm_g1(bases, bigints)
	}
	fn bw6_761_msm_g2(bases: Vec<u8>, bigints: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::bw6_761_msm_g2(bases, bigints)
	}
}

type BW6_761Optimized = BW6_761_Host<HostBW6_761>;
type G1AffineBW6_761 = G1AffineBW6_761_Host<HostBW6_761>;
type G2AffineBW6_761 = G2AffineBW6_761_Host<HostBW6_761>;
type G1ProjectiveBW6_761 = G1ProjectiveBW6_761_Host<HostBW6_761>;
type G2ProjectiveBW6_761 = G2ProjectiveBW6_761_Host<HostBW6_761>;

#[pallet::call_index(22)]
#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
pub fn mul_projective_g2_bw6_761(_origin: OriginFor<T>) -> DispatchResult {
	let _out = <sp_ark_bw6_761::g2::Config<HostBW6_761> as SWCurveConfig>::mul_projective(
		&G2ProjectiveBW6_761::generator(),
		&[2u64],
	);
	Ok(())
}
