pub fn is_armstrong_number(num: u32) -> bool {
    num == armstrong_sum(num)
}

fn armstrong_sum(num: u32) -> u32 {
    let digits = digits(num);
    let exponent = digits.len() as u32;
    digits.iter().map(|digit| digit.pow(exponent)).sum()
}

fn digits(num: u32) -> Vec<u32> {
    if num < 10 {
        vec![num]
    }
    else {
        [vec![num % 10], digits(num / 10)].concat()
    }
}
