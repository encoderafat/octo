use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_adds_qualifier_should_work() {
	new_test_ext().execute_with(|| {
		//Dispatch a signed extrinsic.
		assert_ok!(Bhdao::add_qualifier(Origin::root(),1));
		// Read pallet storage and assert an expected result.
		assert_eq!(Bhdao::qualifiers_uid_count(), 1);
	});
}

#[test]
fn it_adds_qualifier_should_fail() {
	new_test_ext().execute_with(|| {
		assert_ok!(Bhdao::add_qualifier(Origin::root(),1));
		assert_noop!(Bhdao::add_qualifier(Origin::root(),1), Error::<Test>::QualifierAlreadyExists);
	});
}

#[test]
fn it_adds_contributor_should_work() {
	new_test_ext().execute_with(|| {
		//Dispatch a signed extrinsic.
		assert_ok!(Bhdao::add_contributor(Origin::root(),1));
		// Read pallet storage and assert an expected result.
		assert_eq!(Bhdao::contributors_uid_count(), 1);
	});
}

#[test]
fn it_adds_contributor_should_fail() {
	new_test_ext().execute_with(|| {
		assert_ok!(Bhdao::add_contributor(Origin::root(),1));
		assert_noop!(Bhdao::add_contributor(Origin::root(),1), Error::<Test>::ContributorAlreadyExists);
	});
}

#[test]
fn it_adds_collector_should_work() {
	new_test_ext().execute_with(|| {
		//Dispatch a signed extrinsic.
		assert_ok!(Bhdao::add_collector(Origin::root(),1));
		// Read pallet storage and assert an expected result.
		assert_eq!(Bhdao::collectors_uid_count(), 1);
	});
}

#[test]
fn it_adds_collector_should_fail() {
	new_test_ext().execute_with(|| {
		assert_ok!(Bhdao::add_collector(Origin::root(),1));
		assert_noop!(Bhdao::add_collector(Origin::root(),1), Error::<Test>::CollectorAlreadyExists);
	});
}

#[test]
fn it_creates_document_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(Bhdao::add_contributor(Origin::root(),2));
		assert_ok!(Bhdao::create_document(Origin::signed(2),b"Doc1".to_vec(),b"Test1".to_vec(),b"pdf".to_vec(),b"https://ipfs.hash".to_vec()));
		assert_eq!(Bhdao::get_total_items(),1);
	});
	
}

#[test]
fn it_changes_qualification_voting_window_should_work() {
	new_test_ext().execute_with(|| {
		assert_eq!(Bhdao::get_qualification_voting_window(),14400u32);
		assert_ok!(Bhdao::set_qualification_voting_window(Origin::root(),1000u32));
		assert_eq!(Bhdao::get_qualification_voting_window(),1000u32);
	});	
}

#[test]
fn it_changes_verification_voting_window_should_work() {
	new_test_ext().execute_with(|| {
		assert_eq!(Bhdao::get_verification_voting_window(),14400u32);
		assert_ok!(Bhdao::set_verification_voting_window(Origin::root(),1000u32));
		assert_eq!(Bhdao::get_verification_voting_window(),1000u32);
	});	
}

#[test]
fn it_creates_qualification_voting_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(Bhdao::add_contributor(Origin::root(),2));
		assert_ok!(Bhdao::create_document(Origin::signed(2),b"Doc1".to_vec(),b"Test1".to_vec(),b"pdf".to_vec(),b"https://ipfs.hash".to_vec()));
		assert_eq!(Bhdao::get_total_items(),1);
		assert_ok!(Bhdao::add_qualifier(Origin::root(),4));
		assert_ok!(Bhdao::create_qualification_voting(Origin::signed(4),1));
	});	
}

#[test]
fn it_changes_qualification_quorum_should_work() {
	new_test_ext().execute_with(|| {
		assert_eq!(Bhdao::get_qualification_quorum(),0u32);
		assert_ok!(Bhdao::set_qualification_quorum(Origin::root(),10u32));
		assert_eq!(Bhdao::get_qualification_quorum(),10u32);
	});	
}

#[test]
fn it_changes_verification_quorum_should_work() {
	new_test_ext().execute_with(|| {
		assert_eq!(Bhdao::get_verification_quorum(),0u32);
		assert_ok!(Bhdao::set_verification_quorum(Origin::root(),1000u32));
		assert_eq!(Bhdao::get_verification_quorum(),1000u32);
	});	
}

#[test]
fn it_creates_and_finalizes_qualification_voting_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(Bhdao::add_contributor(Origin::root(),2));
		assert_ok!(Bhdao::create_document(Origin::signed(2),b"Doc1".to_vec(),b"Test1".to_vec(),b"pdf".to_vec(),b"https://ipfs.hash".to_vec()));
		assert_eq!(Bhdao::get_total_items(),1);
		assert_ok!(Bhdao::add_qualifier(Origin::root(),4));
		assert_ok!(Bhdao::create_qualification_voting(Origin::signed(4),1));
	});	
}

#[test]
fn it_casts_votes_and_verifies_voting_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(Bhdao::init_collections(Origin::root()));
		//Create Three contributors
		assert_ok!(Bhdao::add_contributor(Origin::root(),1));
		assert_ok!(Bhdao::add_contributor(Origin::root(),2));
		assert_ok!(Bhdao::add_contributor(Origin::root(),3));
		// Create three qualifiers
		assert_ok!(Bhdao::add_qualifier(Origin::root(),4));
		assert_ok!(Bhdao::add_qualifier(Origin::root(),5));
		assert_ok!(Bhdao::add_qualifier(Origin::root(),6));

		// create a new document

		assert_ok!(Bhdao::create_document(Origin::signed(2),b"Doc1".to_vec(),b"Test1".to_vec(),b"pdf".to_vec(),b"https://ipfs.hash".to_vec()));
		assert_eq!(Bhdao::get_total_items(),1);

		run_to_block(10);

		// Change Qualification Window voting to 100 blocks
		assert_ok!(Bhdao::set_qualification_voting_window(Origin::root(),100u32));

		run_to_block(15);

		// Create Qualification voting for Document 1
		assert_ok!(Bhdao::create_qualification_voting(Origin::signed(4),1));

		run_to_block(20);

		//cast votes two Yays One nay
		assert_ok!(Bhdao::cast_qualification_vote(Origin::signed(4),1,true));
		assert_ok!(Bhdao::cast_qualification_vote(Origin::signed(5),1,true));
		assert_ok!(Bhdao::cast_qualification_vote(Origin::signed(6),1,false));

		// Skip 100 blocks
		run_to_block(120);

		// Finalize qualification voting
		assert_ok!(Bhdao::finalize_qualification_voting(Origin::signed(4),1));

		// Change Verification Voting Window to 100 blocks
		assert_ok!(Bhdao::set_verification_voting_window(Origin::root(),100u32));

		run_to_block(150);

		// Create Verification voting for Document 1
		assert_ok!(Bhdao::create_verification_voting(Origin::signed(2),1));

		run_to_block(155);

		//cast votes two Yays One nay
		assert_ok!(Bhdao::cast_verification_vote(Origin::signed(1),1,true));
		assert_ok!(Bhdao::cast_verification_vote(Origin::signed(2),1,true));
		assert_ok!(Bhdao::cast_verification_vote(Origin::signed(3),1,false));

		// Skip 100 blocks
		run_to_block(255);

		// Finalize verification voting
		assert_ok!(Bhdao::finalize_verification_voting(Origin::signed(2),1));
	});	
}