
#[derive(Debug)]
enum Error {
    Overflow,
    Zero
}

fn double(n: u32) -> Result<u32, Error> {
    if n == 0 {
        return Err(Error::Zero)
    }
    if n > (u32::MAX / 2) {
        return Err(Error::Overflow)
    }
    Ok(n)
}

fn main() {
    let array = [1, u32::MAX, 0];

    for n in array.iter() {
        if let Ok(x) = double(*n - 2) {
            println!("result with n {}, x {}", n, x);
        }
    }

}