pub fn binary_convert(s: &str) -> usize {
    let mut n = 0;

    for (i, c) in s.chars().rev().enumerate() {
        match c {
            '0' => {},
            '1' => {
                n += 2usize.pow(i as u32);
            },
            _ => {},
        }
    }

    n
}

#[test]
fn test_binary_convert() {
    assert_eq!(binary_convert("110"), 6);
    assert_eq!(binary_convert("101010"), 42);
}
