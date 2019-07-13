use xdelta::Index;

const SOURCE: &str = "the quick brown fox jumps over the slow lazy dog";
const TARGET: &str = "a swift auburn fox jumps over three dormant hounds";

#[test]
fn delta_len() {
    let delta = Index::new(SOURCE).compress(TARGET);

    let lengths = [
        2,
        1 + "a swift aubur".len(),
        3,
        1 + "ree dormant hounds".len(),
    ];

    assert!(!delta.is_empty());
    assert_eq!(delta.len(), lengths.iter().sum());
}

#[test]
fn delta_to_vec() {
    let delta = Index::new(SOURCE).compress(TARGET);

    let bytes = vec![
        vec![0x30, 0x32],
        vec![0x0d],
        "a swift aubur".bytes().collect(),
        vec![0b1_001_0001, 0x0e, 0x13],
        vec![0x12],
        "ree dormant hounds".bytes().collect(),
    ];

    let buffer: Vec<u8> = bytes.into_iter().flatten().collect();

    assert_eq!(delta.len(), delta.to_vec().len());
    assert_eq!(delta.to_vec(), buffer);
}
