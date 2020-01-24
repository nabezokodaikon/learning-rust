fn main() {
    let mut a1 = [5, 4, 3, 2];
    let s1 = &mut a1[1..3];
    s1[0] = 6;
    s1[1] *= 10;
    s1.swap(0, 1);
    assert_eq!(s1, [30, 6]);
    assert_eq!(a1, [5, 30, 6, 2]);

    let a2: [i32; 0] = [];
    let s2 = &a2;
    assert!(s2.is_empty());
    assert_eq!(s2.len(), 0);
    assert_eq!(s2.first(), None);

    let a3 = ["zero", "one", "two", "three", "four"];
    let s3 = &a3[1..4];
    assert!(!s3.is_empty());
    assert_eq!(s3.len(), 3);
    assert_eq!(s3.first(), Some(&"one"));

    assert_eq!(s3[1], "two");
    assert_eq!(s3.get(1), Some(&"two"));
    assert_eq!(s3.get(3), None);

    assert!(s3.contains(&"two"));
    assert!(s3.starts_with(&["one", "two"]));
    assert!(s3.ends_with(&["two", "three"]));

    let mut a4 = [6, 4, 2, 8, 0, 9, 4, 3, 7, 5, 1, 7];

    &mut a4[2..6].sort();
    assert_eq!(&a4[2..6], &[0, 2, 8, 9]);

    let (s4a, s4b) = &mut a4.split_at_mut(5);

    s4a.reverse();
    assert_eq!(s4a, &[8, 2, 0, 4, 6]);

    s4b.sort_unstable();
    assert_eq!(s4b, &[1, 3, 4, 5, 7, 7, 9]);

    let fruits = "あかりんご, あおりんご\nラズベリー, ブラックベリー";
    let mut lines = fruits.lines();
    let apple_line = lines.next();
    assert_eq!(apple_line, Some("あかりんご, あおりんご"));
    assert_eq!(lines.next(), Some("ラズベリー, ブラックベリー"));
    assert_eq!(lines.next(), None);

    if let Some(apples) = apple_line {
        assert!(apples.starts_with("あか"));
        assert!(apples.contains("りんご"));
        assert_eq!(apples.find("あお"), Some(17));

        let mut apple_iter = apples.split(",");
        assert_eq!(apple_iter.next(), Some("あかりんご"));

        let green = apple_iter.next();
        assert_eq!(green, Some(" あおりんご"));
        assert_eq!(green.map(str::trim), Some("あおりんご"));

        assert_eq!(apple_iter.next(), None);
    } else {
        unreachable!();
    }

    let s = "abcあいう";
    assert_eq!(s.get(0..1), Some("a"));
    assert_eq!(s.get(3..6), Some("あ"));
    assert_eq!(s.get(3..4), None);

    let s = "かか\u{3099}く";
    println!("{}", s);

    let mut iter = s.chars();
    assert_eq!(iter.next(), Some('か'));
    assert_eq!(iter.next(), Some('か'));
    assert_eq!(iter.next(), Some('\u{3099}'));
    assert_eq!(iter.next(), Some('く'));
    assert_eq!(iter.next(), None);

    let utf8: [u8; 4] = [0x61, 0xe3, 0x81, 0x82];
    assert_eq!(std::str::from_utf8(&utf8), Ok("aあ"));

    let bad_utf8: [u8; 2] = [0x81, 0x33];
    let result2 = std::str::from_utf8(&bad_utf8);
    assert!(result2.is_err());
    println!("{:?}", result2);

    let mut s1 = "abcあいう".to_string();
    let s2 = s1.as_mut_str();
    s2.make_ascii_uppercase();
    assert_eq!(s2, "ABCあいう");

    let b = unsafe { s2.as_bytes_mut() };
    b[3] = b'*';
    b[4] = b'a';
    b[5] = b'*';

    assert_eq!(s1, "ABC*a*いう");
}
