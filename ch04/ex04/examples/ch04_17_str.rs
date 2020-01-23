fn main() {
    let s1 = "abc1";
    let s2 = "abc2";
    assert!(s1 < s2);
    assert!(s1 != s2);

    let s3 = "文字列を複数行に渡って書くと
        改行やスペースが入る";
    let s4 = "行末にバックスラッシュを付けると\
              改行などが入らない";
    assert_eq!(
        s3,
        "文字列を複数行に渡って書くと\n        改行やスペースが入る"
    );
    assert_eq!(s4, "行末にバックスラッシュを付けると改行などが入らない");

    let _s5 = "文字列に\"と\\を含める";
    let _s6 = r#"文字列に"と\を含める"#;
    let s7 = r###"このように#の数を増やすと"##"があっても大丈夫"###;
    assert_eq!(s7, "このように#の数を増やすと\"##\"があっても大丈夫");

    let s8 = "もちろん絵文字\u{1f600}も使える";
    println!("{}", s8);
}
