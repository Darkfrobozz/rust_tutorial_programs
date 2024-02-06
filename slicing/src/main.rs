fn main() {
    let s = String::from("hello, world");
    let slice1 = get_first_word(&s);
    assert_eq!(slice1, "hello,");
    assert_eq!(s, "hello,");
}

fn get_first_word(s:&str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
