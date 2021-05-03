fn main() {
    for i in 1..10 {
        print_only_if_even_number(i)
    }
}

fn print_only_if_even_number(n:u32) -> () {
    match maybe_number(n) {
        Some(n) => println!("even number: {}", n),
        None => println!("odd number!")
    }
}

fn maybe_number(n: u32) -> Option<u32> {
    if n % 2 != 0 { return None }
    Some(n)
}
