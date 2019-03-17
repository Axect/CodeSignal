fn main() {
    println!("{}", evenDigitsOnly(1000000000));
}

fn evenDigitsOnly(n: i32) -> bool {
    let exp = (n as f64).log(10f64) as u32;
    let q = n / 10i32.pow(exp);
    let reduced_n = n - (q * 10i32.pow(exp));

    if reduced_n <= 0 {
        q % 2 == 0
    } else {
        if q % 2 == 0 {
            evenDigitsOnly(reduced_n)
        } else {
            false
        }
    }
}
