const PIN: usize = 1235;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_line(&mut buf)?;
    buf = buf.trim().to_string();

    let pin = buf.parse::<usize>()?;

    if check_password(pin, PIN) {
        println!("Access granted!");
    } else {
        println!("Access denied!");
    }

    Ok(())
}

#[inline(never)]
fn check_password(lhs: usize, rhs: usize) -> bool {
    lhs == rhs
}
