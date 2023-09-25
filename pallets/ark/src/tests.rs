use crate::{
	benchmarking::{C_SERIALIZED, PROOF_SERIALIZED, VK_SERIALIZED},
	mock::*,
	utils::{
		make_msm_args, make_pairing_args, make_scalar_args, make_scalar_args_projective,
		serialize_argument, ProofFor, VerifyingKeyFor,
	},
};
use ark_ff::{Fp, MontBackend};
use ark_serialize::CanonicalDeserialize;
use codec::Encode;
use frame_support::assert_ok;

const SCALAR_WORDS: u32 = 3;
const MSM_LEN: u32 = 10;

// ---------------------------------------------
// Tests for bls12-381
// ---------------------------------------------

#[test]
fn bls12_381_groth16_verification() {
	let vk =
			VerifyingKeyFor::<ark_bls12_381::Bls12_381, ark_bls12_381::Fr>::deserialize_compressed_unchecked(VK_SERIALIZED)
				.unwrap();
	let vk = serialize_argument(vk);

	let c = Fp::<MontBackend<ark_bls12_381::FrConfig, 4>, 4>::deserialize_compressed_unchecked(
		C_SERIALIZED,
	)
	.unwrap();
	let c = serialize_argument(c);

	let proof =
		ProofFor::<ark_bls12_381::Bls12_381, ark_bls12_381::Fr>::deserialize_compressed_unchecked(
			PROOF_SERIALIZED,
		)
		.unwrap();
	let proof = serialize_argument(proof);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bls12_381_groth16_verification(RuntimeOrigin::none(), vk, c, proof));
	});
}

#[test]
fn bls12_381_groth16_verificaton_opt() {
	let vk = VerifyingKeyFor::<sp_bls12_381::Bls12_381, sp_bls12_381::Fr>::deserialize_compressed_unchecked(
			VK_SERIALIZED,
		)
		.unwrap();
	let vk = serialize_argument(vk);

	let c = Fp::<MontBackend<sp_bls12_381::FrConfig, 4>, 4>::deserialize_compressed_unchecked(
		C_SERIALIZED,
	)
	.unwrap();
	let c = serialize_argument(c);

	let proof =
		ProofFor::<sp_bls12_381::Bls12_381, sp_bls12_381::Fr>::deserialize_compressed_unchecked(
			PROOF_SERIALIZED,
		)
		.unwrap();
	let proof = serialize_argument(proof);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bls12_381_groth16_verification_opt(RuntimeOrigin::none(), vk, c, proof));
	});
}

#[test]
fn bls12_381_pairing() {
	let (a, b) = make_pairing_args::<ark_bls12_381::G1Affine, ark_bls12_381::G2Affine>();

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bls12_381_pairing(RuntimeOrigin::none(), a, b));
	});
}

#[test]
fn bls12_381_pairing_opt() {
	let (a, b) = make_pairing_args::<sp_bls12_381::G1Affine, sp_bls12_381::G2Affine>();

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bls12_381_pairing_opt(RuntimeOrigin::none(), a, b));
	});
}

#[test]
fn bls12_381_msm_g1() {
	let (bases, scalars) = make_msm_args::<ark_bls12_381::G1Projective>(MSM_LEN);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bls12_381_msm_g1(RuntimeOrigin::none(), bases.encode(), scalars.encode()));
	});
}

#[test]
fn bls12_381_msm_g1_opt() {
	let (bases, scalars) = make_msm_args::<sp_bls12_381::G1Projective>(MSM_LEN);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bls12_381_msm_g1_opt(
			RuntimeOrigin::none(),
			bases.encode(),
			scalars.encode()
		));
	});
}

#[test]
fn bls12_381_msm_g2() {
	let (bases, scalars) = make_msm_args::<ark_bls12_381::G2Projective>(MSM_LEN);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bls12_381_msm_g2(RuntimeOrigin::none(), bases.encode(), scalars.encode()));
	});
}

