//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	verify_groth16 {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	verify_groth16_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	pairing_arkworks_bls12_381 {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	msm_g1_bls12_381 {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	msm_g2_bls12_381 {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	mul_affine_g1_bls12_381 {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	mul_projective_g1_bls12_381 {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	mul_affine_g2_bls12_381 {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	mul_projective_g2_bls12_381 {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	mul_affine_g1_bls12_377 {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	mul_projective_g1_bls12_377 {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	mul_affine_g2_bls12_377 {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	mul_projective_g2_bls12_377 {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	pairing_arkworks_bls12_377 {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	msm_g2_bw6_761 {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	mul_affine_g1_bw6_761 {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	mul_projective_g1_bw6_761 {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	mul_affine_g2_bw6_761 {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	mul_projective_g2_bw6_761 {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	pairing_arkworks_bw6_761 {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))


	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
