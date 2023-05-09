use crate::{
	bls12_377, bls12_381, bw6_761, ed_on_bls12_377, ed_on_bls12_381_bandersnatch,
	mock::*,
	utils::{
		generate_msm_args, generate_pairing_args, generate_scalar_args,
		generate_scalar_args_projective,
	},
};
use ark_serialize::{CanonicalDeserialize, Compress, Validate};
use codec::Encode;
use frame_support::assert_ok;

#[test]
fn groth16_verification() {
	let vk = <ark_groth16::Groth16<ark_bls12_381::Bls12_381> as ark_snark::SNARK<
		ark_bls12_381::Fr,
	>>::VerifyingKey::deserialize_with_mode(
		crate::benchmarking::VK_SERIALIZED,
		Compress::Yes,
		Validate::No,
	)
	.unwrap();
	let vk = crate::utils::serialize_argument(vk);

	let c =
		ark_ff::Fp::<ark_ff::MontBackend<ark_bls12_381::FrConfig, 4>, 4>::deserialize_with_mode(
			crate::benchmarking::C_SERIALIZED,
			Compress::Yes,
			Validate::No,
		)
		.unwrap();
	let c = crate::utils::serialize_argument(c);

	let proof = <ark_groth16::Groth16<ark_bls12_381::Bls12_381> as ark_snark::SNARK<
		ark_bls12_381::Fr,
	>>::Proof::deserialize_with_mode(
		crate::benchmarking::PROOF_SERIALIZED,
		Compress::Yes,
		Validate::No,
	)
	.unwrap();
	let proof = crate::utils::serialize_argument(proof);
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::groth16_verification(RuntimeOrigin::signed(1), vk, c, proof));
	});
}

#[test]
fn groth16_verificaton_optimized() {
	new_test_ext().execute_with(|| {
		let vk = <ark_groth16::Groth16<bls12_381::Bls12_381Optimized> as ark_snark::SNARK<
			crate::bls12_381::BlsFrOptimized,
		>>::VerifyingKey::deserialize_with_mode(
			crate::benchmarking::VK_SERIALIZED,
			Compress::Yes,
			Validate::No,
		)
		.unwrap();
		let vk = crate::utils::serialize_argument(vk);

		let c =
			ark_ff::Fp::<ark_ff::MontBackend<sp_bls12_381::FrConfig, 4>, 4>::deserialize_with_mode(
				crate::benchmarking::C_SERIALIZED,
				Compress::Yes,
				Validate::No,
			)
			.unwrap();
		let c = crate::utils::serialize_argument(c);

		let proof =
			<ark_groth16::Groth16<crate::bls12_381::Bls12_381Optimized> as ark_snark::SNARK<
				sp_bls12_381::Fr,
			>>::Proof::deserialize_with_mode(
				crate::benchmarking::PROOF_SERIALIZED,
				Compress::Yes,
				Validate::No,
			)
			.unwrap();
		let proof = crate::utils::serialize_argument(proof);
		assert_ok!(TemplateModule::groth16_verification_optimized(
			RuntimeOrigin::signed(1),
			vk,
			c,
			proof
		));
	});
}

