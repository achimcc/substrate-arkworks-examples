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
use sp_bls12_377::{
	Bls12_377, G1Affine as G1AffineBls12_377,
	G1Projective as G1ProjectiveBls12_377, G2Affine as G2AffineBls12_377,
	G2Projective as G2ProjectiveBls12_377, 
};
use sp_bls12_381::{
	Bls12_381, Fr as BlsFrOptimized, G1Affine as G1AffineBls12_381,
	G1Projective as G1ProjectiveBls12_381, G2Affine as G2AffineBls12_381,
	G2Projective as G2ProjectiveBls12_381, 
};
use sp_bw6_761::{
	G1Affine as G1AffineBW6_761, G1Projective as G1ProjectiveBW6_761,
	G2Affine as G2AffineBW6_761, G2Projective as G2ProjectiveBW6_761,
	BW6_761,
};
use sp_ark_models::{pairing::Pairing, short_weierstrass::SWCurveConfig, AffineRepr, Group};

#[pallet::call_index(22)]
#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
pub fn mul_projective_g2_bw6_761(_origin: OriginFor<T>) -> DispatchResult {
	let _out = <sp_bw6_761::g2::Config<HostBW6_761> as SWCurveConfig>::mul_projective(
		&G2ProjectiveBW6_761::generator(),
		&[2u64],
	);
	Ok(())
}
