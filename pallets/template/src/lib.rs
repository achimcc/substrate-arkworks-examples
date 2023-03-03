#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "1024"]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod bls12_377;
pub mod bls12_381;
pub mod bw6_761;
pub mod ed_on_bls12_377;
pub mod ed_on_bls12_381;
pub mod msm_arguments;
pub mod utils;

#[frame_support::pallet]
pub mod pallet {
	use crate::{
		bls12_377, bls12_381, bw6_761, ed_on_bls12_377, ed_on_bls12_381,
		msm_arguments::{generate_arguments_sw, generate_arguments_te},
	};
	use ark_serialize::{CanonicalDeserialize, Compress, Validate};
	use ark_std::{io::Cursor, vec, vec::Vec};
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Successfull groth16 verification event
		VerificationSuccess { who: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Verification of groth16 proof failed
		VerificationFailed,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn groth16_verification(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin).unwrap();

			let result = crate::bls12_381::do_verify_groth16();

			if result.is_ok() {
				Self::deposit_event(Event::VerificationSuccess { who });
				Ok(())
			} else {
				Err(Error::<T>::VerificationFailed.into())
			}
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn groth16_verification_optimized(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin).unwrap();

			let result = crate::bls12_381::do_verify_groth16_optimized();

			if result.is_ok() {
				Self::deposit_event(Event::VerificationSuccess { who });
				Ok(())
			} else {
				Err(Error::<T>::VerificationFailed.into())
			}
		}

		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_pairing(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::bls12_381::do_pairing();
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_pairing_optimized(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::bls12_381::do_pairing_optimized();
			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_msm_g1(
			_origin: OriginFor<T>,
			bases: Vec<Vec<u8>>,
			scalars: Vec<Vec<u8>>,
		) -> DispatchResult {
			let bases: Vec<_> = bases
				.iter()
				.map(|a| {
					let cursor = Cursor::new(a);
					<ark_bls12_381::Bls12_381 as ark_ec::pairing::Pairing>::G1Affine::deserialize_with_mode(
						cursor,
						Compress::No,
						Validate::No,
					)
					.unwrap()
				})
				.collect();
			let scalars: Vec<_> = scalars
				.iter()
				.map(|a| {
					let cursor = Cursor::new(a);
					<ark_bls12_381::g1::Config as ark_ec::CurveConfig>::ScalarField::deserialize_with_mode(
						cursor,
						Compress::No,
						Validate::No,
					)
					.unwrap()
				})
				.collect();
			let _ = crate::bls12_381::do_msm_g1(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(5)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_msm_g1_optimized(
			_origin: OriginFor<T>,
			bases: Vec<Vec<u8>>,
			scalars: Vec<Vec<u8>>,
		) -> DispatchResult {
			let bases: Vec<_> = bases
				.iter()
				.map(|a| {
					let cursor = Cursor::new(a);
					bls12_381::G1Affine_Host::deserialize_with_mode(
						cursor,
						Compress::No,
						Validate::No,
					)
					.unwrap()
				})
				.collect();
			let scalars: Vec<_> = scalars
				.iter()
				.map(|a| {
					let cursor = Cursor::new(a);
					<sp_ark_bls12_381::g1::Config<bls12_381::HostBls12_381> as sp_ark_models::CurveConfig>::ScalarField::deserialize_with_mode(
						cursor,
						Compress::No,
						Validate::No,
					)
					.unwrap()
				})
				.collect();
			let _ = crate::bls12_381::do_msm_g1_optimized(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(6)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_msm_g2(
			_origin: OriginFor<T>,
			bases: Vec<Vec<u8>>,
			scalars: Vec<Vec<u8>>,
		) -> DispatchResult {
			let bases: Vec<_> = bases
				.iter()
				.map(|a| {
					let cursor = Cursor::new(a);
					<ark_bls12_381::Bls12_381 as ark_ec::pairing::Pairing>::G2Affine::deserialize_with_mode(
						cursor,
						Compress::No,
						Validate::No,
					)
					.unwrap()
				})
				.collect();
			let scalars: Vec<_> = scalars
				.iter()
				.map(|a| {
					let cursor = Cursor::new(a);
					<ark_bls12_381::g2::Config as ark_ec::CurveConfig>::ScalarField::deserialize_with_mode(
						cursor,
						Compress::No,
						Validate::No,
					)
					.unwrap()
				})
				.collect();
			let _ = crate::bls12_381::do_msm_g2(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(7)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_msm_g2_optimized(
			_origin: OriginFor<T>,
			bases: Vec<Vec<u8>>,
			scalars: Vec<Vec<u8>>,
		) -> DispatchResult {
			let bases: Vec<_> = bases
				.iter()
				.map(|a| {
					let cursor = Cursor::new(a);
					bls12_381::G2AffineOptimized::deserialize_with_mode(
						cursor,
						Compress::No,
						Validate::No,
					)
					.unwrap()
				})
				.collect();
			let scalars: Vec<_> = scalars
				.iter()
				.map(|a| {
					let cursor = Cursor::new(a);
					<sp_ark_bls12_381::g1::Config<bls12_381::HostBls12_381> as sp_ark_models::CurveConfig>::ScalarField::deserialize_with_mode(
						cursor,
						Compress::No,
						Validate::No,
					)
					.unwrap()
				})
				.collect();
			let _ = crate::bls12_381::do_msm_g2_optimized(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(8)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_mul_projective_g1(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::bls12_381::do_mul_projective_g1();
			Ok(())
		}

		#[pallet::call_index(9)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_mul_projective_g1_optimized(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::bls12_381::do_mul_projective_g1_optimized();
			Ok(())
		}

		#[pallet::call_index(10)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_mul_affine_g1(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::bls12_381::do_mul_affine_g1();
			Ok(())
		}

		#[pallet::call_index(11)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_mul_affine_g1_optimized(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::bls12_381::do_mul_affine_g1_optimized();
			Ok(())
		}

		#[pallet::call_index(12)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_mul_projective_g2(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::bls12_381::do_mul_projective_g2();
			Ok(())
		}

		#[pallet::call_index(13)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_mul_projective_g2_optimized(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::bls12_381::do_mul_projective_g2_optimized();
			Ok(())
		}

		#[pallet::call_index(14)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_mul_affine_g2(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::bls12_381::do_mul_affine_g2();
			Ok(())
		}

		#[pallet::call_index(15)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_381_mul_affine_g2_optimized(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::bls12_381::do_mul_affine_g2_optimized();
			Ok(())
		}

		#[pallet::call_index(16)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_pairing(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::bls12_377::do_pairing();
			Ok(())
		}

		#[pallet::call_index(17)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_pairing_optimized(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::bls12_377::do_pairing_optimized();
			Ok(())
		}

		#[pallet::call_index(18)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_msm_g1(_origin: OriginFor<T>, samples: u32) -> DispatchResult {
			let (bases, scalars) = generate_arguments_sw::<ark_bls12_377::g1::Config>(samples);
			let _ = crate::bls12_377::do_msm_g1(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(19)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_msm_g1_optimized(_origin: OriginFor<T>, samples: u32) -> DispatchResult {
			let (bases, scalars) = generate_arguments_sw::<
				sp_ark_bls12_377::g1::Config<bls12_377::HostBls12_377>,
			>(samples);
			let _ = crate::bls12_377::do_msm_g1_optimized(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(20)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_msm_g2(_origin: OriginFor<T>, samples: u32) -> DispatchResult {
			let (bases, scalars) = generate_arguments_sw::<ark_bls12_377::g2::Config>(samples);
			let _ = crate::bls12_377::do_msm_g2(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(21)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_msm_g2_optimized(_origin: OriginFor<T>, samples: u32) -> DispatchResult {
			let (bases, scalars) = generate_arguments_sw::<
				sp_ark_bls12_377::g2::Config<bls12_377::HostBls12_377>,
			>(samples);
			let _ = crate::bls12_377::do_msm_g2_optimized(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(22)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_mul_projective_g1(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::bls12_377::do_mul_projective_g1();
			Ok(())
		}

		#[pallet::call_index(23)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_mul_projective_g1_optimized(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::bls12_377::do_mul_projective_g1_optimized();
			Ok(())
		}

		#[pallet::call_index(24)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_mul_affine_g1(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::bls12_377::do_mul_affine_g1();
			Ok(())
		}

		#[pallet::call_index(25)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_mul_affine_g1_optimized(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::bls12_377::do_mul_affine_g1_optimized();
			Ok(())
		}

		#[pallet::call_index(26)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_mul_projective_g2(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::bls12_377::do_mul_projective_g2();
			Ok(())
		}

		#[pallet::call_index(27)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_mul_projective_g2_optimized(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::bls12_377::do_mul_projective_g2_optimized();
			Ok(())
		}

		#[pallet::call_index(28)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_mul_affine_g2(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::bls12_377::do_mul_affine_g2();
			Ok(())
		}

		#[pallet::call_index(29)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bls12_377_mul_affine_g2_optimized(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::bls12_377::do_mul_affine_g2_optimized();
			Ok(())
		}

		#[pallet::call_index(30)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_pairing(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::bw6_761::do_pairing();
			Ok(())
		}

