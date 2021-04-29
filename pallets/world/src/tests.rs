use crate::{mock::*, Error};
use frame_support::assert_noop;

#[test]
fn fill_chunk() {
    new_test_ext().execute_with(|| {
        World::claim_chunk(Origin::signed(1), [0, 0, 0]).unwrap();
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    World::write_block(Origin::signed(1), [x, y, z], 1).unwrap();
                }
            }
        }
        assert_eq!(
            crate::Chunks::<Test>::get([0, 0, 0]),
            Some((1, [1; 16 * 16 * 16])),
        );
    });
}

#[test]
fn unauthorized() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            World::write_block(Origin::signed(1), [0, 0, 0], 1),
            Error::<Test>::ChunkDoesNotExist,
        );
    });
}

#[test]
fn notyours() {
    new_test_ext().execute_with(|| {
        World::claim_chunk(Origin::signed(1), [0, 0, 0]).unwrap();
        assert_noop!(
            World::write_block(Origin::signed(2), [0, 0, 0], 1),
            Error::<Test>::NotYours,
        );
    });
}
