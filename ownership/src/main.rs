fn main() {
    let mut s = String::from("hello");
    println!("{s}");
    let s2 = s;
    println!("{s2}");
    takes_ownership(s2);
    let x = 5;
    makes_copy(x);
    s = String::from("hello");
    s = takes_and_returns(s);
    println!("{s}");
}

fn takes_ownership(some_string : String) {
    println!("{}", some_string);
}
fn makes_copy(some_integer : i32) {
    println!("{}", some_integer);
}
fn takes_and_returns(some_string : String) -> String {
    some_string
}