#[test]
fn pairing_bls12_381() {
	new_test_ext().execute_with(|| {
		let (a, b) = generate_pairing_args::<
			<ark_ec::bls12::Bls12<ark_bls12_381::Config> as ark_ec::pairing::Pairing>::G1Affine,
			<ark_ec::bls12::Bls12<ark_bls12_381::Config> as ark_ec::pairing::Pairing>::G2Affine,
		>();
		assert_ok!(TemplateModule::bls12_381_pairing(
			RuntimeOrigin::signed(1),
			a.encode(),
			b.encode()
		));
	});
}
#[test]
fn pairing_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		let (a, b) =
			generate_pairing_args::<bls12_381::G1AffineOptimized, bls12_381::G2AffineOptimized>();
		assert_ok!(TemplateModule::bls12_381_pairing_optimized(
			RuntimeOrigin::signed(1),
			a.encode(),
			b.encode()
		));
	});
}
#[test]
fn msm_g1_bls12_381() {
	new_test_ext().execute_with(|| {
		let (bases, scalars) = generate_msm_args::<
			ark_ec::short_weierstrass::Projective<ark_bls12_381::g1::Config>,
		>(10);
		assert_ok!(TemplateModule::bls12_381_msm_g1(
			RuntimeOrigin::signed(1),
			bases.encode(),
			scalars.encode()
		));
	});
}
#[test]
fn msm_g1_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		let (bases, scalars) = generate_msm_args::<
			sp_ark_models::short_weierstrass::Projective<sp_bls12_381::g1::Config>,
		>(10);
		assert_ok!(TemplateModule::bls12_381_msm_g1_optimized(
			RuntimeOrigin::signed(1),
			bases.encode(),
			scalars.encode()
		));
	});
}
#[test]
fn msm_g2_bls12_381() {
	new_test_ext().execute_with(|| {
		let (bases, scalars) = generate_msm_args::<
			ark_ec::short_weierstrass::Projective<ark_bls12_381::g2::Config>,
		>(10);
		assert_ok!(TemplateModule::bls12_381_msm_g2(
			RuntimeOrigin::signed(1),
			bases.encode(),
			scalars.encode()
		));
	});
}
#[test]
fn msm_g2_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		let (bases, scalars) = generate_msm_args::<
			sp_ark_models::short_weierstrass::Projective<sp_bls12_381::g2::Config>,
		>(10);
		assert_ok!(TemplateModule::bls12_381_msm_g2_optimized(
			RuntimeOrigin::signed(1),
			bases.encode(),
			scalars.encode()
		));
	});
}
#[test]
fn mul_projective_g1_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		let (base, scalar) = generate_scalar_args_projective::<bls12_381::G1ProjectiveOptimized>();
		assert_ok!(TemplateModule::bls12_381_mul_projective_g1(
			RuntimeOrigin::signed(1),
			base.encode(),
			scalar.encode()
		));
	});
}
#[test]
fn mul_projective_g1_bls12_381() {
	new_test_ext().execute_with(|| {
		let (base, scalar) = generate_scalar_args_projective::<ark_bls12_381::G1Projective>();
		assert_ok!(TemplateModule::bls12_381_mul_projective_g1_optimized(
			RuntimeOrigin::signed(1),
			base.encode(),
			scalar.encode()
		));
	});
}
#[test]
fn mul_affine_g1_bls12_381() {
	new_test_ext().execute_with(|| {
		let (base, scalar) = generate_scalar_args::<ark_bls12_381::G1Affine>();
		assert_ok!(TemplateModule::bls12_381_mul_affine_g1(
			RuntimeOrigin::signed(1),
			base.encode(),
			scalar.encode()
		));
	});
}
#[test]
fn mul_affine_g1_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		let (base, scalar) = generate_scalar_args::<bls12_381::G1AffineOptimized>();
		assert_ok!(TemplateModule::bls12_381_mul_affine_g1_optimized(
			RuntimeOrigin::signed(1),
			base.encode(),
			scalar.encode()
		));
	});
}
#[test]
fn mul_projective_g2_bls12_381() {
	new_test_ext().execute_with(|| {
		let (base, scalar) = generate_scalar_args_projective::<ark_bls12_381::G2Projective>();
		assert_ok!(TemplateModule::bls12_381_mul_projective_g2(
			RuntimeOrigin::signed(1),
			base.encode(),
			scalar.encode()
		));
	});
}
#[test]
fn mul_projective_g2_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		let (base, scalar) = generate_scalar_args_projective::<bls12_381::G2ProjectiveOptimized>();
		assert_ok!(TemplateModule::bls12_381_mul_projective_g2_optimized(
			RuntimeOrigin::signed(1),
			base.encode(),
			scalar.encode()
		));
	});
}
#[test]
fn mul_affine_g2_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		let (base, scalar) = generate_scalar_args::<bls12_381::G2AffineOptimized>();
		assert_ok!(TemplateModule::bls12_381_mul_affine_g2(
			RuntimeOrigin::signed(1),
			base.encode(),
			scalar.encode()
		));
	});
}
#[test]
fn mul_affine_g2_bls12_381() {
	new_test_ext().execute_with(|| {
		let (base, scalar) = generate_scalar_args::<ark_bls12_381::G2Affine>();
		assert_ok!(TemplateModule::bls12_381_mul_affine_g2_optimized(
			RuntimeOrigin::signed(1),
			base.encode(),
			scalar.encode()
		));
	});
}
#[test]
fn pairing_bls12_377() {
	new_test_ext().execute_with(|| {
		let (a, b) = generate_pairing_args::<ark_bls12_377::G1Affine, ark_bls12_377::G2Affine>();
		assert_ok!(TemplateModule::bls12_377_pairing(
			RuntimeOrigin::signed(1),
			a.encode(),
			b.encode()
		));
	});
}
#[test]
fn pairing_bls12_377_optimized() {
	new_test_ext().execute_with(|| {
		let (a, b) =
			generate_pairing_args::<bls12_377::G1AffineOptimized, bls12_377::G2AffineOptimized>();
		assert_ok!(TemplateModule::bls12_377_pairing_optimized(
			RuntimeOrigin::signed(1),
			a.encode(),
			b.encode()
		));
	});
}
#[test]
fn msm_g1_bls12_377() {
	let (bases, scalars) =
		generate_msm_args::<ark_ec::short_weierstrass::Projective<ark_bls12_377::g1::Config>>(10);
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::bls12_377_msm_g1(
			RuntimeOrigin::signed(1),
			bases.encode(),
			scalars.encode()
		));
	});
}
#[test]
fn msm_g1_bls12_377_optimized() {
	let (bases, scalars) = generate_msm_args::<sp_bls12_377::curves::g1::G1Projective>(10);
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::bls12_377_msm_g1_optimized(
			RuntimeOrigin::signed(1),
			bases.encode(),
			scalars.encode()
		));
	});
}
#[test]
fn msm_g2_bls12_377() {
	let (bases, scalars) =
		generate_msm_args::<ark_ec::short_weierstrass::Projective<ark_bls12_377::g2::Config>>(10);
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::bls12_377_msm_g2(
			RuntimeOrigin::signed(1),
			bases.encode(),
			scalars.encode()
		));
	});
}
#[test]
fn msm_g2_bls12_377_optimized() {
	let (bases, scalars) = generate_msm_args::<sp_bls12_377::g2::G2Projective>(10);
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::bls12_377_msm_g2_optimized(
			RuntimeOrigin::signed(1),
			bases.encode(),
			scalars.encode()
		));
	});
}
#[test]
fn mul_projective_g1_bls12_377() {
	new_test_ext().execute_with(|| {
		let (base, scalar) = generate_scalar_args_projective::<ark_bls12_377::G1Projective>();
		assert_ok!(TemplateModule::bls12_377_mul_projective_g1(
			RuntimeOrigin::signed(1),
			base.encode(),
			scalar.encode()
		));
	});
}
#[test]
fn mul_projective_g1_bls12_377_optimized() {
	new_test_ext().execute_with(|| {
		let (base, scalar) = generate_scalar_args_projective::<bls12_377::G1ProjectiveOptimized>();
		assert_ok!(TemplateModule::bls12_377_mul_projective_g1_optimized(
			RuntimeOrigin::signed(1),
			base.encode(),
			scalar.encode()
		));
	});
}
#[test]
fn mul_affine_g1_bls12_377() {
	new_test_ext().execute_with(|| {
		let (base, scalar) = generate_scalar_args::<ark_bls12_377::G1Affine>();
		assert_ok!(TemplateModule::bls12_377_mul_affine_g1(
			RuntimeOrigin::signed(1),
			base.encode(),
			scalar.encode()
		));
	});
}
#[test]
fn mul_affine_g1_bls12_377_optimized() {
	new_test_ext().execute_with(|| {
		let (base, scalar) = generate_scalar_args::<bls12_377::G1AffineOptimized>();
		assert_ok!(TemplateModule::bls12_377_mul_affine_g1_optimized(
			RuntimeOrigin::signed(1),
			base.encode(),
			scalar.encode()
		));
	});
}
#[test]
fn mul_projective_g2_bls12_377() {
	new_test_ext().execute_with(|| {
		let (base, scalar) = generate_scalar_args_projective::<ark_bls12_377::G2Projective>();
		assert_ok!(TemplateModule::bls12_377_mul_projective_g2(
			RuntimeOrigin::signed(1),
			base.encode(),
			scalar.encode()
		));
	});
}
#[test]
fn mul_projective_g2_bls12_377_optimized() {
	new_test_ext().execute_with(|| {
		let (base, scalar) = generate_scalar_args_projective::<bls12_377::G2ProjectiveOptimized>();
		assert_ok!(TemplateModule::bls12_377_mul_projective_g2_optimized(
			RuntimeOrigin::signed(1),
			base.encode(),
			scalar.encode()
		));
	});
}
#[test]
fn mul_affine_g2_bls12_377() {
	new_test_ext().execute_with(|| {
		let (base, scalar) = generate_scalar_args::<ark_bls12_377::G2Affine>();
		assert_ok!(TemplateModule::bls12_377_mul_affine_g2(
			RuntimeOrigin::signed(1),
			base.encode(),
			scalar.encode()
		));
	});
}
#[test]
fn mul_affine_g2_bls12_377_optimized() {
	new_test_ext().execute_with(|| {
		let (base, scalar) = generate_scalar_args::<bls12_377::G2AffineOptimized>();
		assert_ok!(TemplateModule::bls12_377_mul_affine_g2_optimized(
			RuntimeOrigin::signed(1),
			base.encode(),
			scalar.encode()
		));
	});
}
#[test]
fn pairing_bw6_761() {
	new_test_ext().execute_with(|| {
		let (a, b) = generate_pairing_args::<ark_bw6_761::G1Affine, ark_bw6_761::G2Affine>();
		assert_ok!(TemplateModule::bw6_761_pairing(
			RuntimeOrigin::signed(1),
			a.encode(),
			b.encode()
		));
	});
}
#[test]
fn pairing_bw6_761_optimized() {
	new_test_ext().execute_with(|| {
		let (a, b) =
			generate_pairing_args::<bw6_761::G1AffineOptimized, bw6_761::G2AffineOptimized>();
		assert_ok!(TemplateModule::bw6_761_pairing_optimized(
			RuntimeOrigin::signed(1),
			a.encode(),
			b.encode()
		));
	});
}
#[test]
fn msm_g1_bw6_761() {
	new_test_ext().execute_with(|| {
		let (bases, scalars) =
			generate_msm_args::<ark_ec::short_weierstrass::Projective<ark_bw6_761::g1::Config>>(10);
		assert_ok!(TemplateModule::bw6_761_msm_g1(
			RuntimeOrigin::signed(1),
			bases.encode(),
			scalars.encode()
		));
	});
}
#[test]
fn msm_g1_bw6_761_optimized() {
	new_test_ext().execute_with(|| {
		let (bases, scalars) =
			generate_msm_args::<ark_ec::short_weierstrass::Projective<sp_bw6_761::g1::Config>>(10);
		assert_ok!(TemplateModule::bw6_761_msm_g1_optimized(
			RuntimeOrigin::signed(1),
			bases.encode(),
			scalars.encode()
		));
	});
}
#[test]
fn msm_g2_bw6_761() {
	new_test_ext().execute_with(|| {
		let (bases, scalars) =
			generate_msm_args::<ark_ec::short_weierstrass::Projective<ark_bw6_761::g2::Config>>(10);
		assert_ok!(TemplateModule::bw6_761_msm_g2(
			RuntimeOrigin::signed(1),
			bases.encode(),
			scalars.encode()
		));
	});
}
#[test]
fn msm_g2_bw6_761_optimized() {
	new_test_ext().execute_with(|| {
		let (bases, scalars) =
			generate_msm_args::<ark_ec::short_weierstrass::Projective<sp_bw6_761::g2::Config>>(10);
		assert_ok!(TemplateModule::bw6_761_msm_g2_optimized(
			RuntimeOrigin::signed(1),
			bases.encode(),
			scalars.encode()
		));
	});
}
#[test]
fn mul_projective_sw_ed_on_bls12_381_bandersnatch() {
	new_test_ext().execute_with(|| {
		let (base, scalar) =
			generate_scalar_args_projective::<ark_ed_on_bls12_381_bandersnatch::SWProjective>();
		assert_ok!(TemplateModule::ed_on_bls12_381_bandersnatch_mul_projective_sw(
			RuntimeOrigin::signed(1),
			base.encode(),
			scalar.encode()
		));
	});
}
#[test]
fn mul_projective_g1_bw6_761() {
	new_test_ext().execute_with(|| {
		let (base, scalar) = generate_scalar_args_projective::<ark_bw6_761::G1Projective>();
		assert_ok!(TemplateModule::bw6_761_mul_projective_g1(
			RuntimeOrigin::signed(1),
			base.encode(),
			scalar.encode()
		));
	});
}
#[test]
fn mul_projective_g1_bw6_761_optimized() {
	new_test_ext().execute_with(|| {
		let (base, scalar) = generate_scalar_args_projective::<bw6_761::G1ProjectiveOptimized>();
		assert_ok!(TemplateModule::bw6_761_mul_projective_g1_optimized(
			RuntimeOrigin::signed(1),
			base.encode(),
			scalar.encode()
		));
	});
}
#[test]
fn mul_affine_g1_bw6_761() {
	new_test_ext().execute_with(|| {
		let (base, scalar) = generate_scalar_args::<ark_bw6_761::G1Affine>();
		assert_ok!(TemplateModule::bw6_761_mul_affine_g1(
			RuntimeOrigin::signed(1),
			base.encode(),
			scalar.encode()
		));
	});
}
#[test]
fn mul_affine_g1_bw6_761_optimized() {
	new_test_ext().execute_with(|| {
		let (base, scalar) = generate_scalar_args::<bw6_761::G1AffineOptimized>();
		assert_ok!(TemplateModule::bw6_761_mul_affine_g1_optimized(
			RuntimeOrigin::signed(1),
			base.encode(),
			scalar.encode()
		));
	});
}
#[test]
fn mul_projective_g2_bw6_761() {
	new_test_ext().execute_with(|| {
		let (base, scalar) = generate_scalar_args_projective::<ark_bw6_761::G2Projective>();
		assert_ok!(TemplateModule::bw6_761_mul_projective_g2(
			RuntimeOrigin::signed(1),
			base.encode(),
			scalar.encode()
		));
	});
}
#[test]
fn mul_projective_g2_bw6_761_optimized() {
	new_test_ext().execute_with(|| {
		let (base, scalar) = generate_scalar_args_projective::<bw6_761::G2ProjectiveOptimized>();
		assert_ok!(TemplateModule::bw6_761_mul_projective_g2_optimized(
			RuntimeOrigin::signed(1),
			base.encode(),
			scalar.encode()
		));
	});
}
#[test]
fn mul_affine_g2_bw6_761() {
	new_test_ext().execute_with(|| {
		let (base, scalar) = generate_scalar_args::<ark_bw6_761::G2Affine>();
		assert_ok!(TemplateModule::bw6_761_mul_affine_g2(
			RuntimeOrigin::signed(1),
			base.encode(),
			scalar.encode()
		));
	});
}
#[test]
fn mul_affine_g2_bw6_761_optimized() {
	new_test_ext().execute_with(|| {
		let (base, scalar) = generate_scalar_args::<bw6_761::G2AffineOptimized>();
		assert_ok!(TemplateModule::bw6_761_mul_affine_g2_optimized(
			RuntimeOrigin::signed(1),
			base.encode(),
			scalar.encode()
		));
	});
}
#[test]
fn msm_ed_on_bls12_377() {
	new_test_ext().execute_with(|| {
		let (bases, scalars) =
			generate_msm_args::<sp_ed_on_bls12_377::curves::EdwardsProjective>(10);
		assert_ok!(TemplateModule::ed_on_bls12_377_msm(
			RuntimeOrigin::signed(1),
			bases.encode(),
			scalars.encode()
		));
	});
}
#[test]
fn msm_ed_on_bls12_377_optimized() {
	new_test_ext().execute_with(|| {
		let (bases, scalars) =
			generate_msm_args::<sp_ed_on_bls12_377::curves::EdwardsProjective>(10);
		assert_ok!(TemplateModule::ed_on_bls12_377_msm_optimized(
			RuntimeOrigin::signed(1),
			bases.encode(),
			scalars.encode()
		));
	});
}
#[test]
fn mul_projective_ed_on_bls12_377() {
	new_test_ext().execute_with(|| {
		let (base, scalar) =
			generate_scalar_args_projective::<ark_ed_on_bls12_377::EdwardsProjective>();
		assert_ok!(TemplateModule::ed_on_bls12_377_mul_projective(
			RuntimeOrigin::signed(1),
			base.encode(),
			scalar.encode()
		));
	});
}
#[test]
fn mul_projective_ed_on_bls12_377_optimized() {
	new_test_ext().execute_with(|| {
		let (base, scalar) =
			generate_scalar_args_projective::<ed_on_bls12_377::EdwardsProjectiveOptimized>();
		assert_ok!(TemplateModule::ed_on_bls12_377_mul_projective_optimized(
			RuntimeOrigin::signed(1),
			base.encode(),
			scalar.encode()
		));
	});
}
#[test]
fn mul_affine_ed_on_bls12_377() {
	new_test_ext().execute_with(|| {
		let (base, scalar) = generate_scalar_args::<ark_ed_on_bls12_377::EdwardsAffine>();
		assert_ok!(TemplateModule::ed_on_bls12_377_mul_affine(
			RuntimeOrigin::signed(1),
			base.encode(),
			scalar.encode()
		));
	});
}
#[test]
fn mul_affine_ed_on_bls12_377_optimized() {
	new_test_ext().execute_with(|| {
		let (base, scalar) = generate_scalar_args::<ed_on_bls12_377::EdwardsAffineOptimized>();
		assert_ok!(TemplateModule::ed_on_bls12_377_mul_affine_optimized(
			RuntimeOrigin::signed(1),
			base.encode(),
			scalar.encode()
		));
	});
}
#[test]
fn msm_sw_ed_on_bls12_381_bandersnatch() {
	new_test_ext().execute_with(|| {
		let (bases, scalars) = generate_msm_args::<
			ark_ec::short_weierstrass::Projective<ark_ed_on_bls12_381_bandersnatch::EdwardsConfig>,
		>(10);
		assert_ok!(TemplateModule::ed_on_bls12_381_bandersnatch_msm_sw(
			RuntimeOrigin::signed(1),
			bases.encode(),
			scalars.encode()
		));
	});
}
#[test]
fn msm_sw_ed_on_bls12_381_bandersnatch_optimized() {
	new_test_ext().execute_with(|| {
		let (bases, scalars) =
			generate_msm_args::<ed_on_bls12_381_bandersnatch::SWProjectiveOptimized>(10);
		assert_ok!(TemplateModule::ed_on_bls12_381_bandersnatch_msm_sw_optimized(
			RuntimeOrigin::signed(1),
			bases.encode(),
			scalars.encode()
		));
	});
}
#[test]
fn mul_projective_sw_ed_on_bls12_381_bandersnatch_optimized() {
	new_test_ext().execute_with(|| {
		let (base, scalar) = generate_scalar_args_projective::<
			ed_on_bls12_381_bandersnatch::SWProjectiveOptimized,
		>();
		assert_ok!(TemplateModule::ed_on_bls12_381_bandersnatch_mul_projective_sw_optimized(
			RuntimeOrigin::signed(1),
			base.encode(),
			scalar.encode()
		));
	});
}
#[test]
fn mul_affine_sw_ed_on_bls12_381_bandersnatch() {
	new_test_ext().execute_with(|| {
		let (base, scalar) = generate_scalar_args::<ark_ed_on_bls12_381_bandersnatch::SWAffine>();
		assert_ok!(TemplateModule::ed_on_bls12_381_bandersnatch_mul_affine_sw(
			RuntimeOrigin::signed(1),
			base.encode(),
			scalar.encode()
		));
	});
}
#[test]
fn mul_affine_sw_ed_on_bls12_381_bandersnatch_optimized() {
	new_test_ext().execute_with(|| {
		let (base, scalar) =
			generate_scalar_args::<ed_on_bls12_381_bandersnatch::SWProjectiveOptimized>();
		assert_ok!(TemplateModule::ed_on_bls12_381_bandersnatch_mul_affine_sw_optimized(
			RuntimeOrigin::signed(1),
			base.encode(),
			scalar.encode()
		));
	});
}
#[test]
fn msm_te_ed_on_bls12_381_bandersnatch() {
	new_test_ext().execute_with(|| {
		let (bases, scalars) =
			generate_msm_args::<ark_ed_on_bls12_381_bandersnatch::EdwardsProjective>(10);
		assert_ok!(TemplateModule::ed_on_bls12_381_bandersnatch_msm_te(
			RuntimeOrigin::signed(1),
			bases.encode(),
			scalars.encode()
		));
	});
}
#[test]
fn msm_te_ed_on_bls12_381_bandersnatch_optimized() {
	new_test_ext().execute_with(|| {
		let (bases, scalars) =
			generate_msm_args::<ed_on_bls12_381_bandersnatch::EdwardsProjectiveOptimized>(10);
		assert_ok!(TemplateModule::ed_on_bls12_381_bandersnatch_msm_te_optimized(
			RuntimeOrigin::signed(1),
			bases.encode(),
			scalars.encode()
		));
	});
}
#[test]
fn mul_projective_te_ed_on_bls12_381_bandersnatch() {
	new_test_ext().execute_with(|| {
		let (base, scalar) = generate_scalar_args_projective::<
			ark_ed_on_bls12_381_bandersnatch::EdwardsProjective,
		>();
		assert_ok!(TemplateModule::ed_on_bls12_381_bandersnatch_mul_projective_te(
			RuntimeOrigin::signed(1),
			base.encode(),
			scalar.encode()
		));
	});
}
#[test]
fn mul_projective_te_ed_on_bls12_381_bandersnatch_optimized() {
	let (base, scalar) = generate_scalar_args_projective::<
		ed_on_bls12_381_bandersnatch::EdwardsProjectiveOptimized,
	>();
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::ed_on_bls12_381_bandersnatch_mul_projective_te_optimized(
			RuntimeOrigin::signed(1),
			base.encode(),
			scalar.encode()
		));
	});
}
#[test]
fn mul_affine_te_ed_on_bls12_381_bandersnatch() {
	new_test_ext().execute_with(|| {
		let (base, scalar) =
			generate_scalar_args::<ark_ed_on_bls12_381_bandersnatch::EdwardsAffine>();
		assert_ok!(TemplateModule::ed_on_bls12_381_bandersnatch_mul_affine_te(
			RuntimeOrigin::signed(1),
			base.encode(),
			scalar.encode()
		));
	});
}
#[test]
fn mul_affine_te_ed_on_bls12_381_bandersnatch_optimized() {
	let (base, scalar) =
		generate_scalar_args::<ed_on_bls12_381_bandersnatch::EdwardsAffineOptimized>();
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::ed_on_bls12_381_bandersnatch_mul_affine_te_optimized(
			RuntimeOrigin::signed(1),
			base.encode(),
			scalar.encode()
		));
	});
}
