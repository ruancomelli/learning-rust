pub fn is_armstrong_number(num: u32) -> bool {
    let digits = digits(num);
    let exponent = digits.len() as u32;

    if exponent >= 10 {
        return false;
    }

    num == digits.iter().map(|digit| digit.pow(exponent)).sum()
}

fn digits(num: u32) -> Vec<u32> {
    if num < 10 {
        vec![num]
    } else {
        [vec![num % 10], digits(num / 10)].concat()
    }
}
