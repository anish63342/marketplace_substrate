use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn it_stores_correctly() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::store_product(Origin::signed(1),CF4865oY,iphone12,80000,Anish C));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::proof(), Some(CF4865oY,iphone12));

		assert_ok!(TemplateModule::store_product(Origin::signed(1),HG9666kJ,apple macbook air,94000,AnishCY));
		assert_eq!(TemplateModule::proof(), Some(HG9666kJ,apple macbook air));

		assert_ok!(TemplateModule::store_product(Origin::signed(1),BD5400eL,nike airmax11,14000,Shinichi K));
		assert_eq!(TemplateModule::proof(), Some(BD5400eL,nike airmax11));

		assert_ok!(TemplateModule::store_product(Origin::signed(1),AS89881xW, yonex nanoray 18i,3000,Silver Bullet));
		assert_eq!(TemplateModule::proof(), Some(AS89881xW, yonex nanoray 18i));
	});
}

// #[test]
// fn correct_error_for_none_value() {
// 	new_test_ext().execute_with(|| {
// 		// Ensure the expected error is thrown when no value is present.
// 		assert_noop!(
// 			TemplateModule::cause_error(Origin::signed(1)),
// 			Error::<Test>::NoneValue
// 		);
// 	});
// }