#[test]
fn bls12_381_msm_g2_opt() {
	let (bases, scalars) = make_msm_args::<sp_bls12_381::G2Projective>(MSM_LEN);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bls12_381_msm_g2_opt(
			RuntimeOrigin::none(),
			bases.encode(),
			scalars.encode()
		));
	});
}

#[test]
fn bls12_381_mul_projective_g1() {
	let (base, scalar) = make_scalar_args_projective::<ark_bls12_381::G1Projective>(SCALAR_WORDS);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bls12_381_mul_projective_g1(
			RuntimeOrigin::none(),
			base.encode(),
			scalar.encode()
		));
	});
}

#[test]
fn bls12_381_mul_projective_g1_opt() {
	let (base, scalar) = make_scalar_args_projective::<sp_bls12_381::G1Projective>(SCALAR_WORDS);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bls12_381_mul_projective_g1(
			RuntimeOrigin::none(),
			base.encode(),
			scalar.encode()
		));
	});
}

#[test]
fn bls12_381_mul_affine_g1() {
	let (base, scalar) = make_scalar_args::<ark_bls12_381::G1Affine>(SCALAR_WORDS);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bls12_381_mul_affine_g1(
			RuntimeOrigin::none(),
			base.encode(),
			scalar.encode()
		));
	});
}

#[test]
fn bls12_381_mul_affine_g1_opt() {
	let (base, scalar) = make_scalar_args::<sp_bls12_381::G1Affine>(SCALAR_WORDS);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bls12_381_mul_affine_g1_opt(
			RuntimeOrigin::none(),
			base.encode(),
			scalar.encode()
		));
	});
}

#[test]
fn bls12_381_mul_projective_g2() {
	let (base, scalar) = make_scalar_args_projective::<ark_bls12_381::G2Projective>(SCALAR_WORDS);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bls12_381_mul_projective_g2(
			RuntimeOrigin::none(),
			base.encode(),
			scalar.encode()
		));
	});
}

#[test]
fn bls12_381_mul_projective_g2_opt() {
	let (base, scalar) = make_scalar_args_projective::<sp_bls12_381::G2Projective>(SCALAR_WORDS);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bls12_381_mul_projective_g2_opt(
			RuntimeOrigin::none(),
			base.encode(),
			scalar.encode()
		));
	});
}

#[test]
fn bls12_381_mul_affine_g2() {
	let (base, scalar) = make_scalar_args::<ark_bls12_381::G2Affine>(SCALAR_WORDS);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bls12_381_mul_affine_g2_opt(
			RuntimeOrigin::none(),
			base.encode(),
			scalar.encode()
		));
	});
}

#[test]
fn bls12_381_mul_affine_g2_opt() {
	let (base, scalar) = make_scalar_args::<sp_bls12_381::G2Affine>(SCALAR_WORDS);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bls12_381_mul_affine_g2(
			RuntimeOrigin::none(),
			base.encode(),
			scalar.encode()
		));
	});
}

// ---------------------------------------------
// Tests for bls12-377
// ---------------------------------------------

#[test]
fn bls12_377_pairing() {
	let (a, b) = make_pairing_args::<ark_bls12_377::G1Affine, ark_bls12_377::G2Affine>();

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bls12_377_pairing(RuntimeOrigin::none(), a, b));
	});
}

#[test]
fn bls12_377_pairing_opt() {
	let (a, b) = make_pairing_args::<sp_bls12_377::G1Affine, sp_bls12_377::G2Affine>();

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bls12_377_pairing_opt(RuntimeOrigin::none(), a, b));
	});
}

#[test]
fn bls12_377_msm_g1() {
	let (bases, scalars) = make_msm_args::<ark_bls12_377::G1Projective>(MSM_LEN);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bls12_377_msm_g1(RuntimeOrigin::none(), bases.encode(), scalars.encode()));
	});
}

