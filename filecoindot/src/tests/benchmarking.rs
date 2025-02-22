// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! Benchmarking setup for filecoindot

use crate::tests::mock::{Origin, Test, ALICE, RELAYER4};
use crate::*;
#[allow(unused)]
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, vec, whitelisted_caller};

benchmarks! {
    add_relayer {
    }: {
        Pallet::<Test>::add_relayer(Origin::signed(ALICE), RELAYER4)?;
    } verify {
        assert!(Relayers::<Test>::contains_key(&RELAYER4));
    }

    remove_relayer {
        Pallet::<Test>::add_relayer(Origin::signed(ALICE), RELAYER4)?;
    }: {
        Pallet::<Test>::remove_relayer(Origin::signed(ALICE), RELAYER4)?;
    } verify {
        assert!(!Relayers::<Test>::contains_key(&RELAYER4));
    }

    set_vote_threshold {
    }: {
        Pallet::<Test>::set_vote_threshold(Origin::signed(ALICE), 2)?;
    } verify {
        assert_eq!(VoteThreshold::<Test>::get(), 2);
    }

    submit_block_vote_approve {
        Pallet::<Test>::add_relayer(Origin::signed(ALICE), ALICE)?;
        Pallet::<Test>::set_vote_threshold(Origin::signed(ALICE), 1)?;
    }: {
        Pallet::<Test>::submit_block_vote(Origin::signed(ALICE), vec![0], vec![0])?;
    } verify {
        assert!(!BlockSubmissionProposals::<Test>::contains_key(&vec![0]));
    }
}

impl_benchmark_test_suite!(
    Pallet,
    crate::tests::mock::ExtBuilder::default().build(),
    crate::tests::mock::Test
);
