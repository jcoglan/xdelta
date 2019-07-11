use xdelta::Index;

#[test]
fn delta_len() {
    let source = "the quick brown fox jumps over the slow lazy dog";
    let target = "a swift auburn fox jumps over three dormant hounds";

    let delta = Index::new(source).compress(target);

    let lengths = [
        2,
        1 + "a swift aubur".len(),
        3,
        1 + "ree dormant hounds".len(),
    ];

    assert!(!delta.is_empty());
    assert_eq!(delta.len(), lengths.iter().sum());
}