#[test]
fn bls12_377_msm_g1_opt() {
	let (bases, scalars) = make_msm_args::<sp_bls12_377::g1::G1Projective>(MSM_LEN);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bls12_377_msm_g1_opt(
			RuntimeOrigin::none(),
			bases.encode(),
			scalars.encode()
		));
	});
}

#[test]
fn bls12_377_msm_g2() {
	let (bases, scalars) = make_msm_args::<ark_bls12_377::G2Projective>(MSM_LEN);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bls12_377_msm_g2(RuntimeOrigin::none(), bases.encode(), scalars.encode()));
	});
}

#[test]
fn bls12_377_msm_g2_opt() {
	let (bases, scalars) = make_msm_args::<sp_bls12_377::G2Projective>(MSM_LEN);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bls12_377_msm_g2_opt(
			RuntimeOrigin::none(),
			bases.encode(),
			scalars.encode()
		));
	});
}

#[test]
fn bls12_377_mul_projective_g1() {
	let (base, scalar) = make_scalar_args_projective::<ark_bls12_377::G1Projective>(SCALAR_WORDS);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bls12_377_mul_projective_g1(
			RuntimeOrigin::none(),
			base.encode(),
			scalar.encode()
		));
	});
}

#[test]
fn bls12_377_mul_projective_g1_opt() {
	new_test_ext().execute_with(|| {
		let (base, scalar) =
			make_scalar_args_projective::<sp_bls12_377::G1Projective>(SCALAR_WORDS);

		assert_ok!(Ark::bls12_377_mul_projective_g1_opt(
			RuntimeOrigin::none(),
			base.encode(),
			scalar.encode()
		));
	});
}

#[test]
fn bls12_377_mul_affine_g1() {
	let (base, scalar) = make_scalar_args::<ark_bls12_377::G1Affine>(SCALAR_WORDS);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bls12_377_mul_affine_g1(
			RuntimeOrigin::none(),
			base.encode(),
			scalar.encode()
		));
	});
}

#[test]
fn bls12_377_mul_affine_g1_opt() {
	let (base, scalar) = make_scalar_args::<sp_bls12_377::G1Affine>(SCALAR_WORDS);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bls12_377_mul_affine_g1_opt(
			RuntimeOrigin::none(),
			base.encode(),
			scalar.encode()
		));
	});
}

#[test]
fn bls12_377_mul_projective_g2() {
	let (base, scalar) = make_scalar_args_projective::<ark_bls12_377::G2Projective>(SCALAR_WORDS);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bls12_377_mul_projective_g2(
			RuntimeOrigin::none(),
			base.encode(),
			scalar.encode()
		));
	});
}

#[test]
fn bls12_377_mul_projective_g2_opt() {
	let (base, scalar) = make_scalar_args_projective::<sp_bls12_377::G2Projective>(SCALAR_WORDS);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bls12_377_mul_projective_g2_opt(
			RuntimeOrigin::none(),
			base.encode(),
			scalar.encode()
		));
	});
}

#[test]
fn bls12_377_mul_affine_g2() {
	let (base, scalar) = make_scalar_args::<ark_bls12_377::G2Affine>(SCALAR_WORDS);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bls12_377_mul_affine_g2(
			RuntimeOrigin::none(),
			base.encode(),
			scalar.encode()
		));
	});
}

#[test]
fn bls12_377_mul_affine_g2_opt() {
	let (base, scalar) = make_scalar_args::<sp_bls12_377::G2Affine>(SCALAR_WORDS);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bls12_377_mul_affine_g2_opt(
			RuntimeOrigin::none(),
			base.encode(),
			scalar.encode()
		));
	});
}

// ---------------------------------------------
// Tests for bw6-761
// ---------------------------------------------

#[test]
fn bw6_761_pairing() {
	let (a, b) = make_pairing_args::<ark_bw6_761::G1Affine, ark_bw6_761::G2Affine>();

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bw6_761_pairing(RuntimeOrigin::none(), a, b));
	});
}

