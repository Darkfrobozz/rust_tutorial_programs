use std::cmp::Ordering;
fn main() {
    println!("Hello, world!");
    let mut s = String::from("hithere partner, my name is");
    let first = grab_first_word(&mut s);
    println!("{first}"); 
}

fn grab_first_word(s:&mut String) -> String {
    let mut result_string = String::new();
    let mut s_iter = s.chars();

    while let Some(c) = s_iter.next() {
        if c.cmp(&' ') == Ordering::Equal {
            break;
        }
        result_string.push(c);
        
    }
    result_string
}
