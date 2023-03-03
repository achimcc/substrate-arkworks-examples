//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use ark_serialize::CanonicalSerialize;
use ark_std::vec::Vec;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use msm_arguments::{generate_arguments_sw, generate_arguments_te};
use utils::serialize_argument;

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

	bls12_381_msm_g1_10 {
		let caller: T::AccountId = whitelisted_caller();
		let (bases, scalars) = generate_arguments_sw::<ark_bls12_381::g1::Config>(10);
		let bases = bases.iter().map(|base| serialize_argument(bases)).collect::<Vec<_>>();
		let scalars = scalars.iter().map(|scalar| serialize_argument(scalar)).collect::<Vec<_>>();
	}: bls12_381_msm_g1(RawOrigin::Signed(caller), bases, scalars)

	bls12_381_msm_g1_10_optimized {
		let caller: T::AccountId = whitelisted_caller();
		let (bases, scalars) = generate_arguments_sw::<ark_bls12_381::g1::Config>(10);
		let bases = bases.iter().map(|base| serialize_argument(bases)).collect::<Vec<_>>();
		let scalars = scalars.iter().map(|scalar| serialize_argument(scalar)).collect::<Vec<_>>();
	}: bls12_381_msm_g1_optimized(RawOrigin::Signed(caller), bases, scalars)

	bls12_381_msm_g1_1000 {
		let caller: T::AccountId = whitelisted_caller();
		let (bases, scalars) = generate_arguments_sw::<ark_bls12_381::g1::Config>(1000);
		let bases = bases.iter().map(|base| serialize_argument(bases)).collect::<Vec<_>>();
		let scalars = scalars.iter().map(|scalar| serialize_argument(scalar)).collect::<Vec<_>>();
	}: bls12_381_msm_g1(RawOrigin::Signed(caller), bases, scalars)

	bls12_381_msm_g1_1000_optimized {
		let caller: T::AccountId = whitelisted_caller();
		let (bases, scalars) = generate_arguments_sw::<sp_ark_bls12_381::g1::Config<bls12_381::HostBls12_381>>(1000);
		let bases = bases.iter().map(|base| serialize_argument(bases)).collect::<Vec<_>>();
		let scalars = scalars.iter().map(|scalar| serialize_argument(scalar)).collect::<Vec<_>>();
	}: bls12_381_msm_g1_optimized(RawOrigin::Signed(caller), bases, scalars)

	bls12_381_msm_g2_10 {
		let caller: T::AccountId = whitelisted_caller();
		let (bases, scalars) = generate_arguments_sw::<ark_bls12_381::g2::Config>(10);
		let bases = bases.iter().map(|base| serialize_argument(bases)).collect::<Vec<_>>();
		let scalars = scalars.iter().map(|scalar| serialize_argument(scalar)).collect::<Vec<_>>();
	}: bls12_381_msm_g2(RawOrigin::Signed(caller), bases, scalars)

	bls12_381_msm_g2_10_optimized {
		let caller: T::AccountId = whitelisted_caller();
		let (bases, scalars) = generate_arguments_sw::<sp_ark_bls12_381::g2::Config<bls12_381::HostBls12_381>>(1000);
		let bases = bases.iter().map(|base| serialize_argument(bases)).collect::<Vec<_>>();
		let scalars = scalars.iter().map(|scalar| serialize_argument(scalar)).collect::<Vec<_>>();
	}: bls12_381_msm_g2_optimized(RawOrigin::Signed(caller), bases, scalars)

	bls12_381_msm_g2_1000 {
		let caller: T::AccountId = whitelisted_caller();
		let (bases, scalars) = generate_arguments_sw::<ark_bls12_381::g2::Config>(1000);
		let bases = bases.iter().map(|base| serialize_argument(bases)).collect::<Vec<_>>();
		let scalars = scalars.iter().map(|scalar| serialize_argument(scalar)).collect::<Vec<_>>();
	}: bls12_381_msm_g2(RawOrigin::Signed(caller), bases, scalars)

	bls12_381_msm_g2_1000_optimized {
		let caller: T::AccountId = whitelisted_caller();
		let (bases, scalars) = generate_arguments_sw::<sp_ark_bls12_381::g2::Config<bls12_381::Host_Bls12_381>>(1000);
		let bases = bases.iter().map(|base| serialize_argument(bases)).collect::<Vec<_>>();
		let scalars = scalars.iter().map(|scalar| serialize_argument(scalar)).collect::<Vec<_>>();
	}: bls12_381_msm_g2_optimized(RawOrigin::Signed(caller), , bases, scalars)

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

	bls12_377_msm_g1_10 {
		let caller: T::AccountId = whitelisted_caller();
	}: bls12_377_msm_g1(RawOrigin::Signed(caller), 10)

	bls12_377_msm_g1_10_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: bls12_377_msm_g1_optimized(RawOrigin::Signed(caller), 10)

	bls12_377_msm_g1_1000 {
		let caller: T::AccountId = whitelisted_caller();
	}: bls12_377_msm_g1(RawOrigin::Signed(caller), 1000)

	bls12_377_msm_g1_1000_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: bls12_377_msm_g1_optimized(RawOrigin::Signed(caller), 1000)

	bls12_377_msm_g2_10 {
		let caller: T::AccountId = whitelisted_caller();
	}: bls12_377_msm_g2(RawOrigin::Signed(caller), 10)

	bls12_377_msm_g2_10_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: bls12_377_msm_g2_optimized(RawOrigin::Signed(caller), 10)

	bls12_377_msm_g2_1000 {
		let caller: T::AccountId = whitelisted_caller();
	}: bls12_377_msm_g2(RawOrigin::Signed(caller), 1000)

	bls12_377_msm_g2_1000_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: bls12_377_msm_g2_optimized(RawOrigin::Signed(caller), 1000)

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

	bw6_761_msm_g1_10 {
		let caller: T::AccountId = whitelisted_caller();
	}: bw6_761_msm_g1(RawOrigin::Signed(caller), 10)

	bw6_761_msm_g1_10_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: bw6_761_msm_g1_optimized(RawOrigin::Signed(caller), 10)

	bw6_761_msm_g1_1000 {
		let caller: T::AccountId = whitelisted_caller();
	}: bw6_761_msm_g1(RawOrigin::Signed(caller), 1000)

	bw6_761_msm_g1_1000_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: bw6_761_msm_g1_optimized(RawOrigin::Signed(caller), 1000)

	bw6_761_msm_g2_10 {
		let caller: T::AccountId = whitelisted_caller();
	}: bw6_761_msm_g2(RawOrigin::Signed(caller), 10)

	bw6_761_msm_g2_10_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: bw6_761_msm_g2_optimized(RawOrigin::Signed(caller), 10)

	bw6_761_msm_g2_1000 {
		let caller: T::AccountId = whitelisted_caller();
	}: bw6_761_msm_g2(RawOrigin::Signed(caller), 1000)

	bw6_761_msm_g2_1000_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: bw6_761_msm_g2_optimized(RawOrigin::Signed(caller), 1000)

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

	ed_on_bls12_381_msm_sw_10 {
		let caller: T::AccountId = whitelisted_caller();
	}: ed_on_bls12_381_msm_sw(RawOrigin::Signed(caller), 10)

	ed_on_bls12_381_msm_sw_10_optimized{
		let caller: T::AccountId = whitelisted_caller();
	}: ed_on_bls12_381_msm_sw_optimized(RawOrigin::Signed(caller), 10)

	ed_on_bls12_381_msm_sw_1000 {
		let caller: T::AccountId = whitelisted_caller();
	}: ed_on_bls12_381_msm_sw(RawOrigin::Signed(caller), 1000)

	ed_on_bls12_381_msm_sw_1000_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: ed_on_bls12_381_msm_sw_optimized(RawOrigin::Signed(caller), 1000)

	ed_on_bls12_381_mul_projective_sw {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	ed_on_bls12_381_mul_projective_sw_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	ed_on_bls12_381_mul_affine_sw {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	ed_on_bls12_381_mul_affine_sw_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	ed_on_bls12_381_msm_te_10 {
		let caller: T::AccountId = whitelisted_caller();
	}: ed_on_bls12_381_msm_te(RawOrigin::Signed(caller), 10)

	ed_on_bls12_381_msm_te_10_optimized{
		let caller: T::AccountId = whitelisted_caller();
	}: ed_on_bls12_381_msm_te_optimized(RawOrigin::Signed(caller), 10)

	ed_on_bls12_381_msm_te_1000 {
		let caller: T::AccountId = whitelisted_caller();
	}: ed_on_bls12_381_msm_te(RawOrigin::Signed(caller), 1000)

	ed_on_bls12_381_msm_te_1000_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: ed_on_bls12_381_msm_te_optimized(RawOrigin::Signed(caller), 1000)

	ed_on_bls12_381_mul_projective_te {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	ed_on_bls12_381_mul_projective_te_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	ed_on_bls12_381_mul_affine_te {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	ed_on_bls12_381_mul_affine_te_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	ed_on_bls12_377_msm_10 {
		let caller: T::AccountId = whitelisted_caller();
	}: ed_on_bls12_377_msm(RawOrigin::Signed(caller), 10)

	ed_on_bls12_377_msm_10_optimized {
		let caller: T::AccountId = whitelisted_caller();
	}: ed_on_bls12_377_msm_optimized(RawOrigin::Signed(caller), 10)

	ed_on_bls12_377_msm_1000 {
		let caller: T::AccountId = whitelisted_caller();
	}: ed_on_bls12_377_msm(RawOrigin::Signed(caller), 1000)

	ed_on_bls12_377_msm_optimized_1000 {
		let caller: T::AccountId = whitelisted_caller();
	}: ed_on_bls12_377_msm_optimized(RawOrigin::Signed(caller), 1000)

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
