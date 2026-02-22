fn main() {
    let n = 5;
    let result = concat(n);
    println!("Concatenation of numbers from {n} down to 0 is {result}");
}

/// Faulty
#[inline(never)]
fn concat(mut n: usize) -> String {
    let mut result = String::new();
    while n > 0 {
        n -= 1;
        result += n.to_string().as_str();
    }
    result
}
