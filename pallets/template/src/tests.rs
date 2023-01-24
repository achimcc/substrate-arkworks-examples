use crate::mock::*;
use frame_support::assert_ok;

#[test]
fn groth16_verificaton_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::verify_groth16_optimized(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_g1_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::msm_g1_bls12_381(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_g2_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::msm_g2_bls12_381(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g1_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_affine_g1_bls12_381(RuntimeOrigin::signed(1)));
	});
}

#[test]
fn mul_projective_g1_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_projective_g1_bls12_381(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g2_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_affine_g2_bls12_381(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn pairing_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::pairing_bls12_381_optimized(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_g1_bls12_377_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::msm_g1_bls12_377_optimized(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_g2_bls12_377_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::msm_g2_bls12_377_optimized(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g1_bls12_377_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_affine_g1_bls12_377(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g2_bls12_377_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_affine_g2_bls12_377(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_g1_bls12_377_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_projective_g1_bls12_377(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_g2_bls12_377_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_projective_g2_bls12_377(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn pairin_gbls12_377_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::pairing_bls12_377(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_g1_bw6_761_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::msm_g1_bw6_761(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_g2_bw6_761_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::msm_g2_bw6_761(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g1_bw6_761_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_affine_g1_bw6_761(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g2_bw6_761_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_affine_g2_bw6_761(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_g1_bw6_761_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_projective_g1_bw6_761(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_g2_bw6_761_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_projective_g2_bw6_761(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_ed_on_bls12_377_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::msm_ed_on_bls12_377(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_ed_on_bls12_377_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_affine_ed_on_bls12_377(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_ed_on_bls12_377_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_projective_ed_on_bls12_377(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_ed_on_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::msm_ed_on_bls12_381(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_ed_on_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_affine_ed_on_bls12_381(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_ed_on_bls12_381_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_projective_ed_on_bls12_381(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn pairing_bw6_761_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::pairing_bw6_761(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn groth16_verification() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::verify_groth16(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn pairing_bls12_381() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::pairing_bls12_381(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_g1_bls12_381() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::msm_g1_bls12_381(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_g2_bls12_381() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::msm_g2_bls12_381(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g1_bls12_381() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_affine_g1_bls12_381(RuntimeOrigin::signed(1)));
	});
}

#[test]
fn mul_projective_g1_bls12_381() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_projective_g1_bls12_381(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g2_bls12_381() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_affine_g2_bls12_381(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_g2_bls12_381() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_projective_g2_bls12_381(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g1_bls12_377() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_affine_g1_bls12_377(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_g1_bls12_377() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_projective_g1_bls12_377(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g2_bls12_377() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_affine_g2_bls12_377(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_g2_bls12_377() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_projective_g2_bls12_377(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn pairing_bls12_377() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::pairing_bls12_377(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_g2_bw6_761() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::msm_g2_bw6_761(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g1_bw6_761() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_affine_g1_bw6_761(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_g1_bw6_761() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_projective_g1_bw6_761(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g2_bw6_761() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_affine_g2_bw6_761(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_g2_bw6_761() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_projective_g2_bw6_761(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_ed_on_bls12_377() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::msm_ed_on_bls12_377(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_ed_on_bls12_377() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_affine_ed_on_bls12_377(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_ed_on_bls12_377() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_projective_ed_on_bls12_377(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_ed_on_bls12_381() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::msm_ed_on_bls12_381(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_ed_on_bls12_381() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_affine_ed_on_bls12_381(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_ed_on_bls12_381() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_projective_ed_on_bls12_381(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn pairing_bw6_761() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::pairing_bw6_761(RuntimeOrigin::signed(1)));
	});
}
