#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "1024"]

#[cfg(test)]
mod mock;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[cfg(test)]
mod benchmarking;
#[cfg(test)]
mod tests;

mod bls12_377;
mod bls12_381;
mod bw6_761;
mod ed_on_bls12_377;
mod ed_on_bls12_381_bandersnatch;
mod utils;

pub mod weights;
pub use weights::*;

use ark_scale::hazmat::ArkScaleProjective;

const USAGE: ark_scale::Usage = ark_scale::WIRE;

type ArkScale<T> = ark_scale::ArkScale<T, USAGE>;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use crate::{
		bls12_377, bls12_381, bw6_761, ed_on_bls12_377, ed_on_bls12_381_bandersnatch,
		utils::ScalarFieldFor, ArkScale, ArkScaleProjective, WeightInfo,
	};
	// use ark_ec::CurveConfig;
	use ark_std::{vec, vec::Vec};
	use codec::Decode;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type WeightInfo: WeightInfo;
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// ---------------------------------------------
		// Calls for bls12-381
		// ---------------------------------------------

		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn groth16_verification(
			_: OriginFor<T>,
			vk: Vec<u8>,
			c: Vec<u8>,
			proof: Vec<u8>,
		) -> DispatchResult {
			bls12_381::groth16_verify(vk, c, proof);
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn groth16_verification_opt(
			_: OriginFor<T>,
			vk: Vec<u8>,
			c: Vec<u8>,
			proof: Vec<u8>,
		) -> DispatchResult {
			bls12_381::groth16_verify_opt(vk, c, proof);
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_pairing(
			_: OriginFor<T>,
			a: ArkScale<ark_bls12_381::G1Affine>,
			b: ArkScale<ark_bls12_381::G2Affine>,
		) -> DispatchResult {
			bls12_381::pairing(a.0, b.0);
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_pairing_opt(
			_: OriginFor<T>,
			a: ArkScale<sp_bls12_381::G1Affine>,
			b: ArkScale<sp_bls12_381::G2Affine>,
		) -> DispatchResult {
			bls12_381::pairing_opt(a.0, b.0);
			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_msm_g1(
			_: OriginFor<T>,
			bases: Vec<u8>,
			scalars: Vec<u8>,
		) -> DispatchResult {
			let bases =
				ArkScale::<Vec<ark_bls12_381::G1Affine>>::decode(&mut bases.as_slice()).unwrap();
			let scalars = ArkScale::<Vec<ScalarFieldFor<ark_bls12_381::G1Affine>>>::decode(
				&mut scalars.as_slice(),
			)
			.unwrap();

			bls12_381::msm_g1(&bases.0, &scalars.0);
			Ok(())
		}

		#[pallet::call_index(5)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_msm_g1_opt(
			_: OriginFor<T>,
			bases: Vec<u8>,
			scalars: Vec<u8>,
		) -> DispatchResult {
			let bases =
				ArkScale::<Vec<sp_bls12_381::G1Affine>>::decode(&mut bases.as_slice()).unwrap();
			let scalars = ArkScale::<Vec<ScalarFieldFor<sp_bls12_381::G1Affine>>>::decode(
				&mut scalars.as_slice(),
			)
			.unwrap();

			bls12_381::msm_g1_opt(&bases.0, &scalars.0);
			Ok(())
		}

		#[pallet::call_index(6)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_msm_g2(
			_: OriginFor<T>,
			bases: Vec<u8>,
			scalars: Vec<u8>,
		) -> DispatchResult {
			let bases =
				ArkScale::<Vec<ark_bls12_381::G2Affine>>::decode(&mut bases.as_slice()).unwrap();
			let scalars = ArkScale::<Vec<ScalarFieldFor<ark_bls12_381::G2Affine>>>::decode(
				&mut scalars.as_slice(),
			)
			.unwrap();

			bls12_381::msm_g2(&bases.0, &scalars.0);
			Ok(())
		}

		#[pallet::call_index(7)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_msm_g2_opt(
			_: OriginFor<T>,
			bases: Vec<u8>,
			scalars: Vec<u8>,
		) -> DispatchResult {
			let bases =
				ArkScale::<Vec<sp_bls12_381::G2Affine>>::decode(&mut bases.as_slice()).unwrap();
			let scalars = ArkScale::<Vec<ScalarFieldFor<sp_bls12_381::G2Affine>>>::decode(
				&mut scalars.as_slice(),
			)
			.unwrap();

			bls12_381::msm_g2_opt(&bases.0, &scalars.0);
			Ok(())
		}

		#[pallet::call_index(8)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_mul_projective_g1(
			_: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base =
				ArkScaleProjective::<ark_bls12_381::G1Projective>::decode(&mut base.as_slice())
					.unwrap();
			let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();

			bls12_381::mul_projective_g1(&base.0, &scalar.0);
			Ok(())
		}

		#[pallet::call_index(9)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_mul_projective_g1_opt(
			_: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base =
				ArkScaleProjective::<sp_bls12_381::G1Projective>::decode(&mut base.as_slice())
					.unwrap();
			let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();

			bls12_381::mul_projective_g1_opt(&base.0, &scalar.0);
			Ok(())
		}

		#[pallet::call_index(10)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_mul_affine_g1(
			_: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = ArkScale::<ark_bls12_381::G1Affine>::decode(&mut base.as_slice()).unwrap();
			let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();

			bls12_381::mul_affine_g1(&base.0, &scalar.0);
			Ok(())
		}

		#[pallet::call_index(11)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_mul_affine_g1_opt(
			_: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = ArkScale::<sp_bls12_381::G1Affine>::decode(&mut base.as_slice()).unwrap();
			let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();

			bls12_381::mul_affine_g1_opt(&base.0, &scalar.0);
			Ok(())
		}

		#[pallet::call_index(12)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_mul_projective_g2(
			_: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base =
				ArkScaleProjective::<ark_bls12_381::G2Projective>::decode(&mut base.as_slice())
					.unwrap();
			let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();

			bls12_381::mul_projective_g2(&base.0, &scalar.0);
			Ok(())
		}

		#[pallet::call_index(13)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_mul_projective_g2_opt(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base =
				ArkScaleProjective::<sp_bls12_381::G2Projective>::decode(&mut base.as_slice())
					.unwrap();
			let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).unwrap();

			bls12_381::mul_projective_g2_opt(&base.0, &scalar.0);
			Ok(())
		}

		#[pallet::call_index(14)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_mul_affine_g2(
			_: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = <ArkScale<ark_bls12_381::G2Affine> as Decode>::decode(&mut base.as_slice())
				.unwrap();
			let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).unwrap();

			bls12_381::mul_affine_g2(&base.0, &scalar.0);
			Ok(())
		}

		#[pallet::call_index(15)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_mul_affine_g2_opt(
			_: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = ArkScale::<sp_bls12_381::G2Affine>::decode(&mut base.as_slice()).unwrap();
			let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();

			bls12_381::mul_affine_g2_opt(&base.0, &scalar.0);
			Ok(())
		}

		// ---------------------------------------------
		// Calls for bls12-377
		// ---------------------------------------------

		#[pallet::call_index(16)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_pairing(
			_: OriginFor<T>,
			a: ArkScale<ark_bls12_377::G1Affine>,
			b: ArkScale<ark_bls12_377::G2Affine>,
		) -> DispatchResult {
			bls12_377::pairing(a.0, b.0);
			Ok(())
		}

		#[pallet::call_index(17)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_pairing_opt(
			_: OriginFor<T>,
			a: ArkScale<sp_bls12_377::G1Affine>,
			b: ArkScale<sp_bls12_377::G2Affine>,
		) -> DispatchResult {
			bls12_377::pairing_opt(a.0, b.0);
			Ok(())
		}

		#[pallet::call_index(18)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_msm_g1(
			_: OriginFor<T>,
			bases: Vec<u8>,
			scalars: Vec<u8>,
		) -> DispatchResult {
			let bases =
				ArkScale::<Vec<ark_bls12_377::G1Affine>>::decode(&mut bases.as_slice()).unwrap();
			let scalars = ArkScale::<Vec<ScalarFieldFor<ark_bls12_377::G1Affine>>>::decode(
				&mut scalars.as_slice(),
			)
			.unwrap();

			bls12_377::msm_g1(&bases.0, &scalars.0);
			Ok(())
		}

		#[pallet::call_index(19)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_msm_g1_opt(
			_: OriginFor<T>,
			bases: Vec<u8>,
			scalars: Vec<u8>,
		) -> DispatchResult {
			let bases =
				ArkScale::<Vec<sp_bls12_377::G1Affine>>::decode(&mut bases.as_slice()).unwrap();
			let scalars = ArkScale::<Vec<ScalarFieldFor<sp_bls12_377::G1Affine>>>::decode(
				&mut scalars.as_slice(),
			)
			.unwrap();

			bls12_377::msm_g1_opt(&bases.0, &scalars.0);
			Ok(())
		}

		#[pallet::call_index(20)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_msm_g2(
			_: OriginFor<T>,
			bases: Vec<u8>,
			scalars: Vec<u8>,
		) -> DispatchResult {
			let bases =
				ArkScale::<Vec<ark_bls12_377::G2Affine>>::decode(&mut bases.as_slice()).unwrap();
			let scalars = ArkScale::<Vec<ScalarFieldFor<ark_bls12_377::G2Affine>>>::decode(
				&mut scalars.as_slice(),
			)
			.unwrap();

			bls12_377::msm_g2(&bases.0, &scalars.0);
			Ok(())
		}

		#[pallet::call_index(21)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_msm_g2_opt(
			_: OriginFor<T>,
			bases: Vec<u8>,
			scalars: Vec<u8>,
		) -> DispatchResult {
			let bases =
				ArkScale::<Vec<sp_bls12_377::G2Affine>>::decode(&mut bases.as_slice()).unwrap();
			let scalars = ArkScale::<Vec<ScalarFieldFor<sp_bls12_377::G2Affine>>>::decode(
				&mut scalars.as_slice(),
			)
			.unwrap();

			bls12_377::msm_g2_opt(&bases.0, &scalars.0);
			Ok(())
		}

		#[pallet::call_index(22)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_mul_projective_g1(
			_: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base =
				ArkScaleProjective::<ark_bls12_377::G1Projective>::decode(&mut base.as_slice())
					.unwrap();
			let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();

			bls12_377::mul_projective_g1(&base.0, &scalar.0);
			Ok(())
		}

		#[pallet::call_index(23)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_mul_projective_g1_opt(
			_: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base =
				ArkScaleProjective::<sp_bls12_377::G1Projective>::decode(&mut base.as_slice())
					.unwrap();
			let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();

			bls12_377::mul_projective_g1_opt(&base.0, &scalar.0);
			Ok(())
		}

		#[pallet::call_index(24)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_mul_affine_g1(
			_: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = ArkScale::<ark_bls12_377::G1Affine>::decode(&mut base.as_slice()).unwrap();
			let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();

			bls12_377::mul_affine_g1(&base.0, &scalar.0);
			Ok(())
		}

		#[pallet::call_index(25)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_mul_affine_g1_opt(
			_: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = ArkScale::<sp_bls12_377::G1Affine>::decode(&mut base.as_slice()).unwrap();
			let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();

			bls12_377::mul_affine_g1_opt(&base.0, &scalar.0);
			Ok(())
		}

		#[pallet::call_index(26)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_mul_projective_g2(
			_: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base =
				ArkScaleProjective::<ark_bls12_377::G2Projective>::decode(&mut base.as_slice())
					.unwrap();
			let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();

			bls12_377::mul_projective_g2(&base.0, &scalar.0);
			Ok(())
		}

		#[pallet::call_index(27)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_mul_projective_g2_opt(
			_: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base =
				ArkScaleProjective::<sp_bls12_377::G2Projective>::decode(&mut base.as_slice())
					.unwrap();
			let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();

			bls12_377::mul_projective_g2_opt(&base.0, &scalar.0);
			Ok(())
		}

		#[pallet::call_index(28)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_mul_affine_g2(
			_: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = ArkScale::<ark_bls12_377::G2Affine>::decode(&mut base.as_slice()).unwrap();
			let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();

			bls12_377::mul_affine_g2(&base.0, &scalar.0);
			Ok(())
		}

		#[pallet::call_index(29)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_mul_affine_g2_opt(
			_: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = ArkScale::<sp_bls12_377::G2Affine>::decode(&mut base.as_slice()).unwrap();
			let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();

			bls12_377::mul_affine_g2_opt(&base.0, &scalar.0);
			Ok(())
		}

		// ---------------------------------------------
		// Calls for bw6-761
		// ---------------------------------------------

		#[pallet::call_index(30)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_pairing(
			_: OriginFor<T>,
			a: ArkScale<ark_bw6_761::G1Affine>,
			b: ArkScale<ark_bw6_761::G2Affine>,
		) -> DispatchResult {
			bw6_761::pairing(a.0, b.0);
			Ok(())
		}

		#[pallet::call_index(31)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_pairing_opt(
			_: OriginFor<T>,
			a: ArkScale<sp_bw6_761::g1::G1Affine>,
			b: ArkScale<sp_bw6_761::g2::G2Affine>,
		) -> DispatchResult {
			bw6_761::pairing_opt(a.0, b.0);
			Ok(())
		}

		// // --- NEW FROM HERE

		#[pallet::call_index(32)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_msm_g1(_: OriginFor<T>, bases: Vec<u8>, scalars: Vec<u8>) -> DispatchResult {
			let bases =
				ArkScale::<Vec<ark_bw6_761::G1Affine>>::decode(&mut bases.as_slice()).unwrap();
			let scalars = ArkScale::<Vec<ScalarFieldFor<ark_bw6_761::G1Affine>>>::decode(
				&mut scalars.as_slice(),
			)
			.unwrap();

			bw6_761::msm_g1(&bases.0, &scalars.0);
			Ok(())
		}

		#[pallet::call_index(33)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_msm_g1_opt(
			_: OriginFor<T>,
			bases: Vec<u8>,
			scalars: Vec<u8>,
		) -> DispatchResult {
			let bases =
				ArkScale::<Vec<sp_bw6_761::g1::G1Affine>>::decode(&mut bases.as_slice()).unwrap();
			let scalars = ArkScale::<Vec<ScalarFieldFor<sp_bw6_761::g1::G1Affine>>>::decode(
				&mut scalars.as_slice(),
			)
			.unwrap();
			bw6_761::msm_g1_opt(&bases.0, &scalars.0);
			Ok(())
		}

		#[pallet::call_index(34)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_msm_g2(_: OriginFor<T>, bases: Vec<u8>, scalars: Vec<u8>) -> DispatchResult {
			let bases =
				ArkScale::<Vec<ark_bw6_761::G2Affine>>::decode(&mut bases.as_slice()).unwrap();
			let scalars = ArkScale::<Vec<ScalarFieldFor<ark_bw6_761::G2Affine>>>::decode(
				&mut scalars.as_slice(),
			)
			.unwrap();

			bw6_761::msm_g2(&bases.0, &scalars.0);
			Ok(())
		}

		#[pallet::call_index(35)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_msm_g2_opt(
			_: OriginFor<T>,
			bases: Vec<u8>,
			scalars: Vec<u8>,
		) -> DispatchResult {
			let bases =
				ArkScale::<Vec<sp_bw6_761::g2::G2Affine>>::decode(&mut bases.as_slice()).unwrap();
			let scalars = ArkScale::<Vec<ScalarFieldFor<sp_bw6_761::g2::G2Affine>>>::decode(
				&mut scalars.as_slice(),
			)
			.unwrap();

			bw6_761::msm_g2_opt(&bases.0, &scalars.0);
			Ok(())
		}

		#[pallet::call_index(36)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_mul_projective_g1(
			_: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base =
				ArkScaleProjective::<ark_bw6_761::G1Projective>::decode(&mut base.as_slice())
					.unwrap();
			let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();

			bw6_761::mul_projective_g1(&base.0, &scalar.0);
			Ok(())
		}

		#[pallet::call_index(37)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_mul_projective_g1_opt(
			_: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base =
				ArkScaleProjective::<sp_bw6_761::g1::G1Projective>::decode(&mut base.as_slice())
					.unwrap();
			let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();

			bw6_761::mul_projective_g1_opt(&base.0, &scalar.0);
			Ok(())
		}

		#[pallet::call_index(38)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_mul_affine_g1(
			_: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = ArkScale::<ark_bw6_761::G1Affine>::decode(&mut base.as_slice()).unwrap();
			let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();

			bw6_761::mul_affine_g1(&base.0, &scalar.0);
			Ok(())
		}

		#[pallet::call_index(39)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_mul_affine_g1_opt(
			_: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = ArkScale::<sp_bw6_761::g1::G1Affine>::decode(&mut base.as_slice()).unwrap();
			let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();

			bw6_761::mul_affine_g1_opt(&base.0, &scalar.0);
			Ok(())
		}

		#[pallet::call_index(40)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_mul_projective_g2(
			_: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base =
				ArkScaleProjective::<ark_bw6_761::G2Projective>::decode(&mut base.as_slice())
					.unwrap();
			let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();

			bw6_761::mul_projective_g2(&base.0, &scalar.0);
			Ok(())
		}

		#[pallet::call_index(41)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_mul_projective_g2_opt(
			_: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base =
				ArkScaleProjective::<sp_bw6_761::g2::G2Projective>::decode(&mut base.as_slice())
					.unwrap();
			let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();

			bw6_761::mul_projective_g2_opt(&base.0, &scalar.0);
			Ok(())
		}

		#[pallet::call_index(42)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_mul_affine_g2(
			_: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = ArkScale::<ark_bw6_761::G2Affine>::decode(&mut base.as_slice()).unwrap();
			let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();

			bw6_761::mul_affine_g2(&base.0, &scalar.0);
			Ok(())
		}

		#[pallet::call_index(43)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_mul_affine_g2_opt(
			_: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = ArkScale::<sp_bw6_761::g2::G2Affine>::decode(&mut base.as_slice()).unwrap();
			let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();

			bw6_761::mul_affine_g2_opt(&base.0, &scalar.0);
			Ok(())
		}

		// ---------------------------------------------
		// Calls for ed-on-bls12-381-bandersnatch
		// ---------------------------------------------

		#[pallet::call_index(44)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_381_bandersnatch_msm_sw(
			_: OriginFor<T>,
			bases: Vec<u8>,
			scalars: Vec<u8>,
		) -> DispatchResult {
			let bases = ArkScale::<Vec<ark_ed_on_bls12_381_bandersnatch::SWAffine>>::decode(
				&mut bases.as_slice(),
			)
			.unwrap();
			let scalars = ArkScale::<
				Vec<ScalarFieldFor<ark_ed_on_bls12_381_bandersnatch::SWAffine>>,
			>::decode(&mut scalars.as_slice())
			.unwrap();

			ed_on_bls12_381_bandersnatch::msm_sw(&bases.0, &scalars.0);
			Ok(())
		}

		#[pallet::call_index(45)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_381_bandersnatch_msm_sw_opt(
			_: OriginFor<T>,
			bases: Vec<u8>,
			scalars: Vec<u8>,
		) -> DispatchResult {
			let bases = ArkScale::<Vec<sp_ed_on_bls12_381_bandersnatch::SWAffine>>::decode(
				&mut bases.as_slice(),
			)
			.unwrap();
			let scalars =
				ArkScale::<Vec<ScalarFieldFor<sp_ed_on_bls12_381_bandersnatch::SWAffine>>>::decode(
					&mut scalars.as_slice(),
				)
				.unwrap();

			ed_on_bls12_381_bandersnatch::msm_sw_opt(&bases.0, &scalars.0);
			Ok(())
		}

		#[pallet::call_index(46)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_381_bandersnatch_msm_te(
			_origin: OriginFor<T>,
			bases: Vec<u8>,
			scalars: Vec<u8>,
		) -> DispatchResult {
			let bases = ArkScale::<Vec<ark_ed_on_bls12_381_bandersnatch::EdwardsAffine>>::decode(
				&mut bases.as_slice(),
			)
			.unwrap();
			let scalars = ArkScale::<
				Vec<ScalarFieldFor<ark_ed_on_bls12_381_bandersnatch::EdwardsAffine>>,
			>::decode(&mut scalars.as_slice())
			.unwrap();

			ed_on_bls12_381_bandersnatch::msm_te(&bases.0, &scalars.0);
			Ok(())
		}

		#[pallet::call_index(47)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_381_bandersnatch_msm_te_opt(
			_origin: OriginFor<T>,
			bases: Vec<u8>,
			scalars: Vec<u8>,
		) -> DispatchResult {
			let bases = ArkScale::<Vec<sp_ed_on_bls12_381_bandersnatch::EdwardsAffine>>::decode(
				&mut bases.as_slice(),
			)
			.unwrap();
			let scalars = <ArkScale<
				Vec<ScalarFieldFor<sp_ed_on_bls12_381_bandersnatch::EdwardsAffine>>,
			> as Decode>::decode(&mut scalars.as_slice())
			.unwrap();

			ed_on_bls12_381_bandersnatch::msm_te_opt(&bases.0, &scalars.0);
			Ok(())
		}

		#[pallet::call_index(48)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_381_bandersnatch_mul_projective_sw(
			_: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base =
				ArkScaleProjective::<ark_ed_on_bls12_381_bandersnatch::SWProjective>::decode(
					&mut base.as_slice(),
				)
				.unwrap();
			let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();

			ed_on_bls12_381_bandersnatch::mul_projective_sw(&base.0, &scalar.0);
			Ok(())
		}

		#[pallet::call_index(49)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_381_bandersnatch_mul_projective_sw_opt(
			_: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = ArkScaleProjective::<sp_ed_on_bls12_381_bandersnatch::SWProjective>::decode(
				&mut base.as_slice(),
			)
			.unwrap();
			let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();

			ed_on_bls12_381_bandersnatch::mul_projective_sw_opt(&base.0, &scalar.0);
			Ok(())
		}

		#[pallet::call_index(50)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_381_bandersnatch_mul_projective_te(
			_: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base =
				ArkScaleProjective::<ark_ed_on_bls12_381_bandersnatch::EdwardsProjective>::decode(
					&mut base.as_slice(),
				)
				.unwrap();
			let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();

			ed_on_bls12_381_bandersnatch::mul_projective_te(&base.0, &scalar.0);
			Ok(())
		}

		#[pallet::call_index(51)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_381_bandersnatch_mul_projective_te_opt(
			_: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base =
				ArkScaleProjective::<sp_ed_on_bls12_381_bandersnatch::EdwardsProjective>::decode(
					&mut base.as_slice(),
				)
				.unwrap();
			let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();

			ed_on_bls12_381_bandersnatch::mul_projective_te_opt(&base.0, &scalar.0);
			Ok(())
		}

		#[pallet::call_index(52)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_381_bandersnatch_mul_affine_sw(
			_: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = ArkScale::<ark_ed_on_bls12_381_bandersnatch::SWAffine>::decode(
				&mut base.as_slice(),
			)
			.unwrap();
			let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();

			ed_on_bls12_381_bandersnatch::mul_affine_sw(&base.0, &scalar.0);
			Ok(())
		}

		#[pallet::call_index(53)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_381_bandersnatch_mul_affine_sw_opt(
			_: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base =
				ArkScale::<sp_ed_on_bls12_381_bandersnatch::SWAffine>::decode(&mut base.as_slice())
					.unwrap();
			let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();

			ed_on_bls12_381_bandersnatch::mul_affine_sw_opt(&base.0, &scalar.0);
			Ok(())
		}

		#[pallet::call_index(54)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_381_bandersnatch_mul_affine_te(
			_: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = ArkScale::<ark_ed_on_bls12_381_bandersnatch::EdwardsAffine>::decode(
				&mut base.as_slice(),
			)
			.unwrap();
			let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();

			ed_on_bls12_381_bandersnatch::mul_affine_te(&base.0, &scalar.0);
			Ok(())
		}

		#[pallet::call_index(55)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_381_bandersnatch_mul_affine_te_opt(
			_: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = ArkScale::<sp_ed_on_bls12_381_bandersnatch::EdwardsAffine>::decode(
				&mut base.as_slice(),
			)
			.unwrap();
			let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();

			ed_on_bls12_381_bandersnatch::mul_affine_te_opt(&base.0, &scalar.0);
			Ok(())
		}

		// ---------------------------------------------
		// Calls for ed-on-bls12-377
		// ---------------------------------------------

		#[pallet::call_index(56)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_377_msm(
			_: OriginFor<T>,
			bases: Vec<u8>,
			scalars: Vec<u8>,
		) -> DispatchResult {
			let bases =
				ArkScale::<Vec<ark_ed_on_bls12_377::EdwardsAffine>>::decode(&mut bases.as_slice())
					.unwrap();
			let scalars =
				ArkScale::<Vec<ScalarFieldFor<ark_ed_on_bls12_377::EdwardsAffine>>>::decode(
					&mut scalars.as_slice(),
				)
				.unwrap();

			ed_on_bls12_377::msm(&bases.0, &scalars.0);
			Ok(())
		}

		#[pallet::call_index(57)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_377_msm_opt(
			_: OriginFor<T>,
			bases: Vec<u8>,
			scalars: Vec<u8>,
		) -> DispatchResult {
			let bases =
				ArkScale::<Vec<sp_ed_on_bls12_377::EdwardsAffine>>::decode(&mut bases.as_slice())
					.unwrap();

			let scalars =
				ArkScale::<Vec<ScalarFieldFor<sp_ed_on_bls12_377::EdwardsAffine>>>::decode(
					&mut scalars.as_slice(),
				)
				.unwrap();

			ed_on_bls12_377::msm_opt(&bases.0, &scalars.0);
			Ok(())
		}

		#[pallet::call_index(58)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_377_mul_projective(
			_: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = ArkScaleProjective::<ark_ed_on_bls12_377::EdwardsProjective>::decode(
				&mut base.as_slice(),
			)
			.unwrap();
			let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();

			ed_on_bls12_377::mul_projective(&base.0, &scalar.0);
			Ok(())
		}

		#[pallet::call_index(59)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_377_mul_projective_opt(
			_: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = ArkScaleProjective::<sp_ed_on_bls12_377::EdwardsProjective>::decode(
				&mut base.as_slice(),
			)
			.unwrap();
			let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();

			ed_on_bls12_377::mul_projective_opt(&base.0, &scalar.0);
			Ok(())
		}

		#[pallet::call_index(60)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_377_mul_affine(
			_: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = ArkScale::<ark_ed_on_bls12_377::EdwardsAffine>::decode(&mut base.as_slice())
				.unwrap();

			let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();
			ed_on_bls12_377::mul_affine(&base.0, &scalar.0);
			Ok(())
		}

		#[pallet::call_index(61)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_377_mul_affine_opt(
			_: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = ArkScale::<sp_ed_on_bls12_377::EdwardsAffine>::decode(&mut base.as_slice())
				.unwrap();

			let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();
			ed_on_bls12_377::mul_affine_opt(&base.0, &scalar.0);
			Ok(())
		}
	}
}