		#[pallet::call_index(31)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_pairing_optimized(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::bw6_761::do_pairing_optimized();
			Ok(())
		}

		#[pallet::call_index(32)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_msm_g1(_origin: OriginFor<T>, samples: u32) -> DispatchResult {
			let (bases, scalars) = generate_arguments_sw::<ark_bw6_761::g1::Config>(samples);
			let _ = crate::bw6_761::do_msm_g1(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(33)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_msm_g1_optimized(_origin: OriginFor<T>, samples: u32) -> DispatchResult {
			let (bases, scalars) =
				generate_arguments_sw::<sp_ark_bw6_761::g1::Config<bw6_761::HostBW6_761>>(samples);
			let _ = crate::bw6_761::do_msm_g1_optimized(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(34)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_msm_g2(_origin: OriginFor<T>, samples: u32) -> DispatchResult {
			let (bases, scalars) = generate_arguments_sw::<ark_bw6_761::g2::Config>(samples);
			let _ = crate::bw6_761::do_msm_g2(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(35)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_msm_g2_optimized(_origin: OriginFor<T>, samples: u32) -> DispatchResult {
			let (bases, scalars) =
				generate_arguments_sw::<sp_ark_bw6_761::g2::Config<bw6_761::HostBW6_761>>(samples);
			let _ = crate::bw6_761::do_msm_g2_optimized(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(36)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_mul_projective_g1(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::bw6_761::do_mul_projective_g1();
			Ok(())
		}

		#[pallet::call_index(37)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_mul_projective_g1_optimized(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::bw6_761::do_mul_projective_g1_optimized();
			Ok(())
		}

		#[pallet::call_index(38)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_mul_affine_g1(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::bw6_761::do_mul_affine_g1();
			Ok(())
		}

		#[pallet::call_index(39)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_mul_affine_g1_optimized(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::bw6_761::do_mul_affine_g1_optimized();
			Ok(())
		}

		#[pallet::call_index(40)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_mul_projective_g2(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::bw6_761::do_mul_projective_g2();
			Ok(())
		}

		#[pallet::call_index(41)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_mul_projective_g2_optimized(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::bw6_761::do_mul_projective_g2_optimized();
			Ok(())
		}

		#[pallet::call_index(42)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_mul_affine_g2(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::bw6_761::do_mul_affine_g2();
			Ok(())
		}

		#[pallet::call_index(43)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn bw6_761_mul_affine_g2_optimized(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::bw6_761::do_mul_affine_g2_optimized();
			Ok(())
		}

		#[pallet::call_index(44)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_381_msm_sw(_origin: OriginFor<T>, samples: u32) -> DispatchResult {
			let (bases, scalars) = generate_arguments_sw::<ark_ed_on_bls12_381::SWConfig>(samples);
			let _ = crate::ed_on_bls12_381::do_msm_sw(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(45)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_381_msm_sw_optimized(
			_origin: OriginFor<T>,
			samples: u32,
		) -> DispatchResult {
			let (bases, scalars) = generate_arguments_sw::<
				sp_ark_ed_on_bls12_381::SWConfig<ed_on_bls12_381::HostEdOnBls12_381>,
			>(samples);
			let _ = crate::ed_on_bls12_381::do_msm_sw_optimized(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(46)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_381_msm_te(_origin: OriginFor<T>, samples: u32) -> DispatchResult {
			// let Test = sp_ark_models::Affine<sp_ark_ed_on_bls12_381::JubjubConfig>;
			let (bases, scalars) =
				generate_arguments_te::<ark_ed_on_bls12_381::EdwardsConfig>(samples);
			let _ = crate::ed_on_bls12_381::do_msm_te(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(47)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_381_msm_te_optimized(
			_origin: OriginFor<T>,
			samples: u32,
		) -> DispatchResult {
			let (bases, scalars) = generate_arguments_te::<
				sp_ark_ed_on_bls12_381::EdwardsConfig<ed_on_bls12_381::HostEdOnBls12_381>,
			>(samples);
			let _ = crate::ed_on_bls12_381::do_msm_te_optimized(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(48)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_381_mul_projective_sw(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::ed_on_bls12_381::do_mul_projective_sw();
			Ok(())
		}

		#[pallet::call_index(49)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_381_mul_projective_sw_optimized(
			_origin: OriginFor<T>,
		) -> DispatchResult {
			let _ = crate::ed_on_bls12_381::do_mul_projective_sw_optimized();
			Ok(())
		}

		#[pallet::call_index(50)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_381_mul_projective_te(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::ed_on_bls12_381::do_mul_projective_te();
			Ok(())
		}

		#[pallet::call_index(51)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_381_mul_projective_te_optimized(
			_origin: OriginFor<T>,
		) -> DispatchResult {
			let _ = crate::ed_on_bls12_381::do_mul_projective_te_optimized();
			Ok(())
		}

		#[pallet::call_index(52)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_381_mul_affine_sw(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::ed_on_bls12_381::do_mul_affine_sw();
			Ok(())
		}

		#[pallet::call_index(53)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_381_mul_affine_sw_optimized(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::ed_on_bls12_381::do_mul_affine_sw_optimized();
			Ok(())
		}

		#[pallet::call_index(54)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_381_mul_affine_te(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::ed_on_bls12_381::do_mul_affine_te();
			Ok(())
		}

		#[pallet::call_index(55)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_381_mul_affine_te_optimized(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::ed_on_bls12_381::do_mul_affine_te_optimized();
			Ok(())
		}

		#[pallet::call_index(56)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_377_msm(_origin: OriginFor<T>, samples: u32) -> DispatchResult {
			let (bases, scalars) =
				generate_arguments_te::<ark_ed_on_bls12_377::EdwardsConfig>(samples);
			let _ = crate::ed_on_bls12_377::do_msm(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(57)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_377_msm_optimized(
			_origin: OriginFor<T>,
			samples: u32,
		) -> DispatchResult {
			let (bases, scalars) = generate_arguments_te::<
				sp_ark_ed_on_bls12_377::EdwardsConfig<ed_on_bls12_377::HostEdOnBls12_377>,
			>(samples);
			let _ = crate::ed_on_bls12_377::do_msm_optimized(&bases[..], &scalars[..]);
			Ok(())
		}

		#[pallet::call_index(58)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_377_mul_projective(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::ed_on_bls12_377::do_mul_projective();
			Ok(())
		}

		#[pallet::call_index(59)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_377_mul_projective_optimized(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::ed_on_bls12_377::do_mul_projective_optimized();
			Ok(())
		}

		#[pallet::call_index(60)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_377_mul_affine(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::ed_on_bls12_377::do_mul_affine();
			Ok(())
		}

		#[pallet::call_index(61)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn ed_on_bls12_377_mul_affine_optimized(_origin: OriginFor<T>) -> DispatchResult {
			let _ = crate::ed_on_bls12_377::do_mul_affine_optimized();
			Ok(())
		}
	}
}