#[test]
fn bw6_761_pairing_opt() {
	let (a, b) = make_pairing_args::<sp_bw6_761::g1::G1Affine, sp_bw6_761::g2::G2Affine>();

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bw6_761_pairing_opt(RuntimeOrigin::none(), a, b));
	});
}

#[test]
fn bw6_761_msm_g1() {
	let (bases, scalars) = make_msm_args::<ark_bw6_761::G1Projective>(MSM_LEN);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bw6_761_msm_g1(RuntimeOrigin::none(), bases.encode(), scalars.encode()));
	});
}

#[test]
fn bw6_761_msm_g1_opt() {
	let (bases, scalars) = make_msm_args::<sp_bw6_761::g1::G1Projective>(MSM_LEN);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bw6_761_msm_g1_opt(
			RuntimeOrigin::none(),
			bases.encode(),
			scalars.encode()
		));
	});
}

#[test]
fn bw6_761_msm_g2() {
	new_test_ext().execute_with(|| {
		let (bases, scalars) = make_msm_args::<ark_bw6_761::G2Projective>(MSM_LEN);

		assert_ok!(Ark::bw6_761_msm_g2(RuntimeOrigin::none(), bases.encode(), scalars.encode()));
	});
}

#[test]
fn bw6_761_msm_g2_opt() {
	let (bases, scalars) = make_msm_args::<sp_bw6_761::g2::G2Projective>(MSM_LEN);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bw6_761_msm_g2_opt(
			RuntimeOrigin::none(),
			bases.encode(),
			scalars.encode()
		));
	});
}

#[test]
fn bw6_761_mul_projective_g1() {
	let (base, scalar) = make_scalar_args_projective::<ark_bw6_761::G1Projective>(SCALAR_WORDS);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bw6_761_mul_projective_g1(
			RuntimeOrigin::none(),
			base.encode(),
			scalar.encode()
		));
	});
}

#[test]
fn bw6_761_mul_projective_g1_opt() {
	let (base, scalar) = make_scalar_args_projective::<sp_bw6_761::g1::G1Projective>(SCALAR_WORDS);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bw6_761_mul_projective_g1_opt(
			RuntimeOrigin::none(),
			base.encode(),
			scalar.encode()
		));
	});
}

#[test]
fn bw6_761_mul_affine_g1() {
	let (base, scalar) = make_scalar_args::<ark_bw6_761::G1Affine>(SCALAR_WORDS);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bw6_761_mul_affine_g1(
			RuntimeOrigin::none(),
			base.encode(),
			scalar.encode()
		));
	});
}

#[test]
fn bw6_761_mul_affine_g1_opt() {
	let (base, scalar) = make_scalar_args::<sp_bw6_761::g1::G1Affine>(SCALAR_WORDS);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bw6_761_mul_affine_g1_opt(
			RuntimeOrigin::none(),
			base.encode(),
			scalar.encode()
		));
	});
}

#[test]
fn bw6_761_mul_projective_g2() {
	let (base, scalar) = make_scalar_args_projective::<ark_bw6_761::G2Projective>(SCALAR_WORDS);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bw6_761_mul_projective_g2(
			RuntimeOrigin::none(),
			base.encode(),
			scalar.encode()
		));
	});
}

#[test]
fn bw6_761_mul_projective_g2_opt() {
	let (base, scalar) = make_scalar_args_projective::<sp_bw6_761::g2::G2Projective>(SCALAR_WORDS);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bw6_761_mul_projective_g2_opt(
			RuntimeOrigin::none(),
			base.encode(),
			scalar.encode()
		));
	});
}

#[test]
fn bw6_761_mul_affine_g2() {
	let (base, scalar) = make_scalar_args::<ark_bw6_761::G2Affine>(SCALAR_WORDS);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bw6_761_mul_affine_g2(
			RuntimeOrigin::none(),
			base.encode(),
			scalar.encode()
		));
	});
}

