fn find_fibonachi(n : u32) -> u32 {
    if n <= 1 { n } else { find_fibonachi(n - 1) + find_fibonachi(n - 2)}
}

fn main() { 
    const END : u32 = 10;
    let mut count = 1;

    while count < END {
        println!("fib {count} is {}", find_fibonachi(count));
        count += 1;
    }
}
