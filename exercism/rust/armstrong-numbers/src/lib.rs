pub fn is_armstrong_number(num: u32) -> bool {
    let digits = Digits::new(num);
    let exponent = digits.num_digits;

    if exponent >= 10 {
        // skip the calculation if the exponent is too large
        return false;
    }

    num == Digits::new(num).map(|digit| digit.pow(exponent)).sum()
}

struct Digits {
    num: u32,
    num_digits: u32,
}

impl Digits {
    fn new(num: u32) -> Self {
        Digits {
            num,
            num_digits: number_of_digits(num),
        }
    }
}

impl Iterator for Digits {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.num_digits == 0 {
            None
        } else {
            let digit = self.num % 10;
            self.num /= 10;
            self.num_digits -= 1;
            Some(digit)
        }
    }
}

fn number_of_digits(num: u32) -> u32 {
    if num < 10 {
        1
    } else {
        1 + number_of_digits(num / 10)
    }
}
