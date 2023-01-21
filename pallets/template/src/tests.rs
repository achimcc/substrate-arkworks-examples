use crate::mock::*;
use frame_support::assert_ok;

#[test]
fn groth16_verificaton_works_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::verify_groth16_optimized_works(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_g1_bls12_381_works_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::msm_g1_bls12_381_works(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_g2_bls12_381_works_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::msm_g2_bls12_381_works(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g1_bls12_381_works_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_affine_g1_bls12_381_works(RuntimeOrigin::signed(1)));
	});
}

#[test]
fn mul_projective_g1_bls12_381_works_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_projective_g1_bls12_381_works(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g2_bls12_381_works_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_affine_g2_bls12_381_works(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_g2_bls12_381_works_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::pairing_arkworks_bls12_377_works(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn pairing_arkworks_bls12_381_works_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::pairing_arkworks_bls12_381_optimized_works(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_g1_bls12_377_works_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::msm_g1_bls12_377_optimized_works(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_g2_bls12_377_works_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::msm_g2_bls12_377_optimized_works(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g1_bls12_377_works_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_affine_g1_bls12_377_works(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g2_bls12_377_works_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_affine_g2_bls12_377_works(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_g1_bls12_377_works_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_projective_g1_bls12_377_works(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_g2_bls12_377_works_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_projective_g2_bls12_377_works(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn pairing_arkworks_bls12_377_works_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::pairing_arkworks_bls12_377_works(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_g1_bw6_761_works_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::msm_g1_bw6_761_works(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_g2_bw6_761_works_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::msm_g2_bw6_761_works(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g1_bw6_761_works_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_affine_g1_bw6_761_works(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g2_bw6_761_works_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_affine_g2_bw6_761_works(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_g1_bw6_761_works_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_projective_g1_bw6_761_works(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_g2_bw6_761_works_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_projective_g2_bw6_761_works(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_ed_on_bls12_377_works_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::msm_ed_on_bls12_377_works(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_ed_on_bls12_377_works_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_affine_ed_on_bls12_377_works(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_ed_on_bls12_377_works_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_projective_ed_on_bls12_377_works(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_ed_on_bls12_381_works_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::msm_ed_on_bls12_381_works(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_ed_on_bls12_381_works_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_affine_ed_on_bls12_381_works(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_ed_on_bls12_381_works_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_projective_ed_on_bls12_381_works(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn pairing_arkworks_bw6_761_works_optimized() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::pairing_arkworks_bw6_761_works(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn groth16_verification_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::verify_groth16(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn pairing_arkworks_bls12_381_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::pairing_arkworks_bls12_381(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_g1_bls12_381_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::msm_g1_bls12_381(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_g2_bls12_381_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::msm_g2_bls12_381(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g1_bls12_381_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_affine_g1_bls12_381(RuntimeOrigin::signed(1)));
	});
}

#[test]
fn mul_projective_g1_bls12_381_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_projective_g1_bls12_381(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g2_bls12_381_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_affine_g2_bls12_381(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_g2_bls12_381_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_projective_g2_bls12_381(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g1_bls12_377_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_affine_g1_bls12_377(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_g1_bls12_377_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_projective_g1_bls12_377(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g2_bls12_377_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_affine_g2_bls12_377(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_g2_bls12_377_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_projective_g2_bls12_377(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn pairing_arkworks_bls12_377_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::pairing_arkworks_bls12_377(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_g2_bw6_761_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::msm_g2_bw6_761(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g1_bw6_761_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_affine_g1_bw6_761(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_g1_bw6_761_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_projective_g1_bw6_761(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_g2_bw6_761_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_affine_g2_bw6_761(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_g2_bw6_761_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_projective_g2_bw6_761(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_ed_on_bls12_377_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::msm_ed_on_bls12_377(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_ed_on_bls12_377_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_affine_ed_on_bls12_377(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_ed_on_bls12_377_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_projective_ed_on_bls12_377(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn msm_ed_on_bls12_381_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::msm_ed_on_bls12_381(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_affine_ed_on_bls12_381_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_affine_ed_on_bls12_381(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn mul_projective_ed_on_bls12_381_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::mul_projective_ed_on_bls12_381(RuntimeOrigin::signed(1)));
	});
}
#[test]
fn pairing_arkworks_bw6_761_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::pairing_arkworks_bw6_761(RuntimeOrigin::signed(1)));
	});
}
