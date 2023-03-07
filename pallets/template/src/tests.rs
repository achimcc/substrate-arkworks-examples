use crate::{
	bls12_377, bls12_381, bw6_761, ed_on_bls12_377, ed_on_bls12_381, mock::*,
	utils::generate_arguments,
};
use frame_support::assert_ok;

#[test]
fn groth16_verification() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::groth16_verification(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn groth16_verificaton_optimized() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::groth16_verification_optimized(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn pairing_bls12_381() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::bls12_381_pairing(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn pairing_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::bls12_381_pairing_optimized(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_g1_bls12_381() {
	new_test_ext().execute_with(|| {
		let (bases, scalars) = generate_arguments::<
			ark_ec::short_weierstrass::Projective<ark_bls12_381::g1::Config>,
		>(10);
		assert_ok!(TemplateModule::bls12_381_msm_g1(RuntimeOrigin::signed(1), bases, scalars));
	});
}
#[test]
fn msm_g1_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		let (bases, scalars) = generate_arguments::<
			sp_ark_models::short_weierstrass::Projective<
				sp_ark_bls12_381::g1::Config<bls12_381::HostBls12_381>,
			>,
		>(10);
		assert_ok!(TemplateModule::bls12_381_msm_g1_optimized(
			RuntimeOrigin::signed(1),
			bases,
			scalars
		));
	});
}
#[test]
fn msm_g2_bls12_381() {
	new_test_ext().execute_with(|| {
		let (bases, scalars) = generate_arguments::<
			ark_ec::short_weierstrass::Projective<ark_bls12_381::g2::Config>,
		>(10);
		assert_ok!(TemplateModule::bls12_381_msm_g2(RuntimeOrigin::signed(1), bases, scalars));
	});
}
#[test]
fn msm_g2_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		let (bases, scalars) = generate_arguments::<
			sp_ark_models::short_weierstrass::Projective<
				sp_ark_bls12_381::g2::Config<bls12_381::HostBls12_381>,
			>,
		>(10);
		assert_ok!(TemplateModule::bls12_381_msm_g2_optimized(
			RuntimeOrigin::signed(1),
			bases,
			scalars
		));
	});
}
#[test]
fn mul_projective_g1_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::bls12_381_mul_projective_g1(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_g1_bls12_381() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::bls12_381_mul_projective_g1_optimized(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g1_bls12_381() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::bls12_381_mul_affine_g1(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g1_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::bls12_381_mul_affine_g1_optimized(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_g2_bls12_381() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::bls12_381_mul_projective_g2(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_g2_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::bls12_381_mul_projective_g2_optimized(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g2_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::bls12_381_mul_affine_g2(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g2_bls12_381() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::bls12_381_mul_affine_g2_optimized(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn pairing_bls12_377() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::bls12_377_pairing(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn pairing_bls12_377_optimized() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::bls12_377_pairing_optimized(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_g1_bls12_377() {
	let (bases, scalars) =
		generate_arguments::<ark_ec::short_weierstrass::Projective<ark_bls12_377::g1::Config>>(10);
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::bls12_377_msm_g1(RuntimeOrigin::signed(1), bases, scalars));
	});
}
#[test]
fn msm_g1_bls12_377_optimized() {
	let (bases, scalars) = generate_arguments::<
		ark_ec::short_weierstrass::Projective<
			sp_ark_bls12_377::g1::Config<bls12_377::HostBls12_377>,
		>,
	>(10);
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::bls12_377_msm_g1_optimized(
			RuntimeOrigin::signed(1),
			bases,
			scalars
		));
	});
}
#[test]
fn msm_g2_bls12_377() {
	let (bases, scalars) =
		generate_arguments::<ark_ec::short_weierstrass::Projective<ark_bls12_377::g2::Config>>(10);
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::bls12_377_msm_g2(RuntimeOrigin::signed(1), bases, scalars));
	});
}
#[test]
fn msm_g2_bls12_377_optimized() {
	let (bases, scalars) = generate_arguments::<
		ark_ec::short_weierstrass::Projective<
			sp_ark_bls12_377::g2::Config<bls12_377::HostBls12_377>,
		>,
	>(10);
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::bls12_377_msm_g2_optimized(
			RuntimeOrigin::signed(1),
			bases,
			scalars
		));
	});
}
#[test]
fn mul_projective_g1_bls12_377() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::bls12_377_mul_projective_g1(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_g1_bls12_377_optimized() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::bls12_377_mul_projective_g1_optimized(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g1_bls12_377() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::bls12_377_mul_affine_g1(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g1_bls12_377_optimized() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::bls12_377_mul_affine_g1_optimized(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_g2_bls12_377() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::bls12_377_mul_projective_g2(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_g2_bls12_377_optimized() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::bls12_377_mul_projective_g2_optimized(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g2_bls12_377() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::bls12_377_mul_affine_g2(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g2_bls12_377_optimized() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::bls12_377_mul_affine_g2_optimized(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn pairing_bw6_761() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::bw6_761_pairing(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn pairing_bw6_761_optimized() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::bw6_761_pairing_optimized(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_g1_bw6_761() {
	new_test_ext().execute_with(|| {
		let (bases, scalars) = generate_arguments::<
			ark_ec::short_weierstrass::Projective<ark_bw6_761::g1::Config>,
		>(10);
		assert_ok!(TemplateModule::bw6_761_msm_g1(RuntimeOrigin::signed(1), bases, scalars));
	});
}
#[test]
fn msm_g1_bw6_761_optimized() {
	new_test_ext().execute_with(|| {
		let (bases, scalars) = generate_arguments::<
			ark_ec::short_weierstrass::Projective<sp_ark_bw6_761::g1::Config<bw6_761::HostBW6_761>>,
		>(10);
		assert_ok!(TemplateModule::bw6_761_msm_g1_optimized(
			RuntimeOrigin::signed(1),
			bases,
			scalars
		));
	});
}
#[test]
fn msm_g2_bw6_761() {
	new_test_ext().execute_with(|| {
		let (bases, scalars) = generate_arguments::<
			ark_ec::short_weierstrass::Projective<ark_bw6_761::g2::Config>,
		>(10);
		assert_ok!(TemplateModule::bw6_761_msm_g2(RuntimeOrigin::signed(1), bases, scalars));
	});
}
#[test]
fn msm_g2_bw6_761_optimized() {
	new_test_ext().execute_with(|| {
		let (bases, scalars) = generate_arguments::<
			ark_ec::short_weierstrass::Projective<sp_ark_bw6_761::g2::Config<bw6_761::HostBW6_761>>,
		>(10);
		assert_ok!(TemplateModule::bw6_761_msm_g2_optimized(
			RuntimeOrigin::signed(1),
			bases,
			scalars
		));
	});
}
#[test]
fn mul_projective_sw_ed_on_bls12_381() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::ed_on_bls12_381_mul_projective_sw(RuntimeOrigin::signed(1),));
	});
}
#[test]
fn mul_projective_g1_bw6_761() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::bw6_761_mul_projective_g1(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_g1_bw6_761_optimized() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::bw6_761_mul_projective_g1_optimized(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g1_bw6_761() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::bw6_761_mul_affine_g1(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g1_bw6_761_optimized() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::bw6_761_mul_affine_g1_optimized(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_g2_bw6_761() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::bw6_761_mul_projective_g2(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_g2_bw6_761_optimized() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::bw6_761_mul_projective_g2_optimized(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g2_bw6_761() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::bw6_761_mul_affine_g2(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g2_bw6_761_optimized() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::bw6_761_mul_affine_g2_optimized(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_ed_on_bls12_377() {
	new_test_ext().execute_with(|| {
		let (bases, scalars) = generate_arguments::<
			sp_ark_ed_on_bls12_377::EdwardsProjective<ed_on_bls12_377::HostEdOnBls12_377>,
		>(10);
		assert_ok!(TemplateModule::ed_on_bls12_377_msm(RuntimeOrigin::signed(1), bases, scalars));
	});
}
#[test]
fn msm_ed_on_bls12_377_optimized() {
	new_test_ext().execute_with(|| {
		let (bases, scalars) = generate_arguments::<
			sp_ark_ed_on_bls12_377::EdwardsProjective<ed_on_bls12_377::HostEdOnBls12_377>,
		>(10);
		assert_ok!(TemplateModule::ed_on_bls12_377_msm_optimized(
			RuntimeOrigin::signed(1),
			bases,
			scalars
		));
	});
}
#[test]
fn mul_projective_ed_on_bls12_377() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::ed_on_bls12_377_mul_projective(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_ed_on_bls12_377_optimized() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::ed_on_bls12_377_mul_projective_optimized(
			RuntimeOrigin::signed(1)
		));
	});
}
#[test]
fn mul_affine_ed_on_bls12_377() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::ed_on_bls12_377_mul_affine(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_ed_on_bls12_377_optimized() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::ed_on_bls12_377_mul_affine_optimized(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_sw_ed_on_bls12_381() {
	new_test_ext().execute_with(|| {
		let (bases, scalars) = generate_arguments::<
			ark_ec::short_weierstrass::Projective<ark_ed_on_bls12_381::EdwardsConfig>,
		>(10);
		assert_ok!(TemplateModule::ed_on_bls12_381_msm_sw(
			RuntimeOrigin::signed(1),
			bases,
			scalars
		));
	});
}
#[test]
fn msm_sw_ed_on_bls12_381_optimized() {
	let (bases, scalars) = generate_arguments::<
		sp_ark_models::short_weierstrass::Projective<
			sp_ark_ed_on_bls12_381::EdwardsConfig<ed_on_bls12_381::HostEdOnBls12_381>,
		>,
	>(10);
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::ed_on_bls12_381_mul_projective_sw(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_sw_ed_on_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::ed_on_bls12_381_mul_projective_sw_optimized(
			RuntimeOrigin::signed(1)
		));
	});
}
#[test]
fn mul_affine_sw_ed_on_bls12_381() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::ed_on_bls12_381_mul_affine_sw(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_sw_ed_on_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::ed_on_bls12_381_mul_affine_sw_optimized(RuntimeOrigin::signed(
			1
		)));
	});
}
#[test]
fn msm_te_ed_on_bls12_381() {
	new_test_ext().execute_with(|| {
		let (bases, scalars) = generate_arguments::<ark_ed_on_bls12_381::EdwardsProjective>(10);
		assert_ok!(TemplateModule::ed_on_bls12_381_msm_te(
			RuntimeOrigin::signed(1),
			bases,
			scalars
		));
	});
}
#[test]
fn msm_te_ed_on_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		let (bases, scalars) = generate_arguments::<
			sp_ark_ed_on_bls12_381::EdwardsProjective<ed_on_bls12_381::HostEdOnBls12_381>,
		>(10);
		assert_ok!(TemplateModule::ed_on_bls12_381_msm_te_optimized(
			RuntimeOrigin::signed(1),
			bases,
			scalars
		));
	});
}
#[test]
fn mul_projective_te_ed_on_bls12_381() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::ed_on_bls12_381_mul_projective_te(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_te_ed_on_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::ed_on_bls12_381_mul_projective_te_optimized(
			RuntimeOrigin::signed(1)
		));
	});
}
#[test]
fn mul_affine_te_ed_on_bls12_381() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::ed_on_bls12_381_mul_affine_te(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_te_ed_on_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::ed_on_bls12_381_mul_affine_te_optimized(RuntimeOrigin::signed(
			1
		)));
	});
}
