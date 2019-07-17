#![cfg(test)]

use super::Op::{Copy, Insert};
use crate::Index;

//  0               16               32               48
//  +----------------+----------------+----------------+
//  |the quick brown |fox jumps over t|he slow lazy dog|
//  +----------------+----------------+----------------+

#[test]
fn compress_string() {
    let source = "the quick brown fox jumps over the slow lazy dog";
    let target = "a swift auburn fox jumps over three dormant hounds";

    let delta = Index::new(source).compress(target);

    assert_eq!(
        delta.ops,
        vec![
            Insert("a swift aubur".into()),
            Copy(14, 19),
            Insert("ree dormant hounds".into())
        ]
    );
}

#[test]
fn compress_incomplete_block() {
    let source = "the quick brown fox jumps over the slow lazy dog";
    let target = "he quick brown fox jumps over trees";

    let delta = Index::new(source).compress(target);

    assert_eq!(delta.ops, vec![Copy(1, 31), Insert("rees".into())]);
}

#[test]
fn compress_at_source_start() {
    let source = "the quick brown fox jumps over the slow lazy dog";
    let target = "the quick brown ";

    let delta = Index::new(source).compress(target);

    assert_eq!(delta.ops, vec![Copy(0, 16)]);
}

#[test]
fn compress_at_source_start_with_right_expansion() {
    let source = "the quick brown fox jumps over the slow lazy dog";
    let target = "the quick brown fox hops";

    let delta = Index::new(source).compress(target);

    assert_eq!(delta.ops, vec![Copy(0, 20), Insert("hops".into())]);
}

#[test]
fn compress_at_source_start_with_left_offset() {
    let source = "the quick brown fox jumps over the slow lazy dog";
    let target = "behold the quick brown foal";

    let delta = Index::new(source).compress(target);

    assert_eq!(
        delta.ops,
        vec![Insert("behold ".into()), Copy(0, 18), Insert("al".into())]
    );
}

#[test]
fn compress_at_source_end() {
    let source = "the quick brown fox jumps over the slow lazy dog";
    let target = "he slow lazy dog";

    let delta = Index::new(source).compress(target);

    assert_eq!(delta.ops, vec![Copy(32, 16)]);
}

#[test]
fn compress_at_source_end_with_left_expansion() {
    let source = "the quick brown fox jumps over the slow lazy dog";
    let target = "under the slow lazy dog";

    let delta = Index::new(source).compress(target);

    assert_eq!(delta.ops, vec![Insert("und".into()), Copy(28, 20)]);
}

#[test]
fn compress_at_source_end_with_right_offset() {
    let source = "the quick brown fox jumps over the slow lazy dog";
    let target = "under the slow lazy dog's legs";

    let delta = Index::new(source).compress(target);

    assert_eq!(
        delta.ops,
        vec![Insert("und".into()), Copy(28, 20), Insert("'s legs".into())]
    );
}

#[test]
fn compress_unindexed_bytes() {
    let source = "the quick brown fox";
    let target = "see the quick brown fox";

    let delta = Index::new(source).compress(target);

    assert_eq!(delta.ops, vec![Insert("see ".into()), Copy(0, 19)]);
}

#[test]
fn do_not_compress_unindexed_bytes() {
    let source = "the quick brown fox";
    let target = "a quick brown fox";

    let delta = Index::new(source).compress(target);

    assert_eq!(delta.ops, vec![Insert("a quick brown fox".into())]);
}
