use crate::mock::*;
use frame_support::assert_ok;

#[test]
fn groth16_verificaton_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::groth16_verification_optimized(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_g1_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::bls12_381_msm_g1(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_g2_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::bls12_381_msm_g2(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g1_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::bls12_381_mul_affine_g1(RuntimeOrigin::signed(1)));
	});
}

#[test]
fn mul_projective_g1_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::bls12_381_mul_projective_g1(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g2_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::bls12_381_mul_affine_g2(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn pairing_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::bls12_381_pairing_optimized(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_g1_bls12_377_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::bls12_377_msm_g1_optimized(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_g2_bls12_377_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::bls12_377_msm_g2_optimized(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g1_bls12_377_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::bls12_377_mul_affine_g1(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g2_bls12_377_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::bls12_377_mul_affine_g2(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_g1_bls12_377_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::bls12_377_mul_projective_g1(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_g2_bls12_377_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::bls12_377_mul_projective_g2(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn pairin_gbls12_377_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::bls12_377_pairing(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_g1_bw6_761_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::bw6_761_msm_g1(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_g2_bw6_761_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::bw6_761_msm_g2(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g1_bw6_761_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::bw6_761_mul_affine_g1(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g2_bw6_761_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::bw6_761_mul_affine_g2(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_g1_bw6_761_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::bw6_761_mul_projective_g1(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_g2_bw6_761_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::bw6_761_mul_projective_g2(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_ed_on_bls12_377_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::ed_on_bls12_377_msm(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_ed_on_bls12_377_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::ed_on_bls12_377_mul_affine(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_ed_on_bls12_377_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::ed_on_bls12_377_mul_projective(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_ed_on_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::ed_on_bls12_381_msm(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_ed_on_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::ed_on_bls12_381_mul_affine(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_ed_on_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::ed_on_bls12_381_mul_projective(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn pairing_bw6_761_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::bw6_761_pairing(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn groth16_verification() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::groth16_verification(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn pairing_bls12_381() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::bls12_381_pairing(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_g1_bls12_381() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::bls12_381_msm_g1(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_g2_bls12_381() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::bls12_381_msm_g2(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g1_bls12_381() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::bls12_381_mul_affine_g(RuntimeOrigin::signed(1)));
	});
}

#[test]
fn mul_projective_g1_bls12_381() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::bls12_381_mul_projective_g1(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g2_bls12_381() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::bls12_381_mul_affine_g2(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_g2_bls12_381() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::bls12_381_mul_projective_g2(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g1_bls12_377() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::bls12_377_mul_affine_g1(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_g1_bls12_377() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::bls12_377_mul_projective_g1(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g2_bls12_377() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::bls12_377_mul_affine_g2(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_g2_bls12_377() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::bls12_377_mul_projective_g2(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn pairing_bls12_377() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::bls12_377_pairing(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_g2_bw6_761() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::bw6_761_msm_g2(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g1_bw6_761() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::bw6_761_mul_affine_g1(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_g1_bw6_761() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::bw6_761_mul_projective_g1(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g2_bw6_761() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::bw6_761_mul_affine_g2(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_g2_bw6_761() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::bw6_761_mul_projective_g2(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_ed_on_bls12_377() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::ed_on_bls12_377_msm(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_ed_on_bls12_377() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::ed_on_bls12_377_mul_affine(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_ed_on_bls12_377() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::ed_on_bls12_377_mul_projective(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_ed_on_bls12_381() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::ed_on_bls12_381_msm(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_ed_on_bls12_381() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::ed_on_bls12_381_mul_affine(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_ed_on_bls12_381() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::ed_on_bls12_381_mul_projective(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn pairing_bw6_761() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::bw6_761_pairing(RuntimeOrigin::signed(1)));
	});
}
