//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	groth16_verification {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	groth16_verification_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bls12_381_pairing {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bls12_381_pairing_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bls12_381_msm_g1 {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bls12_381_msm_g1_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bls12_381_msm_g2 {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bls12_381_msm_g2_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bls12_381_mul_projective_g1 {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bls12_381_mul_projective_g1_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bls12_381_mul_affine_g1 {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bls12_381_mul_affine_g1_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bls12_381_mul_projective_g2 {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bls12_381_mul_projective_g2_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bls12_381_mul_affine_g2 {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bls12_381_mul_affine_g2_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bls12_377_pairing {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bls12_377_pairing_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bls12_377_msm_g1 {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bls12_377_msm_g1_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bls12_377_msm_g2 {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bls12_377_msm_g2_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bls12_377_mul_projective_g1 {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bls12_377_mul_projective_g1_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bls12_377_mul_affine_g1 {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bls12_377_mul_affine_g1_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bls12_377_mul_projective_g2 {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bls12_377_mul_projective_g2_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bls12_377_mul_affine_g2 {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bls12_377_mul_affine_g2_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bw6_761_pairing {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bw6_761_pairing_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bw6_761_msm_g1 {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bw6_761_msm_g1_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bw6_761_msm_g2 {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bw6_761_msm_g2_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bw6_761_mul_projective_g1 {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bw6_761_mul_projective_g1_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bw6_761_mul_affine_g1 {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bw6_761_mul_affine_g1_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bw6_761_mul_projective_g2 {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bw6_761_mul_projective_g2_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bw6_761_mul_affine_g2 {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	bw6_761_mul_affine_g2_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	ed_on_bls12_381_msm {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	ed_on_bls12_381_msm_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	ed_on_bls12_381_mul_projective {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	ed_on_bls12_381_mul_projective_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	ed_on_bls12_381_mul_affine {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	ed_on_bls12_381_mul_affine_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	ed_on_bls12_377_msm {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	ed_on_bls12_377_msm_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	ed_on_bls12_377_mul_projective {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	ed_on_bls12_377_mul_projective_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	ed_on_bls12_377_mul_affine {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	ed_on_bls12_377_mul_affine_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