#[test]
fn bw6_761_mul_affine_g2_opt() {
	let (base, scalar) = make_scalar_args::<sp_bw6_761::g2::G2Affine>(SCALAR_WORDS);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::bw6_761_mul_affine_g2_opt(
			RuntimeOrigin::none(),
			base.encode(),
			scalar.encode()
		));
	});
}

// ---------------------------------------------
// Tests for ed-on-bls12-381-bandersnatch
// ---------------------------------------------

#[test]
fn ed_on_bls12_381_bandersnatch_msm_sw() {
	let (bases, scalars) = make_msm_args::<ark_ed_on_bls12_381_bandersnatch::SWProjective>(MSM_LEN);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::ed_on_bls12_381_bandersnatch_msm_sw(
			RuntimeOrigin::none(),
			bases.encode(),
			scalars.encode()
		));
	});
}

#[test]
fn ed_on_bls12_381_bandersnatch_msm_sw_opt() {
	let (bases, scalars) = make_msm_args::<sp_ed_on_bls12_381_bandersnatch::SWProjective>(MSM_LEN);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::ed_on_bls12_381_bandersnatch_msm_sw_opt(
			RuntimeOrigin::none(),
			bases.encode(),
			scalars.encode()
		));
	});
}

#[test]
fn ed_on_bls12_381_bandersnatch_msm_te() {
	let (bases, scalars) =
		make_msm_args::<ark_ed_on_bls12_381_bandersnatch::EdwardsProjective>(MSM_LEN);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::ed_on_bls12_381_bandersnatch_msm_te(
			RuntimeOrigin::none(),
			bases.encode(),
			scalars.encode()
		));
	});
}

#[test]
fn ed_on_bls12_381_bandersnatch_msm_te_opt() {
	let (bases, scalars) =
		make_msm_args::<sp_ed_on_bls12_381_bandersnatch::EdwardsProjective>(MSM_LEN);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::ed_on_bls12_381_bandersnatch_msm_te_opt(
			RuntimeOrigin::none(),
			bases.encode(),
			scalars.encode()
		));
	});
}

#[test]
fn ed_on_bls12_381_bandersnatch_mul_projective_sw() {
	let (base, scalar) =
		make_scalar_args_projective::<ark_ed_on_bls12_381_bandersnatch::SWProjective>(SCALAR_WORDS);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::ed_on_bls12_381_bandersnatch_mul_projective_sw(
			RuntimeOrigin::none(),
			base.encode(),
			scalar.encode()
		));
	});
}

#[test]
fn ed_on_bls12_381_bandersnatch_mul_projective_sw_opt() {
	new_test_ext().execute_with(|| {
		let (base, scalar) = make_scalar_args_projective::<
			sp_ed_on_bls12_381_bandersnatch::SWProjective,
		>(SCALAR_WORDS);

		assert_ok!(Ark::ed_on_bls12_381_bandersnatch_mul_projective_sw_opt(
			RuntimeOrigin::none(),
			base.encode(),
			scalar.encode()
		));
	});
}

#[test]
fn ed_on_bls12_381_bandersnatch_mul_affine_sw() {
	let (base, scalar) =
		make_scalar_args::<ark_ed_on_bls12_381_bandersnatch::SWAffine>(SCALAR_WORDS);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::ed_on_bls12_381_bandersnatch_mul_affine_sw(
			RuntimeOrigin::none(),
			base.encode(),
			scalar.encode()
		));
	});
}

#[test]
fn ed_on_bls12_381_bandersnatch_mul_affine_sw_opt() {
	let (base, scalar) =
		make_scalar_args::<sp_ed_on_bls12_381_bandersnatch::SWAffine>(SCALAR_WORDS);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::ed_on_bls12_381_bandersnatch_mul_affine_sw_opt(
			RuntimeOrigin::none(),
			base.encode(),
			scalar.encode()
		));
	});
}

