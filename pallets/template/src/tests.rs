use crate::mock::*;
use frame_support::{assert_noop, assert_ok};

#[test]
fn groth16_verification_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::verify_groth16(RuntimeOrigin::signed(1)));
	});
}
