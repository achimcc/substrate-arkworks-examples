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

const HOST_CALL: ark_scale::Usage = ark_scale::HOST_CALL;

type ArkScale<T> = ark_scale::ArkScale<T, HOST_CALL>;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use crate::{
		bls12_377, bls12_381,
		bls12_381::BlsFrOptimized,
		bw6_761, ed_on_bls12_377, ed_on_bls12_381_bandersnatch, utils,
		utils::{ProofFor, VerifyingKeyFor},
		ArkScale, ArkScaleProjective, WeightInfo,
	};
	use ark_bls12_381::Fr as BlsFr;
	use ark_ec::{pairing::Pairing, CurveConfig};
	use ark_ff::{Fp, MontBackend};
	use ark_groth16::Groth16;
	use ark_serialize::{CanonicalDeserialize, Compress, Validate};
	use ark_snark::SNARK;
	use ark_std::{io::Cursor, vec, vec::Vec};
	use codec::Decode;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Type representing the call weights for this pallet.
		type WeightInfo: WeightInfo;
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Verification of Groth16 proof failed.
		Verification,
		/// Serialization or deserialization failure.
		Serialization,
		/// Circuit synthesis failure.
		CircuitSynthesis,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn groth16_verification(
			origin: OriginFor<T>,
			vk: Vec<u8>,
			c: Vec<u8>,
			proof: Vec<u8>,
		) -> DispatchResult {
			ensure_signed(origin)?;

			let cursor = Cursor::new(&vk);
			let vk = <Groth16<crate::bls12_381::Bls12_381Optimized> as SNARK<BlsFrOptimized>>::VerifyingKey::deserialize_with_mode(
				cursor,
				Compress::No,
				Validate::No,
			)
			.unwrap();
			let vk = crate::utils::serialize_argument(vk);

			let cursor = Cursor::new(&c);
			let c: ark_ff::Fp<ark_ff::MontBackend<ark_bls12_381::FrConfig, 4>, 4> =
				Fp::deserialize_with_mode(cursor, Compress::No, Validate::No).unwrap();
			let c = crate::utils::serialize_argument(c);

			let cursor = Cursor::new(&proof);
			let proof =
				<Groth16<ark_bls12_381::Bls12_381> as SNARK<BlsFr>>::Proof::deserialize_with_mode(
					cursor,
					Compress::No,
					Validate::No,
				)
				.unwrap();
			let proof = crate::utils::serialize_argument(proof);

			bls12_381::do_verify_groth16(vk, c, proof).map_err::<Error<T>, _>(Into::into)?;

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn groth16_verification_opt(
			origin: OriginFor<T>,
			vk: Vec<u8>,
			c: Vec<u8>,
			proof: Vec<u8>,
		) -> DispatchResult {
			ensure_signed(origin)?;

			let vk = VerifyingKeyFor::<ark_bls12_381::Bls12_381, BlsFr>::deserialize_uncompressed_unchecked(&vk[..])
				.map_err(|_| Error::<T>::Serialization)?;
			let vk = utils::serialize_argument(vk);

			let c = ark_ff::Fp::<ark_ff::MontBackend<ark_bls12_381::FrConfig, 4>, 4>::deserialize_uncompressed_unchecked(&c[..])
					.map_err(|_| Error::<T>::Serialization)?;
			let c = crate::utils::serialize_argument(c);

			let proof =
				ProofFor::<ark_bls12_381::Bls12_381, BlsFr>::deserialize_uncompressed_unchecked(
					&proof[..],
				)
				.map_err(|_| Error::<T>::Serialization)?;
			let proof = utils::serialize_argument(proof);

			bls12_381::do_verify_groth16_opt(vk, c, proof).map_err::<Error<T>, _>(Into::into)?;

			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_pairing(_origin: OriginFor<T>, a: Vec<u8>, b: Vec<u8>) -> DispatchResult {
			let a = <ArkScale<ark_bls12_381::G1Affine> as Decode>::decode(&mut a.as_slice())
				.unwrap()
				.0;
			let b = <ArkScale<ark_bls12_381::G2Affine> as Decode>::decode(&mut b.as_slice())
				.unwrap()
				.0;
			let _ = crate::bls12_381::do_pairing(a, b);
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_pairing_optimized(
			_origin: OriginFor<T>,
			a: Vec<u8>,
			b: Vec<u8>,
		) -> DispatchResult {
			let a = <ArkScale<bls12_381::G1AffineOptimized> as Decode>::decode(&mut a.as_slice())
				.unwrap()
				.0;
			let b = <ArkScale<bls12_381::G2AffineOptimized> as Decode>::decode(&mut b.as_slice())
				.unwrap()
				.0;
			let _ = crate::bls12_381::do_pairing_optimized(a, b);
			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_msm_g1(
			_origin: OriginFor<T>,
			bases: Vec<u8>,
			scalars: Vec<u8>,
		) -> DispatchResult {
			let bases =
				<ArkScale<Vec<ark_bls12_381::G1Affine>> as Decode>::decode(&mut bases.as_slice())
					.unwrap()
					.0;
			let scalars = <ArkScale<
				Vec<<ark_bls12_381::Bls12_381 as Pairing>::ScalarField>,
			> as Decode>::decode(&mut scalars.as_slice())
			.unwrap().0;
			let _ = crate::bls12_381::do_msm_g1(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(5)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_msm_g1_optimized(
			_origin: OriginFor<T>,
			bases: Vec<u8>,
			scalars: Vec<u8>,
		) -> DispatchResult {
			let bases = <ArkScale<Vec<bls12_381::G1AffineOptimized>> as Decode>::decode(
				&mut bases.as_slice(),
			)
			.unwrap()
			.0;
			let scalars = <ArkScale<
				Vec<<bls12_381::Bls12_381Optimized as Pairing>::ScalarField>,
			> as Decode>::decode(&mut scalars.as_slice())
			.unwrap().0;
			let _ = crate::bls12_381::do_msm_g1_optimized(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(6)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_msm_g2(
			_origin: OriginFor<T>,
			bases: Vec<u8>,
			scalars: Vec<u8>,
		) -> DispatchResult {
			let bases =
				<ArkScale<Vec<ark_bls12_381::G2Affine>> as Decode>::decode(&mut bases.as_slice())
					.unwrap()
					.0;
			let scalars = <ArkScale<
				Vec<<ark_bls12_381::Bls12_381 as Pairing>::ScalarField>,
			> as Decode>::decode(&mut scalars.as_slice())
			.unwrap().0;
			let _ = crate::bls12_381::do_msm_g2(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(7)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_msm_g2_optimized(
			_origin: OriginFor<T>,
			bases: Vec<u8>,
			scalars: Vec<u8>,
		) -> DispatchResult {
			let bases = <ArkScale<Vec<bls12_381::G2AffineOptimized>> as Decode>::decode(
				&mut bases.as_slice(),
			)
			.unwrap()
			.0;
			let scalars = <ArkScale<
				Vec<<ark_bls12_381::Bls12_381 as Pairing>::ScalarField>,
			> as Decode>::decode(&mut scalars.as_slice())
			.unwrap().0;
			let _ = crate::bls12_381::do_msm_g2_optimized(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(8)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_mul_projective_g1(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = <ArkScaleProjective<ark_bls12_381::G1Projective> as Decode>::decode(
				&mut base.as_slice(),
			)
			.unwrap()
			.0;
			let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).unwrap().0;
			let _ = crate::bls12_381::do_mul_projective_g1(&base, &scalar);
			Ok(())
		}

		#[pallet::call_index(9)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_mul_projective_g1_optimized(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = <ArkScaleProjective<bls12_381::G1ProjectiveOptimized> as Decode>::decode(
				&mut base.as_slice(),
			)
			.unwrap()
			.0;
			let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).unwrap().0;
			let _ = crate::bls12_381::do_mul_projective_g1_optimized(&base, &scalar);
			Ok(())
		}

		#[pallet::call_index(10)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_mul_affine_g1(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = <ArkScale<ark_bls12_381::G1Affine> as Decode>::decode(&mut base.as_slice())
				.unwrap()
				.0;
			let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).unwrap().0;
			let _ = crate::bls12_381::do_mul_affine_g1(&base, &scalar);
			Ok(())
		}

		#[pallet::call_index(11)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_mul_affine_g1_optimized(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base =
				<ArkScale<bls12_381::G1AffineOptimized> as Decode>::decode(&mut base.as_slice())
					.unwrap()
					.0;
			let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).unwrap().0;
			let _ = crate::bls12_381::do_mul_affine_g1_optimized(&base, &scalar);
			Ok(())
		}

		#[pallet::call_index(12)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_mul_projective_g2(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = <ArkScaleProjective<ark_bls12_381::G2Projective> as Decode>::decode(
				&mut base.as_slice(),
			)
			.unwrap()
			.0;
			let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).unwrap().0;
			let _ = crate::bls12_381::do_mul_projective_g2(&base, &scalar);
			Ok(())
		}

		#[pallet::call_index(13)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_mul_projective_g2_optimized(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = <ArkScaleProjective<bls12_381::G2ProjectiveOptimized> as Decode>::decode(
				&mut base.as_slice(),
			)
			.unwrap()
			.0;
			let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).unwrap().0;
			let _ = crate::bls12_381::do_mul_projective_g2_optimized(&base, &scalar);
			Ok(())
		}

		#[pallet::call_index(14)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_mul_affine_g2(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = <ArkScale<ark_bls12_381::G2Affine> as Decode>::decode(&mut base.as_slice())
				.unwrap()
				.0;
			let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).unwrap().0;
			let _ = crate::bls12_381::do_mul_affine_g2(&base, &scalar);
			Ok(())
		}

		#[pallet::call_index(15)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_mul_affine_g2_optimized(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base =
				<ArkScale<bls12_381::G2AffineOptimized> as Decode>::decode(&mut base.as_slice())
					.unwrap()
					.0;
			let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).unwrap().0;
			let _ = crate::bls12_381::do_mul_affine_g2_optimized(&base, &scalar);
			Ok(())
		}

		#[pallet::call_index(16)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_pairing(_origin: OriginFor<T>, a: Vec<u8>, b: Vec<u8>) -> DispatchResult {
			let a = <ArkScale<ark_bls12_377::G1Affine> as Decode>::decode(&mut a.as_slice())
				.unwrap()
				.0;
			let b = <ArkScale<ark_bls12_377::G2Affine> as Decode>::decode(&mut b.as_slice())
				.unwrap()
				.0;
			let _ = crate::bls12_377::do_pairing(a, b);
			Ok(())
		}

		#[pallet::call_index(17)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_pairing_optimized(
			_origin: OriginFor<T>,
			a: Vec<u8>,
			b: Vec<u8>,
		) -> DispatchResult {
			let a = <ArkScale<bls12_377::G1AffineOptimized> as Decode>::decode(&mut a.as_slice())
				.unwrap()
				.0;
			let b = <ArkScale<bls12_377::G2AffineOptimized> as Decode>::decode(&mut b.as_slice())
				.unwrap()
				.0;
			let _ = crate::bls12_377::do_pairing_optimized(a, b);
			Ok(())
		}

		#[pallet::call_index(18)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_msm_g1(
			_origin: OriginFor<T>,
			bases: Vec<u8>,
			scalars: Vec<u8>,
		) -> DispatchResult {
			let bases =
				<ArkScale<Vec<ark_bls12_377::G1Affine>> as Decode>::decode(&mut bases.as_slice())
					.unwrap()
					.0;
			let scalars = <ArkScale<
				Vec<<ark_bls12_377::Bls12_377 as Pairing>::ScalarField>,
			> as Decode>::decode(&mut scalars.as_slice())
			.unwrap().0;
			let _ = crate::bls12_377::do_msm_g1(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(19)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_msm_g1_optimized(
			_origin: OriginFor<T>,
			bases: Vec<u8>,
			scalars: Vec<u8>,
		) -> DispatchResult {
			let bases = <ArkScale<Vec<bls12_377::G1AffineOptimized>> as Decode>::decode(
				&mut bases.as_slice(),
			)
			.unwrap()
			.0;
			let scalars = <ArkScale<
				Vec<<bls12_377::Bls12_377Optimized as Pairing>::ScalarField>,
			> as Decode>::decode(&mut scalars.as_slice())
			.unwrap()
			.0;
			let _ = crate::bls12_377::do_msm_g1_optimized(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(20)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_msm_g2(
			_origin: OriginFor<T>,
			bases: Vec<u8>,
			scalars: Vec<u8>,
		) -> DispatchResult {
			let bases =
				<ArkScale<Vec<ark_bls12_377::G2Affine>> as Decode>::decode(&mut bases.as_slice())
					.unwrap()
					.0;
			let scalars = <ArkScale<
				Vec<<ark_bls12_377::Bls12_377 as Pairing>::ScalarField>,
			> as Decode>::decode(&mut scalars.as_slice())
			.unwrap().0;
			let _ = crate::bls12_377::do_msm_g2(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(21)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_msm_g2_optimized(
			_origin: OriginFor<T>,
			bases: Vec<u8>,
			scalars: Vec<u8>,
		) -> DispatchResult {
			let bases = <ArkScale<Vec<bls12_377::G2AffineOptimized>> as Decode>::decode(
				&mut bases.as_slice(),
			)
			.unwrap()
			.0;
			let scalars = <ArkScale<
				Vec<<bls12_377::Bls12_377Optimized as Pairing>::ScalarField>,
			> as Decode>::decode(&mut scalars.as_slice())
			.unwrap()
			.0;
			let _ = crate::bls12_377::do_msm_g2_optimized(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(22)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_mul_projective_g1(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = <ArkScaleProjective<ark_bls12_377::G1Projective> as Decode>::decode(
				&mut base.as_slice(),
			)
			.unwrap()
			.0;
			let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).unwrap().0;
			let _ = crate::bls12_377::do_mul_projective_g1(&base, &scalar);
			Ok(())
		}

		#[pallet::call_index(23)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_mul_projective_g1_optimized(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = <ArkScaleProjective<bls12_377::G1ProjectiveOptimized> as Decode>::decode(
				&mut base.as_slice(),
			)
			.unwrap()
			.0;
			let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).unwrap().0;
			let _ = crate::bls12_377::do_mul_projective_g1_optimized(&base, &scalar);
			Ok(())
		}

		#[pallet::call_index(24)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_mul_affine_g1(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = <ArkScale<ark_bls12_377::G1Affine> as Decode>::decode(&mut base.as_slice())
				.unwrap()
				.0;
			let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).unwrap().0;
			let _ = crate::bls12_377::do_mul_affine_g1(&base, &scalar);
			Ok(())
		}

		#[pallet::call_index(25)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_mul_affine_g1_optimized(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base =
				<ArkScale<bls12_377::G1AffineOptimized> as Decode>::decode(&mut base.as_slice())
					.unwrap()
					.0;
			let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).unwrap().0;
			let _ = crate::bls12_377::do_mul_affine_g1_optimized(&base, &scalar);
			Ok(())
		}

		#[pallet::call_index(26)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_mul_projective_g2(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = <ArkScaleProjective<ark_bls12_377::G2Projective> as Decode>::decode(
				&mut base.as_slice(),
			)
			.unwrap()
			.0;
			let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).unwrap().0;
			let _ = crate::bls12_377::do_mul_projective_g2(&base, &scalar);
			Ok(())
		}

		#[pallet::call_index(27)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_mul_projective_g2_optimized(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = <ArkScaleProjective<bls12_377::G2ProjectiveOptimized> as Decode>::decode(
				&mut base.as_slice(),
			)
			.unwrap()
			.0;
			let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).unwrap().0;
			let _ = crate::bls12_377::do_mul_projective_g2_optimized(&base, &scalar);
			Ok(())
		}

		#[pallet::call_index(28)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_mul_affine_g2(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = <ArkScale<ark_bls12_377::G2Affine> as Decode>::decode(&mut base.as_slice())
				.unwrap()
				.0;
			let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).unwrap().0;
			let _ = crate::bls12_377::do_mul_affine_g2(&base, &scalar);
			Ok(())
		}

		#[pallet::call_index(29)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_mul_affine_g2_optimized(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base =
				<ArkScale<bls12_377::G2AffineOptimized> as Decode>::decode(&mut base.as_slice())
					.unwrap()
					.0;
			let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).unwrap().0;
			let _ = crate::bls12_377::do_mul_affine_g2_optimized(&base, &scalar);
			Ok(())
		}

		#[pallet::call_index(30)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_pairing(_origin: OriginFor<T>, a: Vec<u8>, b: Vec<u8>) -> DispatchResult {
			let a = <ArkScale<ark_bw6_761::G1Affine> as Decode>::decode(&mut a.as_slice())
				.unwrap()
				.0;
			let b = <ArkScale<ark_bw6_761::G2Affine> as Decode>::decode(&mut b.as_slice())
				.unwrap()
				.0;
			let _ = crate::bw6_761::do_pairing(a, b);
			Ok(())
		}

		#[pallet::call_index(31)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_pairing_optimized(
			_origin: OriginFor<T>,
			a: Vec<u8>,
			b: Vec<u8>,
		) -> DispatchResult {
			let a = <ArkScale<bw6_761::G1AffineOptimized> as Decode>::decode(&mut a.as_slice())
				.unwrap()
				.0;
			let b = <ArkScale<bw6_761::G2AffineOptimized> as Decode>::decode(&mut b.as_slice())
				.unwrap()
				.0;
			let _ = crate::bw6_761::do_pairing_optimized(a, b);
			Ok(())
		}

		#[pallet::call_index(32)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_msm_g1(
			_origin: OriginFor<T>,
			bases: Vec<u8>,
			scalars: Vec<u8>,
		) -> DispatchResult {
			let bases =
				<ArkScale<Vec<ark_bw6_761::G1Affine>> as Decode>::decode(&mut bases.as_slice())
					.unwrap()
					.0;
			let scalars = <ArkScale<
				Vec<<ark_bw6_761::g1::Config as CurveConfig>::ScalarField>,
			> as Decode>::decode(&mut scalars.as_slice())
			.unwrap().0;
			let _ = crate::bw6_761::do_msm_g1(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(33)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_msm_g1_optimized(
			_origin: OriginFor<T>,
			bases: Vec<u8>,
			scalars: Vec<u8>,
		) -> DispatchResult {
			let bases = <ArkScale<Vec<bw6_761::G1AffineOptimized>> as Decode>::decode(
				&mut bases.as_slice(),
			)
			.unwrap()
			.0;
			let scalars = <ArkScale<
				Vec<<ark_bw6_761::g2::Config as CurveConfig>::ScalarField>,
			> as Decode>::decode(&mut scalars.as_slice())
			.unwrap().0;
			let _ = crate::bw6_761::do_msm_g1_optimized(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(34)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_msm_g2(
			_origin: OriginFor<T>,
			bases: Vec<u8>,
			scalars: Vec<u8>,
		) -> DispatchResult {
			let bases =
				<ArkScale<Vec<ark_bw6_761::G2Affine>> as Decode>::decode(&mut bases.as_slice())
					.unwrap()
					.0;
			let scalars = <ArkScale<
				Vec<<ark_bw6_761::g2::Config as CurveConfig>::ScalarField>,
			> as Decode>::decode(&mut scalars.as_slice())
			.unwrap().0;
			let _ = crate::bw6_761::do_msm_g2(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(35)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_msm_g2_optimized(
			_origin: OriginFor<T>,
			bases: Vec<u8>,
			scalars: Vec<u8>,
		) -> DispatchResult {
			let bases = <ArkScale<Vec<bw6_761::G2AffineOptimized>> as Decode>::decode(
				&mut bases.as_slice(),
			)
			.unwrap()
			.0;
			let scalars = <ArkScale<
				Vec<<ark_bw6_761::g2::Config as CurveConfig>::ScalarField>,
			> as Decode>::decode(&mut scalars.as_slice())
			.unwrap().0;
			let _ = crate::bw6_761::do_msm_g2_optimized(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(36)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_mul_projective_g1(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = <ArkScaleProjective<ark_bw6_761::G1Projective> as Decode>::decode(
				&mut base.as_slice(),
			)
			.unwrap()
			.0;
			let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).unwrap().0;
			let _ = crate::bw6_761::do_mul_projective_g1(&base, &scalar);
			Ok(())
		}

		#[pallet::call_index(37)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_mul_projective_g1_optimized(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = <ArkScaleProjective<bw6_761::G1ProjectiveOptimized> as Decode>::decode(
				&mut base.as_slice(),
			)
			.unwrap()
			.0;
			let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).unwrap().0;
			let _ = crate::bw6_761::do_mul_projective_g1_optimized(&base, &scalar);
			Ok(())
		}

		#[pallet::call_index(38)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_mul_affine_g1(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = <ArkScale<ark_bw6_761::G1Affine> as Decode>::decode(&mut base.as_slice())
				.unwrap()
				.0;
			let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).unwrap().0;
			let _ = crate::bw6_761::do_mul_affine_g1(&base, &scalar);
			Ok(())
		}

		#[pallet::call_index(39)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_mul_affine_g1_optimized(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base =
				<ArkScale<bw6_761::G1AffineOptimized> as Decode>::decode(&mut base.as_slice())
					.unwrap()
					.0;
			let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).unwrap().0;
			let _ = crate::bw6_761::do_mul_affine_g1_optimized(&base, &scalar);
			Ok(())
		}

		#[pallet::call_index(40)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_mul_projective_g2(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = <ArkScaleProjective<ark_bw6_761::G2Projective> as Decode>::decode(
				&mut base.as_slice(),
			)
			.unwrap()
			.0;
			let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).unwrap().0;
			let _ = crate::bw6_761::do_mul_projective_g2(&base, &scalar);
			Ok(())
		}

		#[pallet::call_index(41)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_mul_projective_g2_optimized(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = <ArkScaleProjective<bw6_761::G2ProjectiveOptimized> as Decode>::decode(
				&mut base.as_slice(),
			)
			.unwrap()
			.0;
			let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).unwrap().0;
			let _ = crate::bw6_761::do_mul_projective_g2_optimized(&base, &scalar);
			Ok(())
		}

		#[pallet::call_index(42)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_mul_affine_g2(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = <ArkScale<ark_bw6_761::G2Affine> as Decode>::decode(&mut base.as_slice())
				.unwrap()
				.0;
			let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).unwrap().0;
			let _ = crate::bw6_761::do_mul_affine_g2(&base, &scalar);
			Ok(())
		}

		#[pallet::call_index(43)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_mul_affine_g2_optimized(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base =
				<ArkScale<bw6_761::G2AffineOptimized> as Decode>::decode(&mut base.as_slice())
					.unwrap()
					.0;
			let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).unwrap().0;
			let _ = crate::bw6_761::do_mul_affine_g2_optimized(&base, &scalar);
			Ok(())
		}

		#[pallet::call_index(44)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_381_bandersnatch_msm_sw(
			_origin: OriginFor<T>,
			bases: Vec<u8>,
			scalars: Vec<u8>,
		) -> DispatchResult {
			let bases =
				<ArkScale<Vec<ark_ed_on_bls12_381_bandersnatch::SWAffine>> as Decode>::decode(
					&mut bases.as_slice(),
				)
				.unwrap()
				.0;
			let scalars = <ArkScale<
				Vec<<ark_ed_on_bls12_381_bandersnatch::BandersnatchConfig as CurveConfig>::ScalarField>,
			> as Decode>::decode(&mut scalars.as_slice())
			.unwrap()
			.0;
			let _ = crate::ed_on_bls12_381_bandersnatch::do_msm_sw(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(45)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_381_bandersnatch_msm_sw_optimized(
			_origin: OriginFor<T>,
			bases: Vec<u8>,
			scalars: Vec<u8>,
		) -> DispatchResult {
			let bases =
				<ArkScale<Vec<ed_on_bls12_381_bandersnatch::SWAffineOptimized>> as Decode>::decode(
					&mut bases.as_slice(),
				)
				.unwrap()
				.0;
			let scalars = <ArkScale<
				Vec<<ark_ed_on_bls12_381_bandersnatch::BandersnatchConfig as CurveConfig>::ScalarField>,
			> as Decode>::decode(&mut scalars.as_slice())
			.unwrap()
			.0;
			let _ =
				crate::ed_on_bls12_381_bandersnatch::do_msm_sw_optimized(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(46)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_381_bandersnatch_msm_te(
			_origin: OriginFor<T>,
			bases: Vec<u8>,
			scalars: Vec<u8>,
		) -> DispatchResult {
			let bases =
				<ArkScale<Vec<ark_ed_on_bls12_381_bandersnatch::EdwardsAffine>> as Decode>::decode(
					&mut bases.as_slice(),
				)
				.unwrap()
				.0;
			let scalars = <ArkScale<
				Vec<<ark_ed_on_bls12_381_bandersnatch::BandersnatchConfig as ark_ec::models::CurveConfig>::ScalarField>,
			> as Decode>::decode(&mut scalars.as_slice())
			.unwrap()
			.0;
			let _ = crate::ed_on_bls12_381_bandersnatch::do_msm_te(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(47)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_381_bandersnatch_msm_te_optimized(
			_origin: OriginFor<T>,
			bases: Vec<u8>,
			scalars: Vec<u8>,
		) -> DispatchResult {
			let bases =
				<ArkScale<Vec<ed_on_bls12_381_bandersnatch::EdwardsAffineOptimized>> as Decode>::decode(
					&mut bases.as_slice(),
				)
				.unwrap()
				.0;
			let scalars = <ArkScale<
				Vec<<ed_on_bls12_381_bandersnatch::BandersnatchConfig as CurveConfig>::ScalarField>,
			> as Decode>::decode(&mut scalars.as_slice())
			.unwrap()
			.0;
			let _ =
				crate::ed_on_bls12_381_bandersnatch::do_msm_te_optimized(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(48)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_381_bandersnatch_mul_projective_sw(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = <ArkScaleProjective<ark_ed_on_bls12_381_bandersnatch::SWProjective> as Decode>::decode(
				&mut base.as_slice(),
			)
			.unwrap()
			.0;
			let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).unwrap().0;
			let _ = crate::ed_on_bls12_381_bandersnatch::do_mul_projective_sw(&base, &scalar);
			Ok(())
		}

		#[pallet::call_index(49)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_381_bandersnatch_mul_projective_sw_optimized(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = <ArkScaleProjective<ed_on_bls12_381_bandersnatch::SWProjectiveOptimized> as Decode>::decode(
				&mut base.as_slice(),
			)
			.unwrap()
			.0;
			let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).unwrap().0;
			let _ =
				crate::ed_on_bls12_381_bandersnatch::do_mul_projective_sw_optimized(&base, &scalar);
			Ok(())
		}

		#[pallet::call_index(50)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_381_bandersnatch_mul_projective_te(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = <ArkScaleProjective<ark_ed_on_bls12_381_bandersnatch::EdwardsProjective> as Decode>::decode(
				&mut base.as_slice(),
			)
			.unwrap()
			.0;
			let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).unwrap().0;
			let _ = crate::ed_on_bls12_381_bandersnatch::do_mul_projective_te(&base, &scalar);
			Ok(())
		}

		#[pallet::call_index(51)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_381_bandersnatch_mul_projective_te_optimized(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base =
				<ArkScaleProjective<ed_on_bls12_381_bandersnatch::EdwardsProjectiveOptimized> as Decode>::decode(
					&mut base.as_slice(),
				)
				.unwrap()
				.0;
			let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).unwrap().0;
			let _ =
				crate::ed_on_bls12_381_bandersnatch::do_mul_projective_te_optimized(&base, &scalar);
			Ok(())
		}

		#[pallet::call_index(52)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_381_bandersnatch_mul_affine_sw(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = <ArkScale<ark_ed_on_bls12_381_bandersnatch::SWAffine> as Decode>::decode(
				&mut base.as_slice(),
			)
			.unwrap()
			.0;
			let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).unwrap().0;
			let _ = crate::ed_on_bls12_381_bandersnatch::do_mul_affine_sw(&base, &scalar);
			Ok(())
		}

		#[pallet::call_index(53)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_381_bandersnatch_mul_affine_sw_optimized(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base =
				<ArkScale<ed_on_bls12_381_bandersnatch::SWAffineOptimized> as Decode>::decode(
					&mut base.as_slice(),
				)
				.unwrap()
				.0;
			let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).unwrap().0;
			let _ = crate::ed_on_bls12_381_bandersnatch::do_mul_affine_sw_optimized(&base, &scalar);
			Ok(())
		}

		#[pallet::call_index(54)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_381_bandersnatch_mul_affine_te(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base =
				<ArkScale<ark_ed_on_bls12_381_bandersnatch::EdwardsAffine> as Decode>::decode(
					&mut base.as_slice(),
				)
				.unwrap()
				.0;
			let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).unwrap().0;
			let _ = crate::ed_on_bls12_381_bandersnatch::do_mul_affine_te(&base, &scalar);
			Ok(())
		}

		#[pallet::call_index(55)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_381_bandersnatch_mul_affine_te_optimized(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base =
				<ArkScale<ed_on_bls12_381_bandersnatch::EdwardsAffineOptimized> as Decode>::decode(
					&mut base.as_slice(),
				)
				.unwrap()
				.0;
			let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).unwrap().0;
			let _ = crate::ed_on_bls12_381_bandersnatch::do_mul_affine_te_optimized(&base, &scalar);
			Ok(())
		}

		#[pallet::call_index(56)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_377_msm(
			_origin: OriginFor<T>,
			bases: Vec<u8>,
			scalars: Vec<u8>,
		) -> DispatchResult {
			let bases = <ArkScale<Vec<ark_ed_on_bls12_377::EdwardsAffine>> as Decode>::decode(
				&mut bases.as_slice(),
			)
			.unwrap()
			.0;
			let scalars = <ArkScale<Vec<Fp<MontBackend<ark_ed_on_bls12_377::FrConfig, 4>, 4>>> as Decode>::decode(
				&mut scalars.as_slice(),
			)
			.unwrap()
			.0;
			let _ = crate::ed_on_bls12_377::do_msm(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(57)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_377_msm_optimized(
			_origin: OriginFor<T>,
			bases: Vec<u8>,
			scalars: Vec<u8>,
		) -> DispatchResult {
			let bases = <ArkScale<Vec<ed_on_bls12_377::EdwardsAffineOptimized>> as Decode>::decode(
				&mut bases.as_slice(),
			)
			.unwrap()
			.0;
			let scalars = <ArkScale<Vec<Fp<MontBackend<ark_ed_on_bls12_377::FrConfig, 4>, 4>>> as Decode>::decode(
				&mut scalars.as_slice(),
			)
			.unwrap()
			.0;
			let _ = crate::ed_on_bls12_377::do_msm_optimized(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(58)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_377_mul_projective(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base =
				<ArkScaleProjective<ark_ed_on_bls12_377::EdwardsProjective> as Decode>::decode(
					&mut base.as_slice(),
				)
				.unwrap()
				.0;
			let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).unwrap().0;
			let _ = crate::ed_on_bls12_377::do_mul_projective(&base, &scalar);
			Ok(())
		}

		#[pallet::call_index(59)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_377_mul_projective_optimized(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = <ArkScaleProjective<ed_on_bls12_377::EdwardsProjectiveOptimized> as Decode>::decode(
				&mut base.as_slice(),
			)
			.unwrap().0;
			let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).unwrap().0;
			let _ = crate::ed_on_bls12_377::do_mul_projective_optimized(&base, &scalar);
			Ok(())
		}

		#[pallet::call_index(60)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_377_mul_affine(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = <ArkScale<ark_ed_on_bls12_377::EdwardsAffine> as Decode>::decode(
				&mut base.as_slice(),
			)
			.unwrap()
			.0;
			let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).unwrap().0;
			let _ = crate::ed_on_bls12_377::do_mul_affine(&base, &scalar);
			Ok(())
		}

		#[pallet::call_index(61)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_377_mul_affine_optimized(
			_origin: OriginFor<T>,
			base: Vec<u8>,
			scalar: Vec<u8>,
		) -> DispatchResult {
			let base = <ArkScale<ed_on_bls12_377::EdwardsAffineOptimized> as Decode>::decode(
				&mut base.as_slice(),
			)
			.unwrap()
			.0;
			let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).unwrap().0;
			let _ = crate::ed_on_bls12_377::do_mul_affine_optimized(&base, &scalar);
			Ok(())
		}
	}
}