#[test]
fn ed_on_bls12_381_bandersnatch_mul_projective_te() {
	let (base, scalar) = make_scalar_args_projective::<
		ark_ed_on_bls12_381_bandersnatch::EdwardsProjective,
	>(SCALAR_WORDS);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::ed_on_bls12_381_bandersnatch_mul_projective_te(
			RuntimeOrigin::none(),
			base.encode(),
			scalar.encode()
		));
	});
}

#[test]
fn ed_on_bls12_381_bandersnatch_mul_projective_te_opt() {
	let (base, scalar) = make_scalar_args_projective::<
		ark_ed_on_bls12_381_bandersnatch::EdwardsProjective,
	>(SCALAR_WORDS);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::ed_on_bls12_381_bandersnatch_mul_projective_te_opt(
			RuntimeOrigin::none(),
			base.encode(),
			scalar.encode()
		));
	});
}

#[test]
fn ed_on_bls12_381_bandersnatch_mul_affine_te() {
	let (base, scalar) =
		make_scalar_args::<ark_ed_on_bls12_381_bandersnatch::EdwardsAffine>(SCALAR_WORDS);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::ed_on_bls12_381_bandersnatch_mul_affine_te(
			RuntimeOrigin::none(),
			base.encode(),
			scalar.encode()
		));
	});
}

#[test]
fn ed_on_bls12_381_bandersnatch_mul_affine_te_opt() {
	let (base, scalar) =
		make_scalar_args::<sp_ed_on_bls12_381_bandersnatch::EdwardsAffine>(SCALAR_WORDS);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::ed_on_bls12_381_bandersnatch_mul_affine_te_opt(
			RuntimeOrigin::none(),
			base.encode(),
			scalar.encode()
		));
	});
}

// ---------------------------------------------
// Tests for ed-on-bls12-377
// ---------------------------------------------

#[test]
fn ed_on_bls12_377_msm() {
	let (bases, scalars) = make_msm_args::<ark_ed_on_bls12_377::EdwardsProjective>(SCALAR_WORDS);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::ed_on_bls12_377_msm(
			RuntimeOrigin::none(),
			bases.encode(),
			scalars.encode()
		));
	});
}

#[test]
fn ed_on_bls12_377_msm_opt() {
	let (bases, scalars) = make_msm_args::<sp_ed_on_bls12_377::EdwardsProjective>(SCALAR_WORDS);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::ed_on_bls12_377_msm_opt(
			RuntimeOrigin::none(),
			bases.encode(),
			scalars.encode()
		));
	});
}

#[test]
fn ed_on_bls12_377_mul_projective() {
	let (base, scalar) =
		make_scalar_args_projective::<ark_ed_on_bls12_377::EdwardsProjective>(SCALAR_WORDS);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::ed_on_bls12_377_mul_projective(
			RuntimeOrigin::none(),
			base.encode(),
			scalar.encode()
		));
	});
}

#[test]
fn ed_on_bls12_377_mul_projective_opt() {
	let (base, scalar) =
		make_scalar_args_projective::<sp_ed_on_bls12_377::EdwardsProjective>(SCALAR_WORDS);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::ed_on_bls12_377_mul_projective_opt(
			RuntimeOrigin::none(),
			base.encode(),
			scalar.encode()
		));
	});
}

#[test]
fn ed_on_bls12_377_mul_affine() {
	let (base, scalar) = make_scalar_args::<
		ark_ec::twisted_edwards::Affine<ark_ed_on_bls12_377::EdwardsConfig>,
	>(SCALAR_WORDS);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::ed_on_bls12_377_mul_affine(
			RuntimeOrigin::none(),
			base.encode(),
			scalar.encode()
		));
	});
}

#[test]
fn ed_on_bls12_377_mul_affine_opt() {
	let (base, scalar) = make_scalar_args::<sp_ed_on_bls12_377::EdwardsAffine>(SCALAR_WORDS);

	new_test_ext().execute_with(|| {
		assert_ok!(Ark::ed_on_bls12_377_mul_affine_opt(
			RuntimeOrigin::none(),
			base.encode(),
			scalar.encode()
		));
	});
}
